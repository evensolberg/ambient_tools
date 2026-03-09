//! `ambient_process` — process and convert Ambient Weather JSON data files.

mod cli;

use anyhow::{Context, Result};
use chrono::{NaiveDate, TimeZone};
use shared::{
    datapoint::units::SystemOfUnits,
    pipeline::{
        export::{
            csv::{write_csv, ALL_FIELDS},
            toon::write_toon,
        },
        filename::format_output_filename,
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
        Some(("reorganize", args)) => cmd_reorganize(args),
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
    log::info!(
        "Loaded {} records from {} pattern(s).",
        records.len(),
        patterns.len()
    );

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

    // Output format
    let format = args
        .get_one::<String>("format")
        .map_or("csv", String::as_str);

    // Write output
    if let Some(path) = args.get_one::<String>("output") {
        let file = std::fs::File::create(path)
            .with_context(|| format!("Failed to create output file: {path}"))?;
        match format {
            "toon" => write_toon(file, &records, fields, units)?,
            _ => write_csv(file, &records, fields, units)?,
        }
        log::info!("Wrote {format} to {path}.");
    } else {
        let stdout = std::io::stdout();
        let handle = stdout.lock();
        match format {
            "toon" => write_toon(handle, &records, fields, units)?,
            _ => write_csv(handle, &records, fields, units)?,
        }
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

fn cmd_reorganize(args: &clap::ArgMatches) -> Result<()> {
    // Load config file if provided; CLI flags override its values.
    let config = args
        .get_one::<String>("config")
        .map(|path| {
            shared::config::Config::from_file(path)
                .with_context(|| format!("Failed to read config file: {path}"))
        })
        .transpose()?;

    let default_pattern = String::from("%Y-%m-%d.json");
    let default_output_dir = String::from(".");

    let pattern = if args.value_source("pattern") != Some(clap::parser::ValueSource::DefaultValue)
    {
        args.get_one::<String>("pattern")
            .unwrap_or(&default_pattern)
            .clone()
    } else if let Some(ref cfg) = config {
        cfg.filename_pattern.clone()
    } else {
        default_pattern
    };

    let output_dir = if args.value_source("output-dir")
        != Some(clap::parser::ValueSource::DefaultValue)
    {
        args.get_one::<String>("output-dir")
            .unwrap_or(&default_output_dir)
            .clone()
    } else if let Some(ref cfg) = config {
        cfg.output_folder.clone()
    } else {
        default_output_dir
    };

    let mac = args
        .get_one::<String>("mac")
        .cloned()
        .or_else(|| config.as_ref().map(|c| c.mac_address.clone()))
        .unwrap_or_default();

    let station = args
        .get_one::<String>("station")
        .cloned()
        .or_else(|| config.as_ref().map(|c| c.station_name.clone()))
        .unwrap_or_default();

    let dry_run = args.get_flag("dry-run");

    let patterns: Vec<&String> = args.get_many("files").unwrap_or_default().collect();

    let mut moved: u32 = 0;
    let mut skipped: u32 = 0;
    let mut errors: u32 = 0;

    for glob_pattern in &patterns {
        let paths: Vec<_> = glob::glob(glob_pattern)
            .with_context(|| format!("Invalid glob: {glob_pattern}"))?
            .filter_map(|e| {
                e.map_err(|err| log::warn!("Glob error: {err}"))
                    .ok()
            })
            .collect();

        for path in paths {
            match reorganize_file(&path, &pattern, &output_dir, &mac, &station, dry_run) {
                Ok(ReorganizeOutcome::Moved | ReorganizeOutcome::DryRun) => moved += 1,
                Ok(ReorganizeOutcome::Skipped) => skipped += 1,
                Err(e) => {
                    log::warn!("Error processing {}: {e}", path.display());
                    errors += 1;
                }
            }
        }
    }

    if dry_run {
        log::info!("Dry run: {moved} file(s) would be moved, {skipped} skipped, {errors} error(s).");
    } else {
        log::info!("Done: {moved} file(s) moved, {skipped} skipped, {errors} error(s).");
    }

    Ok(())
}

enum ReorganizeOutcome {
    Moved,
    Skipped,
    DryRun,
}

fn reorganize_file(
    source: &std::path::Path,
    pattern: &str,
    output_dir: &str,
    mac: &str,
    station: &str,
    dry_run: bool,
) -> Result<ReorganizeOutcome> {
    // Read JSON and extract the date from the first record.
    let content = std::fs::read_to_string(source)
        .with_context(|| format!("Failed to read {}", source.display()))?;
    let records: Vec<serde_json::Value> = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON in {}", source.display()))?;
    let first = records
        .first()
        .with_context(|| format!("No records in {}", source.display()))?;
    let dateutc = first
        .get("dateutc")
        .and_then(serde_json::Value::as_u64)
        .with_context(|| format!("No dateutc in first record of {}", source.display()))?;

    // Convert milliseconds-since-epoch to DateTime<Local>.
    let secs = (dateutc / 1000).cast_signed();
    let nanos = u32::try_from((dateutc % 1000) * 1_000_000)
        .with_context(|| format!("Nanosecond overflow for timestamp {dateutc}"))?;
    let dt = chrono::Local
        .timestamp_opt(secs, nanos)
        .single()
        .with_context(|| format!("Invalid timestamp {dateutc} in {}", source.display()))?;

    let rel_path = format_output_filename(pattern, &dt, mac, station);
    let target = std::path::PathBuf::from(output_dir).join(&rel_path);

    if target.exists() {
        log::warn!(
            "Skipping {} -> {}: target already exists.",
            source.display(),
            target.display()
        );
        return Ok(ReorganizeOutcome::Skipped);
    }

    if dry_run {
        println!("{} -> {}", source.display(), target.display());
        return Ok(ReorganizeOutcome::DryRun);
    }

    // Create parent directories as needed.
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    std::fs::rename(source, &target).with_context(|| {
        format!(
            "Failed to move {} -> {}",
            source.display(),
            target.display()
        )
    })?;
    log::info!("{} -> {}", source.display(), target.display());

    Ok(ReorganizeOutcome::Moved)
}
