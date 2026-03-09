//! `ambient_process` — process and convert Ambient Weather JSON data files.

mod cli;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use shared::{
    datapoint::units::SystemOfUnits,
    pipeline::{
        export::csv::{write_csv, ALL_FIELDS},
        filter::DateFilter,
        reader::read_glob,
    },
};

fn main() -> Result<()> {
    let matches = cli::build_cli().get_matches();

    // Initialise logger
    let detail: u8 = *matches.get_one("detail").unwrap_or(&1u8);
    let level = match detail {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    env_logger::builder().filter_level(level).init();

    match matches.subcommand() {
        Some(("convert", args)) => cmd_convert(args),
        Some(("prettify", args)) => cmd_prettify(args),
        _ => {
            cli::build_cli().print_long_help()?;
            Ok(())
        }
    }
}

fn cmd_convert(args: &clap::ArgMatches) -> Result<()> {
    // Collect all input patterns
    let patterns: Vec<&String> = args.get_many("files").unwrap_or_default().collect();

    // Read and merge all matching files
    let mut records = Vec::new();
    for pattern in &patterns {
        let mut batch = read_glob(pattern)
            .with_context(|| format!("Failed to read files matching: {pattern}"))?;
        records.append(&mut batch);
    }
    records.sort_by_key(|r| r.dateutc);
    log::info!("Loaded {} records from {} pattern(s).", records.len(), patterns.len());

    // Date filter
    let from = args
        .get_one::<String>("from")
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .context("--from date must be YYYY-MM-DD")?;
    let to = args
        .get_one::<String>("to")
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .context("--to date must be YYYY-MM-DD")?;
    let filter = DateFilter::new(from, to);
    let records = filter.apply(records);
    log::info!("{} records after date filter.", records.len());

    // Unit system
    let units = match args.get_one::<String>("units").map(String::as_str) {
        Some("si") => SystemOfUnits::SI,
        _ => SystemOfUnits::Imperial,
    };

    // Field list
    let fields_override: Option<Vec<&str>> = args.get_one::<String>("fields").map(|s| {
        if s == "all" {
            ALL_FIELDS.to_vec()
        } else {
            s.split(',').map(str::trim).collect()
        }
    });
    let fields: Option<&[&str]> = fields_override.as_deref();

    // Write CSV
    if let Some(path) = args.get_one::<String>("output") {
        let file = std::fs::File::create(path)
            .with_context(|| format!("Failed to create output file: {path}"))?;
        write_csv(file, &records, fields, units)?;
        log::info!("Wrote CSV to {path}.");
    } else {
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        write_csv(handle, &records, fields, units)?;
    }

    Ok(())
}

fn cmd_prettify(args: &clap::ArgMatches) -> Result<()> {
    let patterns: Vec<&String> = args.get_many("files").unwrap_or_default().collect();

    for pattern in &patterns {
        let paths: Vec<_> = glob::glob(pattern)
            .with_context(|| format!("Invalid glob: {pattern}"))?
            .filter_map(|e| {
                e.map_err(|err| {
                    log::warn!("Glob error: {err}");
                })
                .ok()
            })
            .collect();

        for path in paths {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let value: serde_json::Value = serde_json::from_str(&content)
                .with_context(|| format!("Invalid JSON in {}", path.display()))?;
            let pretty = serde_json::to_string_pretty(&value)?;
            std::fs::write(&path, pretty.as_bytes())
                .with_context(|| format!("Failed to write {}", path.display()))?;
            log::info!("Prettified {}", path.display());
        }
    }

    Ok(())
}
