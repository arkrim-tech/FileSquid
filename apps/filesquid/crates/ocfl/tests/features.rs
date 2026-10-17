//! Cucumber harness for filesquid-ocfl Gherkin under `tests/features/`.

use std::fs;
use std::path::PathBuf;

use cucumber::{then, when, writer, StatsWriter as _, World, WriterExt as _};

#[derive(Debug, Default, World)]
struct OcflWorld {
    cargo: String,
    lib: String,
    desktop_cargo: String,
    desktop_src: String,
}

fn read_rel(parts: &[&str]) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for part in parts {
        path.push(part);
    }
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()))
}

fn load_identity(world: &mut OcflWorld) {
    world.cargo = read_rel(&["Cargo.toml"]);
    world.lib = read_rel(&["src", "lib.rs"]);
    world.desktop_cargo = read_rel(&["..", "desktop", "Cargo.toml"]);
    world.desktop_src = read_rel(&["..", "desktop", "src", "lib.rs"])
        + &read_rel(&["..", "desktop", "src", "main.rs"]);
}

#[when("I inspect the filesquid-ocfl public API")]
fn inspect_api(world: &mut OcflWorld) {
    load_identity(world);
}

#[then("FileSquid catalog, recipe, C2PA, and collection-snapshot types should be absent")]
fn no_application_types(world: &mut OcflWorld) {
    for needle in ["CollectionSnapshot", "C2paImportRecord", "RecipeOperation"] {
        assert!(
            !world.lib.contains(needle),
            "{needle} must be absent from the engine crate"
        );
    }
}

#[when("I inspect filesquid-ocfl library dependencies")]
fn inspect_deps(world: &mut OcflWorld) {
    load_identity(world);
}

#[then("Arrow, Parquet, DuckDB, and Iceberg should be absent from default features")]
fn no_analytics(world: &mut OcflWorld) {
    let cargo = world.cargo.to_ascii_lowercase();
    for needle in ["arrow", "parquet", "duckdb", "iceberg"] {
        assert!(!cargo.contains(needle), "{needle} must be absent");
    }
}

#[when("I inspect filesquid-ocfl product identity")]
fn inspect_identity(world: &mut OcflWorld) {
    load_identity(world);
}

#[then("it should remain installable without the FileSquid app")]
fn installable(world: &mut OcflWorld) {
    assert!(world
        .cargo
        .contains("Installable without the FileSquid app"));
}

#[then("it should build and publish without other FileSquid crates")]
fn standalone_crate(world: &mut OcflWorld) {
    assert!(
        !world.cargo.contains("path = \"../"),
        "filesquid-ocfl MUST NOT path-depend on another FileSquid crate"
    );
}

#[then("community OCFL programs should not be the engine")]
fn no_community_engine(world: &mut OcflWorld) {
    let cargo = world.cargo.to_ascii_lowercase();
    for needle in ["rocfl", "ocfl-py", "ocfl-java", "ocfl-tools"] {
        assert!(
            !cargo.contains(needle),
            "{needle} must not be a crate dependency"
        );
    }
}

#[when("I inspect the desktop crate")]
fn inspect_desktop(world: &mut OcflWorld) {
    load_identity(world);
}

#[then("desktop Cargo should depend on filesquid-ocfl")]
fn desktop_depends(world: &mut OcflWorld) {
    assert!(world.desktop_cargo.contains("filesquid-ocfl"));
}

#[then("FileSquid should not shell out to fsquid for canonical writes")]
fn no_cli_shell(world: &mut OcflWorld) {
    assert!(!world.desktop_src.contains("fsquid"));
}

fn write_junit_report(path: &PathBuf, passed: usize, failed: usize, skipped: usize) {
    let tests = passed + failed + skipped;
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="filesquid_ocfl.features" tests="{tests}" failures="{failed}" skipped="{skipped}" />
"#
    );
    fs::write(path, xml).expect("write junit");
}

#[tokio::main]
async fn main() {
    let results_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/allure-results");
    let _ = fs::remove_dir_all(&results_dir);
    fs::create_dir_all(&results_dir).expect("allure-results");
    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");
    let writer = OcflWorld::cucumber()
        .with_writer(writer::Basic::stdout().summarized())
        .run(features)
        .await;
    write_junit_report(
        &results_dir.join("cucumber-junit.xml"),
        writer.passed_steps(),
        writer.failed_steps(),
        writer.skipped_steps(),
    );
    if writer.execution_has_failed() {
        std::process::exit(1);
    }
}
