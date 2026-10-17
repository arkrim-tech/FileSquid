//! Cucumber harness for the F-050 `fsquid` CLI Gherkin under `tests/features/`.
//!
//! Each run clears `tests/allure-results/` and writes
//! `tests/allure-results/cucumber-junit.xml` with one `<testcase>` per scenario.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use cucumber::{then, when, writer, StatsWriter as _, World, WriterExt as _};
use filesquid_cli::{resolve, run, Route};

/// World state for F-050 CLI scenarios.
#[derive(Debug, Default, World)]
struct CliWorld {
    /// Arguments of the last `fsquid` invocation.
    argv: Vec<String>,
    /// Outcome of the last `fsquid` invocation.
    outcome: Option<Result<i32, String>>,
    /// CLI binary names declared by workspace crates (hosts excluded).
    binaries: Vec<String>,
}

#[when(expr = "I run fsquid with {string}")]
fn run_fsquid(world: &mut CliWorld, line: String) {
    world.argv = line.split_whitespace().map(str::to_string).collect();
    world.outcome = Some(run(&world.argv));
}

#[then("the metadata module should handle it")]
fn metadata_handles(world: &mut CliWorld) {
    match resolve(&world.argv) {
        Route::Run { module, .. } => assert_eq!(module.name, "metadata"),
        other => panic!("expected the metadata module, got {other:?}"),
    }
}

#[then(expr = "fsquid should refuse and suggest {string}")]
fn refuses_with_hint(world: &mut CliWorld, hint: String) {
    let err = world.outcome.clone().expect("ran").expect_err("refused");
    assert!(err.contains(&hint), "{err}");
}

#[then("the command should succeed")]
fn succeeds(world: &mut CliWorld) {
    assert_eq!(world.outcome.clone().expect("ran"), Ok(0));
}

#[then("fsquid should report an unknown command")]
fn unknown(world: &mut CliWorld) {
    let err = world.outcome.clone().expect("ran").expect_err("unknown");
    assert!(err.contains("unknown command"), "{err}");
}

#[when("I list the CLI binaries the workspace builds")]
fn list_binaries(world: &mut CliWorld) {
    let crates = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let hosts = ["filesquid-desktop", "filesquid-server"];
    for entry in fs::read_dir(crates).expect("crates dir") {
        let manifest = entry.expect("entry").path().join("Cargo.toml");
        let Ok(text) = fs::read_to_string(&manifest) else {
            continue;
        };
        let names = bin_names(&text)
            .into_iter()
            .filter(|n| !hosts.contains(&n.as_str()));
        world.binaries.extend(names);
    }
}

#[then("the only CLI binary should be fsquid")]
fn only_fsquid(world: &mut CliWorld) {
    assert_eq!(world.binaries, vec!["fsquid".to_string()]);
}

/// Return the `name` of every `[[bin]]` table in a Cargo manifest.
fn bin_names(manifest: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_bin = false;
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            in_bin = line == "[[bin]]";
            continue;
        }
        if let Some(value) = line.strip_prefix("name = ").filter(|_| in_bin) {
            names.push(value.trim_matches('"').to_string());
        }
    }
    names
}

/// Remove `dir` if it exists; any other error fails the run.
fn remove_dir_if_present(dir: &Path) {
    match fs::remove_dir_all(dir) {
        Ok(()) => {}
        Err(err) if err.kind() == ErrorKind::NotFound => {}
        Err(err) => panic!("cannot clear {}: {err}", dir.display()),
    }
}

#[tokio::main]
async fn main() {
    let results_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/allure-results");
    remove_dir_if_present(&results_dir);
    fs::create_dir_all(&results_dir).expect("allure-results");
    let features = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/features");
    let junit = fs::File::create(results_dir.join("cucumber-junit.xml")).expect("junit file");
    let writer = CliWorld::cucumber()
        .with_writer(
            writer::Basic::stdout()
                .summarized()
                .tee::<CliWorld, _>(writer::JUnit::for_tee(junit, 0))
                .normalized(),
        )
        .run(features)
        .await;
    if writer.execution_has_failed() {
        std::process::exit(1);
    }
}
