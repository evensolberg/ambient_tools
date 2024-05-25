mod cli;
mod creds;
mod detail;
mod device;
mod query;
mod weather;

use chrono::Offset;
use shared::config;

use crate::detail::DetailLevel::{self, Quiet};
use crate::query::QueryType;

use std::error::Error;

// Logging
use env_logger::{Builder, Target};
use log::LevelFilter;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// This is where the magic happens.
fn run() -> Result<(), Box<dyn Error>> {
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
            device::get_device_info(&cli_args, &config, &creds, detail_level)?;
        }

        QueryType::GetWeather => {
            weather::get_weather_data(&cli_args, &config, &creds, detail_level)?;
        }

        QueryType::GetTimezone => {
            print_timezone(detail_level);
        }

        QueryType::NewConfig => {
            create_new_config_file(&cli_args, detail_level);
        }

        QueryType::Help => {
            log::debug!("Help requested.");
            cli_cmd.print_help()?;
        }
    }

    // Everything is a-okay in the end
    Ok(())
}

/// Create a new configuration file from the CLI arguments. The default file name is `ambient_download.toml`.
/// If the file already exists, it is overwritten. The function will attempt to populate the timezone offset in the file.
/// If the timezone offset cannot be determined, the value is set to `+00:00`.
/// The function will output an error message if the file cannot be created.
///
/// # Arguments
///
/// * `cli_args` - The command line arguments.
///
/// # Panics
///
/// If the `newconfig` subcommand is not found.
fn create_new_config_file(cli_args: &clap::ArgMatches, detail_level: DetailLevel) {
    if detail_level > Quiet {
        log::info!("Creating new configuration file.");
    }

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

//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// The actual executable function that gets called when the program in invoked.
fn main() {
    std::process::exit(match run() {
        Ok(()) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            log::error!("{}", err.to_string().replace('"', ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}

/// Gets the name of the output folder from the CLI arguments.
/// Checks if the folder exists, and if it doesn't atttempt to create it.
fn check_or_create_output_folder(output_folder: &str) -> Result<(), std::io::Error> {
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
fn print_timezone(detail_level: DetailLevel) {
    if detail_level > Quiet {
        log::info!("Getting timezone information.\n");
    }

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
