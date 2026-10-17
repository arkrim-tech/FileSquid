# Task CLI-0.1 Walkthrough: `fsquid` binary and command routing

## SRS-F-050: FileSquid platform and distributions

Requirement ([SRS-F-050](../../../../../../resources/srs.md#srs-f-050-filesquid-platform-and-distributions), [C-057](../../../../../../resources/product-vision.md#module-rules)): every FileSquid command-line tool is invoked as `fsquid`. The `filesquid-cli` crate builds the only `fsquid` binary with every module's commands; unique commands run at the top level, `fsquid <module> <command>` always works, and `validate` always needs its module. Also covered: [SRS-F-052: Metadata module](../../../../../../resources/srs.md#srs-f-052-metadata-module) (metadata commands now run in `fsquid`; the `filesquid-metadata` binary is retired).

Owner decisions (2026-10-17), recorded in [ADR 036](../../../../../../resources/architecture/036-single-fsquid-cli.md): one CLI crate that always includes every module; subcommand groups (`fsquid c2pa verify`); `validate` always needs its module; documentation and code change in the same branch.

## Changes

- New crate `apps/filesquid/crates/cli` (`filesquid-cli`, bin `fsquid`): pure `resolve` and dispatching `run`, a module table (today `metadata`), `QUALIFIED_ONLY = ["validate"]`.
- `filesquid-metadata`: `[[bin]]` and `src/bin/filesquid-metadata.rs` removed; `cli::COMMANDS` public; usage text names `fsquid`.
- Documentation: C-057 (vision), PRD F-050/F-051/F-052/F-054, SRS-F-050/F-051/F-052/F-054, module SRS rows OCFL-S-006, MD-S-005, C2PA-S-010, ADR 036 with amendment notes on ADRs 016, 023, 024 and 026, module READMEs and plans, global Task FS-0.1, root README.

## Traceability

| Requirement | Tests |
|---|---|
| Unique command at top level | `unit_route::unique_command_runs_at_top_level`; scenario "A command that only one module defines runs without the module name" |
| `fsquid <module> <command>` | `module_group_strips_the_module_name`, `bare_module_name_shows_module_help`; scenario "Any command also runs with its module name" |
| `validate` needs its module | `validate_without_module_needs_the_module`, `validate_with_module_runs`; scenarios "validate always needs the module name", "validate runs with the module name" |
| Collision guard | `shared_command_names_are_qualifier_only`, `module_names_are_not_command_names` |
| Only `fsquid` as CLI binary | `fsquid_is_the_only_cli_binary_in_the_workspace`; scenario "fsquid is the only CLI binary" |
| Unknown command, help | `unknown_command_is_an_error`, `help_forms_resolve_to_help`, `usage_names_fsquid_and_both_forms` |

[ADR 017](../../../../../../resources/architecture/017-test-runners-and-reporting.md) defines no Rust line-coverage tool, so no coverage percentage is reported. Every public item of `src/lib.rs` is exercised by the tests above.

## Red: `cargo test -p filesquid-cli --test unit_route` against a stub

```text
test bare_module_name_shows_module_help ... FAILED
test fsquid_is_the_only_cli_binary_in_the_workspace ... FAILED
test help_forms_resolve_to_help ... FAILED
test module_group_strips_the_module_name ... FAILED
test module_names_are_not_command_names ... ok
test shared_command_names_are_qualifier_only ... ok
test unique_command_runs_at_top_level ... FAILED
test unknown_command_is_an_error ... FAILED
test usage_names_fsquid_and_both_forms ... FAILED
test validate_with_module_runs ... FAILED
test validate_without_module_needs_the_module ... FAILED
test result: FAILED. 2 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.49s
```

## Green: `cargo test -p filesquid-cli --locked`

```text
     Running unittests src/lib.rs (target/debug/deps/filesquid_cli-9a6976f42b67663e)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src/main.rs (target/debug/deps/fsquid-61713270adb8f558)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests/features.rs (target/debug/deps/features-364a2833667d62cd)
Feature: One command-line tool named fsquid
  Scenario: A command that only one module defines runs without the module name
   ✔  When I run fsquid with "get dcterms:title"
   ✔  Then the metadata module should handle it
  Scenario: Any command also runs with its module name
   ✔  When I run fsquid with "metadata get dcterms:title"
   ✔  Then the metadata module should handle it
  Scenario: validate always needs the module name
   ✔  When I run fsquid with "validate dcterms:title"
   ✔  Then fsquid should refuse and suggest "fsquid metadata validate"
  Scenario: validate runs with the module name
   ✔  When I run fsquid with "metadata validate dcterms:title"
   ✔  Then the command should succeed
  Scenario: An unknown command is reported
   ✔  When I run fsquid with "mods"
   ✔  Then fsquid should report an unknown command
  Scenario: fsquid is the only CLI binary
   ✔  When I list the CLI binaries the workspace builds
   ✔  Then the only CLI binary should be fsquid
6 scenarios (6 passed)
12 steps (12 passed)
     Running tests/unit/route.rs (target/debug/deps/unit_route-b0fb8cfb67f91e63)
test bare_module_name_shows_module_help ... ok
test module_group_strips_the_module_name ... ok
test fsquid_is_the_only_cli_binary_in_the_workspace ... ok
test help_forms_resolve_to_help ... ok
test module_names_are_not_command_names ... ok
test unique_command_runs_at_top_level ... ok
test shared_command_names_are_qualifier_only ... ok
test unknown_command_is_an_error ... ok
test usage_names_fsquid_and_both_forms ... ok
test validate_with_module_runs ... ok
test validate_without_module_needs_the_module ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## `cargo test -p filesquid-metadata --locked` (binary removed)

```text
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
18 scenarios (15 passed, 3 skipped)
47 steps (44 passed, 3 skipped)
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Smoke test of the built binary

```text
$ fsquid --help | head -12
fsquid — the FileSquid command line

Usage:
  fsquid <command> [args]
  fsquid <module> <command> [args]

Modules: metadata

A command that only one module defines runs without the module name.
`validate` always needs it: fsquid metadata validate <prefix:term>.
Run `fsquid <module> help` for a module's commands.

$ fsquid set dcterms:title "Harbour at dusk" --object obj
set dcterms:title
exit 0
$ fsquid metadata get dcterms:title --object obj
dcterms:title Harbour at dusk
exit 0
$ fsquid validate dcterms:title
`validate` needs its module name: run fsquid metadata validate
exit 2
$ fsquid metadata validate dcterms:title
dcterms:title is a valid file dcterms term
exit 0
$ fsquid mods
unknown command: mods
exit 2
```
