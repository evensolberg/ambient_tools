/// The commands issued from the CLI
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// Get the device information.
    GetDeviceInfo,

    /// Get the weather information.
    GetWeather,

    /// Output the local time zone information
    GetTimezone,

    /// Print the help message.
    #[default]
    Help,
}

impl QueryType {
    /// Create a new `Command` struct from the command line arguments.
    pub fn from_cli(cli_args: &clap::ArgMatches) -> Self {
        match cli_args.subcommand_name() {
            Some("device") => Self::GetDeviceInfo,
            Some("weather") => Self::GetWeather,
            Some("timezone") => Self::GetTimezone,
            _ => Self::Help,
        }
    }
}
