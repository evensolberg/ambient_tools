mod cli;
mod creds;
mod detail;
mod device;
mod query;
mod weather;

use chrono::Offset;
use shared::config;

use crate::detail::DetailLevel::{self, *};
use crate::device::get_device_info;
use crate::query::QueryType;
use crate::weather::download;

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
            if detail_level > Quiet {
                log::info!("Getting device information.");
            }

            check_output_folder(&config.output_folder)?;

            let full_path = format!("{}/{dev_info_file_name}", config.output_folder);
            let bytes_written = get_device_info(&creds, &full_path, detail_level).await?;
            if detail_level > Quiet {
                log::info!("Wrote {bytes_written} bytes to {full_path}.");
            }
        }

        QueryType::GetWeather => {
            if detail_level > Quiet {
                log::info!("Getting weather information.");
            }

            check_output_folder(&config.output_folder)?;

            // Get the ArgMatches for the subcommand
            let subcmd_args = cli_args.subcommand_matches("weather").unwrap();

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
                if detail_level > Quiet {
                    log::info!("No dates provided. Getting yesterday's weather information.");
                }

                let info = download::download_yesterdays_weather(&creds).await?;
                let filename = download::get_yesterdays_weather_filename()?;
                let full_path = format!("{}/{filename}", config.output_folder);
                let bytes_written = download::write_weather_info_to_file(&full_path, &info)?;

                if detail_level > Quiet {
                    log::info!("Wrote {bytes_written} bytes to {full_path}.");
                }
            }
        }

        QueryType::GetTimezone => {
            if detail_level > Quiet {
                log::info!("Getting timezone information.\n");
            }
            print_timezone();
        }

        QueryType::Help => {
            log::debug!("Help requested.");
            cli_cmd.print_help()?;
        }
    }

    // Everything is a-okay in the end
    Ok(())
} // fn run()

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
#[tokio::main]
async fn main() {
    std::process::exit(match run().await {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
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
        std::fs::create_dir_all(&output_folder)?;
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
