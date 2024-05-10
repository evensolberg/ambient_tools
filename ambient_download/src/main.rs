mod cli;
mod creds;
mod detail;
mod device;
mod query;
mod weather;

use crate::detail::DetailLevel;
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

    let detail_level = DetailLevel::from_count(*cli_args.get_one::<u8>("detail").unwrap_or(&1));
    match detail_level {
        DetailLevel::Quiet => {
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
    }

    // Initialize logging
    logbuilder.target(Target::Stdout).init();

    let output_folder = get_output_folder(&cli_args)?;

    // Output the command line arguments
    log::debug!("Command line arguments: {cli_args:?}");

    let creds = creds::Query::from_cli(&cli_args);
    log::debug!("{creds:?}");

    match query::QueryType::from_cli(&cli_args) {
        QueryType::GetDeviceInfo => {
            let info_file_default = String::from("device-info.json");
            let dev_info_file_name = cli_args
                .get_one::<String>("device-info-filename")
                .unwrap_or(&info_file_default);
            if detail_level > DetailLevel::Quiet {
                log::info!("Getting device information.");
            }

            let full_path = format!("{output_folder}/{dev_info_file_name}");
            let bytes_written =
                crate::device::get_device_info(&creds, &full_path, detail_level).await?;
            if detail_level > DetailLevel::Quiet {
                log::info!("Wrote {bytes_written} bytes to {full_path}.");
            }
        }
        QueryType::GetWeather => {
            if detail_level > DetailLevel::Quiet {
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
                if detail_level > DetailLevel::Quiet {
                    log::info!("No dates provided. Getting yesterday's weather information.");
                }

                let info = download::download_yesterdays_weather(&creds).await?;
                let filename = download::get_yesterdays_weather_filename()?;
                let full_path = format!("{output_folder}/{filename}");
                let bytes_written = download::write_weather_info_to_file(&full_path, &info)?;

                if detail_level > DetailLevel::Quiet {
                    log::info!("Wrote {bytes_written} bytes to {full_path}.");
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

/// Gets the name of the output folder from the CLI arguments.
/// Checks if the folder exists, and if it doesn't atttempt to create it.
fn get_output_folder(cli_args: &clap::ArgMatches) -> Result<String, std::io::Error> {
    let current_folder = String::from(".");
    let output_folder = cli_args
        .get_one::<String>("output-folder")
        .unwrap_or(&current_folder)
        .to_owned();
    if !std::path::Path::new(&output_folder).exists() {
        std::fs::create_dir_all(&output_folder)?;
    }
    Ok(output_folder)
}
