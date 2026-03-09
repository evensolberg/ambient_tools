use clap::{Arg, Command};

#[allow(clippy::too_many_lines)]
pub fn build_cli() -> Command {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .long_about("Process and convert Ambient Weather JSON data files.")
        .arg(
            Arg::new("detail")
                .short('d')
                .long("detail-level")
                .help("Output detail level (0=Quiet 1=Normal 2=Detailed 3=Debug)")
                .action(clap::ArgAction::Set)
                .env("AMBIENT_WEATHER_DETAIL_LEVEL")
                .default_value("1")
                .value_parser(clap::value_parser!(u8).range(..=4)),
        )
        .subcommand(
            Command::new("convert")
                .about("Convert JSON weather files to CSV.")
                .long_about(
                    "Read one or more JSON weather data files (glob patterns accepted) \
                     and write a combined CSV.",
                )
                .arg(
                    Arg::new("files")
                        .value_name("FILES")
                        .help("Input JSON files or glob patterns (e.g. 'data/*.json').")
                        .required(true)
                        .num_args(1..)
                        .action(clap::ArgAction::Append),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output CSV file. Defaults to stdout if not provided.")
                        .action(clap::ArgAction::Set),
                )
                .arg(
                    Arg::new("from")
                        .long("from")
                        .help("Filter records on or after this date (YYYY-MM-DD).")
                        .action(clap::ArgAction::Set),
                )
                .arg(
                    Arg::new("to")
                        .long("to")
                        .help("Filter records on or before this date (YYYY-MM-DD).")
                        .action(clap::ArgAction::Set),
                )
                .arg(
                    Arg::new("units")
                        .short('u')
                        .long("units")
                        .help("Unit system: 'si' or 'imperial' [default: imperial].")
                        .action(clap::ArgAction::Set)
                        .value_parser(["si", "imperial"])
                        .default_value("imperial"),
                )
                .arg(
                    Arg::new("fields")
                        .short('f')
                        .long("fields")
                        .help(
                            "Comma-separated field list, or 'all' for every field. \
                             Default fields: date, temp_out, humidity_out, wind_speed, \
                             wind_gust, wind_dir, baro_rel, solar_radiation, uv, \
                             hourly_rain, daily_rain, temp_in, humidity_in.",
                        )
                        .action(clap::ArgAction::Set),
                ),
        )
        .subcommand(
            Command::new("prettify")
                .about("Pretty-print JSON weather files in place.")
                .long_about(
                    "Read one or more JSON weather data files and re-write them with \
                     indented (pretty-printed) formatting.",
                )
                .arg(
                    Arg::new("files")
                        .value_name("FILES")
                        .help("Input JSON files or glob patterns.")
                        .required(true)
                        .num_args(1..)
                        .action(clap::ArgAction::Append),
                ),
        )
}
