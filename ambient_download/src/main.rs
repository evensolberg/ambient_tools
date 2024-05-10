mod cli;
mod creds;
mod device;
mod query;
mod weather;

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

    // create a log builder
    let mut logbuilder = Builder::new();

    // Figure out what log level to use.
    if cli_args.get_flag("quiet") {
        logbuilder.filter_level(LevelFilter::Off);
    } else {
        match cli_args.get_count("debug") {
            0 => logbuilder.filter_level(LevelFilter::Info),
            1 => logbuilder.filter_level(LevelFilter::Debug),
            _ => logbuilder.filter_level(LevelFilter::Trace),
        };
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    // Output the command line arguments
    log::debug!("Command line arguments: {cli_args:?}");

    let print_summary = cli_args.get_flag("print-summary");
    let mut quiet = cli_args.get_flag("quiet");
    let mut show_detail = cli_args.get_flag("show-detail");

    let creds = creds::Query::from_cli(&cli_args);
    log::debug!("{creds:?}");

    let query_type = query::QueryType::from_cli(&cli_args);
    log::debug!("{query_type:?}");

    match query_type {
        QueryType::GetDeviceInfo => {
            if print_summary {
                show_detail = false;
                quiet = true;
            }
            let info_file_default = String::from("device-info.json");
            let dev_info_file = cli_args
                .get_one::<String>("device-info-filename")
                .unwrap_or(&info_file_default);
            if show_detail && !quiet {
                log::info!("Getting device information.");
            }
            let bytes_written =
                crate::device::get_device_info(&creds, dev_info_file, print_summary).await?;
            if show_detail && !quiet {
                log::info!("Wrote {bytes_written} bytes to {dev_info_file}.");
            }
        }
        QueryType::GetWeather => {
            if show_detail && !quiet {
                log::info!("Getting weather information.");
            }

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
                if show_detail && !quiet {
                    log::info!("No dates provided. Getting yesterday's weather information.");
                }

                let info = download::download_yesterdays_weather(&creds).await?;
                let filename = download::get_yesterdays_weather_filename()?;
                let bytes_written = download::write_weather_info_to_file(&filename, &info)?;

                if show_detail && !quiet {
                    log::info!("Wrote {bytes_written} bytes to {filename}.");
                }
            }
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
