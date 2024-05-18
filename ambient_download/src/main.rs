mod cli;
mod creds;
mod detail;
mod device;
mod query;
mod weather;

use chrono::Offset;
use shared::config;

use crate::detail::DetailLevel::{self, Quiet};
use crate::device::get_device_info;
use crate::query::QueryType;
use crate::weather::download::{self, yesterday};

use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
async fn run() -> Result<(), Box<dyn Error>> {
    // Set up the command line. Ref https://docs.rs/clap for details.
    let mut cli_cmd = cli::build_cli();
    let cli_args = cli_cmd.clone().get_matches();

    // Output the command line arguments
    log::debug!("Command line arguments: {cli_args:?}");

    // Check if the config file has been provided and create a config from it. Otherwise, create it from the CLI arguments.
    let config = if let Some(config_file) = cli_args.get_one::<String>("config-file") {
        config::Config::from_file(config_file)?
    } else {
        config::Config::from_args(&cli_args)
    };

    let detail_level = DetailLevel::from_number(config.detail_level);

    // Set up logging
    let mut logbuilder = Builder::new();
    set_log_level(detail_level, &mut logbuilder);

    let mut creds = creds::Query::from_config(&config);
    creds.query_type = query::QueryType::from_cli(&cli_args);
    log::debug!("{creds:?}");

    match creds.query_type {
        QueryType::GetDeviceInfo => {
            let info_file_default = String::from("device-info.json");
            let dev_info_file_name = cli_args
                .get_one::<String>("device-info-filename")
                .unwrap_or(&info_file_default);

            download_device_info(
                dev_info_file_name,
                detail_level,
                &config.output_folder,
                &creds,
            )
            .await?;
        }

        QueryType::GetWeather => {
            if detail_level > Quiet {
                log::info!("Getting weather information.");
            }

            check_output_folder(&config.output_folder)?;

            // Get the ArgMatches for the subcommand
            let subcmd_args = cli_args
                .subcommand_matches("weather")
                .expect("Weather subcommand not found. Yikes!");

            // If no date has been provided, get yesterday's weather information.
            let mut dates = Vec::<&str>::new();
            for date in subcmd_args
                .get_many::<String>("end-dates")
                .unwrap_or_default()
            {
                dates.push(date);
            }
            log::debug!("dates = {dates:?}.");

            if dates.is_empty() {
                download_yesterdays_weather(detail_level, creds, &config.output_folder).await?;
            }
        }

        QueryType::GetTimezone => {
            if detail_level > Quiet {
                log::info!("Getting timezone information.\n");
            }
            print_timezone();
        }

        QueryType::NewConfig => {
            let subcmd_args = cli_args
                .subcommand_matches("newconfig")
                .expect("newconfig subcommand not found.");

            let config_file = subcmd_args
                .get_one::<String>("config-file")
                .map_or("ambient_download.toml", |file| file);

            if matches!(config::Config::new_config_file(config_file), Ok(())) {
                log::info!("Created new configuration file {config_file}.");
            } else {
                log::error!("Unable to create new configuration file {config_file}.");
            }
        }

        QueryType::Help => {
            log::debug!("Help requested.");
            cli_cmd.print_help()?;
        }
    }

    // Everything is a-okay in the end
    Ok(())
}

/// Download the device information and write it to a file.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
/// * `detail_level` - The level of detail to log.
/// * `config` - The configuration.
///
/// # Returns
///
/// A `Result` with the number of bytes written to the file.
///
/// # Errors
///
/// If the output folder cannot be created or the device information cannot be downloaded.
async fn download_device_info(
    dev_info_file_name: &str,
    detail_level: DetailLevel,
    output_folder: &str,
    creds: &creds::Query,
) -> Result<(), Box<dyn Error>> {
    if detail_level > Quiet {
        log::info!("Getting device information.");
    }

    // Check if the output folder exists and create it if it doesn't.
    check_output_folder(output_folder)?;

    let full_path = format!("{output_folder}/{dev_info_file_name}");
    let bytes_written = get_device_info(creds, &full_path, detail_level).await?;

    if detail_level > Quiet {
        log::info!("Wrote {bytes_written} bytes to {full_path}.");
    };
    Ok(())
}

/// Download yesterday's weather information and write it to a file.
///
/// # Arguments
///
/// * `detail_level` - The level of detail to log.
/// * `creds` - The credentials to use.
/// * `output_folder` - The folder to write the output file to.
///
/// # Returns
///
/// A `Result` with the number of bytes written to the file.
///
/// # Errors
///
/// If the output folder cannot be created, the weather information cannot be downloaded, or the file cannot be written.
async fn download_yesterdays_weather(
    detail_level: DetailLevel,
    creds: creds::Query,
    output_folder: &str,
) -> Result<(), Box<dyn Error>> {
    if detail_level > Quiet {
        log::info!("No dates provided. Getting yesterday's weather information.");
    }

    let yesterday_eod = download::end_of_day(&yesterday())?;
    let weather_data = download::download_weather(&yesterday_eod, &creds).await?;

    let output_file_name = download::filename_from_datetime(&yesterday_eod, "json");
    let full_path = format!("{output_folder}/{output_file_name}");
    let bytes_written = download::write_weather_info_to_file(&full_path, &weather_data)?;

    if detail_level > Quiet {
        log::info!("Wrote {bytes_written} bytes to {full_path}.");
    };
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(()) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}

/// Gets the name of the output folder from the CLI arguments.
/// Checks if the folder exists, and if it doesn't atttempt to create it.
fn check_output_folder(output_folder: &str) -> Result<(), std::io::Error> {
    if !std::path::Path::new(&output_folder).exists() {
        std::fs::create_dir_all(output_folder)?;
    }
    Ok(())
}

/// Sets the logging detail level based on the CLI arguments.
/// The default is `DetailLevel::Normal`.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
/// * `logbuilder` - The log builder.
fn set_log_level(detail_level: DetailLevel, logbuilder: &mut env_logger::Builder) {
    match detail_level {
        Quiet => {
            logbuilder.filter_level(LevelFilter::Off);
        }
        DetailLevel::Normal | DetailLevel::Detailed => {
            logbuilder.filter_level(LevelFilter::Info);
        }
        DetailLevel::Debug => {
            logbuilder.filter_level(LevelFilter::Debug);
        }
        DetailLevel::Trace => {
            logbuilder.filter_level(LevelFilter::Trace);
        }
    };

    // Initialize logging
    logbuilder.target(Target::Stdout).init();
}

/// Print time zone information.
fn print_timezone() {
    let local_time = chrono::Local::now();
    println!("Local time:            {local_time}");
    println!("UTC time:              {}", chrono::Utc::now());
    println!(
        "Local offset:          {} seconds",
        local_time.offset().fix().local_minus_utc()
    );
    println!("Local timezone offset: {} (HH:MM)", local_time.offset());
    if let Ok(tz_name) = iana_time_zone::get_timezone() {
        println!("Local timezone name:   {tz_name}");
    } else {
        println!("Unable to determine local timezone name.");
    }
}
