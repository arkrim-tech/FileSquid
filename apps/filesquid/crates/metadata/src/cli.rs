//! Stand-alone CLI: extract, get, set, export, import, validate.

use std::path::{Path, PathBuf};

use crate::csv::{apply_csv, to_csv};
use crate::extract::extract_into;
use crate::record::parse_curie;
use crate::reject_file_dcterms_term;
use crate::store::{load, save};
use crate::xmp::{from_xmp, to_xmp};

/// Usage text for the stand-alone UI.
pub const USAGE: &str = "\
filesquid-metadata — stand-alone metadata CRUD (via filesquid-ocfl)

Usage:
  filesquid-metadata <command> [args] [--object <dir>]

Commands:
  extract <file>          Extract EXIF/ID3 from a media file into metadata.md
  get [prefix:term]       Print populated statements (or one CURIE)
  set <prefix:term> <val> Set a statement and write metadata.md
  export csv|xmp <path>   Write CSV or XMP interchange
  import csv|xmp <path>   Read CSV or XMP into metadata.md
  validate <prefix:term>  Check a dcterms: term against the /terms/ register
  help                    Show this help

Canonical store: <object>/metadata.md (YAML front matter + Markdown body).
";

const COMMANDS: &[&str] = &[
    "extract", "get", "set", "export", "import", "validate", "help",
];

/// Run the CLI. Returns a process exit code.
pub fn run(args: &[String]) -> Result<i32, String> {
    if args.is_empty() || is_help(&args[0]) {
        print!("{USAGE}");
        return Ok(0);
    }
    let cmd = args[0].as_str();
    if !COMMANDS.contains(&cmd) {
        return Err(format!("unknown command: {cmd}\n{USAGE}"));
    }
    if cmd == "help" {
        print!("{USAGE}");
        return Ok(0);
    }
    let rest = &args[1..];
    match cmd {
        "validate" => validate_args(rest),
        "extract" => cmd_extract(rest),
        "get" => cmd_get(rest),
        "set" => cmd_set(rest),
        "export" => cmd_export(rest),
        "import" => cmd_import(rest),
        _ => unreachable!(),
    }
}

fn is_help(arg: &str) -> bool {
    arg == "-h" || arg == "--help" || arg == "help"
}

fn object_dir(args: &[String]) -> PathBuf {
    args.windows(2)
        .find(|w| w[0] == "--object")
        .map(|w| PathBuf::from(&w[1]))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn positional(args: &[String]) -> Vec<&str> {
    let mut out = Vec::new();
    let mut skip = 0;
    for (i, a) in args.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if a == "--object" {
            skip = 1;
            continue;
        }
        if i + 1 < args.len() && args[i] == "--object" {
            continue;
        }
        out.push(a.as_str());
    }
    out
}

fn validate_args(rest: &[String]) -> Result<i32, String> {
    let pos = positional(rest);
    let Some(curie) = pos.first() else {
        return Err("validate requires a term such as dcterms:title".into());
    };
    let Some((prefix, term)) = curie.split_once(':') else {
        return Err("validate expects prefix:term".into());
    };
    if prefix != "dcterms" {
        return Err("validate currently checks dcterms: terms".into());
    }
    match reject_file_dcterms_term(term) {
        Ok(()) => {
            println!("{curie} is a valid file dcterms term");
            Ok(0)
        }
        Err(e) => {
            eprintln!("{curie}: {e}");
            Ok(1)
        }
    }
}

fn cmd_extract(rest: &[String]) -> Result<i32, String> {
    let pos = positional(rest);
    let Some(file) = pos.first() else {
        return Err("extract requires a media file path".into());
    };
    let object = object_dir(rest);
    let mut record = load(&object).map_err(|e| e.to_string())?;
    extract_into(Path::new(file), &mut record);
    save(&object, &record).map_err(|e| e.to_string())?;
    println!("extracted into {}/metadata.md", object.display());
    Ok(0)
}

fn cmd_get(rest: &[String]) -> Result<i32, String> {
    let pos = positional(rest);
    let object = object_dir(rest);
    let record = load(&object).map_err(|e| e.to_string())?;
    if let Some(curie) = pos.first() {
        match record.get(curie) {
            Some(s) => {
                println!("{} {}", s.curie(), s.value);
                Ok(0)
            }
            None => {
                eprintln!("{curie} is not populated");
                Ok(1)
            }
        }
    } else {
        for s in &record.statements {
            println!("{} {}", s.curie(), s.value);
        }
        Ok(0)
    }
}

fn cmd_set(rest: &[String]) -> Result<i32, String> {
    let pos = positional(rest);
    let Some(curie) = pos.first() else {
        return Err("set requires prefix:term and a value".into());
    };
    let value = pos.get(1).copied().unwrap_or("");
    let object = object_dir(rest);
    let (ns, term) = parse_curie(curie).map_err(|e| e.to_string())?;
    let mut record = load(&object).map_err(|e| e.to_string())?;
    record.set(ns, term, value).map_err(|e| e.to_string())?;
    save(&object, &record).map_err(|e| e.to_string())?;
    println!("set {curie}");
    Ok(0)
}

fn cmd_export(rest: &[String]) -> Result<i32, String> {
    let pos = positional(rest);
    let fmt = pos.first().copied().unwrap_or("");
    let path = pos.get(1).copied().unwrap_or("");
    if path.is_empty() {
        return Err("export requires csv|xmp and a path".into());
    }
    let object = object_dir(rest);
    let record = load(&object).map_err(|e| e.to_string())?;
    let body = match fmt {
        "csv" => to_csv(&object.display().to_string(), &record),
        "xmp" => to_xmp(&record),
        _ => return Err("export format must be csv or xmp".into()),
    };
    std::fs::write(path, body).map_err(|e| e.to_string())?;
    println!("exported {fmt} to {path}");
    Ok(0)
}

fn cmd_import(rest: &[String]) -> Result<i32, String> {
    let pos = positional(rest);
    let fmt = pos.first().copied().unwrap_or("");
    let path = pos.get(1).copied().unwrap_or("");
    if path.is_empty() {
        return Err("import requires csv|xmp and a path".into());
    }
    let object = object_dir(rest);
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut record = load(&object).map_err(|e| e.to_string())?;
    match fmt {
        "csv" => apply_csv(&text, &mut record).map_err(|e| format!("{e:?}"))?,
        "xmp" => {
            let incoming = from_xmp(&text);
            for s in incoming.statements {
                record
                    .set(s.namespace, &s.term, s.value)
                    .map_err(|e| e.to_string())?;
            }
        }
        _ => return Err("import format must be csv or xmp".into()),
    }
    save(&object, &record).map_err(|e| e.to_string())?;
    println!("imported {fmt} from {path}");
    Ok(0)
}
