# Task MD-0.5 Walkthrough: JSON sidecar interchange

Global task: [Task 0.16](../../../../../../resources/implementation-plan.md) (MD-0.5 is its only module slice). Module plan: [plan.md](../plan.md). Branch: `task/md-0.5-json-sidecar`.

## MD-S-010: JSON sidecar import/export MUST ship with CSV and XMP

Full requirement text ([module SRS](../srs.md)): "JSON sidecar import/export MUST ship with CSV and XMP. A sidecar is one UTF-8 JSON document per OCFL object binding `{namespace, term, value}` statements to the OCFL object identifier and the SHA-256 digest of the described logical file. The CLI MUST expose `export json` and `import json`. Import MUST apply the `/terms/` register checks of MD-S-002 and the namespace registry checks of MD-S-012 and MUST reject a sidecar whose digest does not match the target file. Export then import MUST reproduce the same statements. A JSON sidecar is interchange and MUST NOT replace `metadata.md`."

Also covered: MD-S-002 (`/terms/` register on import), MD-S-005 (CLI as the stand-alone UI), MD-S-006 (no MODS or SQL), MD-S-008 (`metadata.md` stays canonical); [PRD F-052](../../../../../../resources/prd.md#f-052-metadata-module), [SRS-F-052](../../../../../../resources/srs.md#srs-f-052-metadata-module), [ADR 022](../../../../../../resources/architecture/022-dublin-core-descriptive-metadata.md) Sidecar JSON row, [ADR 024](../../../../../../resources/architecture/024-filesquid-metadata-engine.md).

## What changed

- **`src/json.rs` (new, pure):** `JsonSidecar`, `JsonStatement`, `JsonTarget`, `JsonError`, `DIGEST_ALGORITHM`, `sha256_hex`, `to_json`, `parse_json`, `apply_json`. `apply_json` checks the algorithm (`sha256` only), the digest (hex, case-insensitive), the object identifier, then stages every statement on a copy of the record, so a rejected sidecar changes nothing.
- **Document shape:** one JSON object, for example `{"objectId":"urn:example:object-1","digestAlgorithm":"sha256","digest":"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","statements":[{"namespace":"dcterms","term":"title","value":"Harbour"}]}`, UTF-8, Unicode written unescaped. Field names follow the OCFL inventory spelling (`digestAlgorithm`).
- **Namespace checks:** `dcterms`, `exif`, `schema` are accepted; `dc` maps onto `dcterms` (MD-S-002); any other prefix is rejected with `namespace prefix <p> is not registered`. This is the MD-S-012 registry check with no extension namespaces registered; the registry itself is MD-0.7, which depends on this task.
- **CLI (`src/cli.rs`):** `export json <path>` and `import json <path>` take `--id <object-id>` and `--file <logical-path>` beside `--object <dir>`. The digest is computed from the logical file read through `filesquid_ocfl::read_logical_file` (Git guard, no path escape). Import writes only `metadata.md`; the sidecar is never written into the object. Flag parsing was flattened into `flag_value` and `positional`, and export/import dispatch moved into `render_export` and `apply_import` to keep each function small.
- **Dependencies:** `serde` 1 (derive), `serde_json` 1 (both already in the workspace lockfile), `sha2` 0.11 (current on crates.io; `sha2` is the SHA-256 implementation named in [ADR 009](../../../../../../resources/architecture/009-cryptography-webcrypto-compatible-suite.md)). All are portable Rust and do not block the planned `wasm32` build.
- **BDD:** restored "JSON sidecar" to the `Interchange excludes MODS and SQL` scenario and added six MD-S-010 scenarios (binding, round trip, digest mismatch, register, unregistered namespace, canonical store).
- **Docs:** crate README (JSON sidecar section, commands verified), module plan (MD-0.5 checked, test link), [implementation plan](../../../../../../resources/implementation-plan.md) (Task 0.16 checked, test link), component diagram (JSON sidecar BUILT), product vision summary line ("CSV, JSON sidecar and XMP interchange").

## SRS traceability

| Requirement | Tests |
|---|---|
| MD-S-010 one UTF-8 JSON document per object | `unit_json::export_is_utf8_without_escaping_unicode`, `unit_json::array_of_objects_is_rejected`, feature "JSON sidecar binds statements to the object and file digest" |
| MD-S-010 `{namespace, term, value}` bound to object identifier and SHA-256 | `unit_json::export_binds_object_id_and_digest`, `export_writes_namespace_term_value_statements`, `sha256_matches_fips_vector`, `import_rejects_other_object_id`, `unit_cli::json_import_rejects_other_object_id` |
| MD-S-010 CLI `export json` / `import json` | `unit_cli::help_lists_json_interchange`, `json_export_then_import_round_trips_through_metadata_md`, `json_requires_id_and_file` |
| MD-S-010 + MD-S-002 register checks on import | `unit_json::import_rejects_unknown_dcterms_term`, `import_rejects_accrual_term_on_file`, `import_maps_dc_onto_dcterms`, feature "JSON sidecar import applies the /terms/ register" |
| MD-S-010 + MD-S-012 unregistered namespace rejected by name | `unit_json::import_rejects_unregistered_namespace_by_name`, feature "JSON sidecar import rejects an unregistered namespace" |
| MD-S-010 reject digest mismatch | `unit_json::import_rejects_digest_mismatch`, `import_rejects_other_digest_algorithm`, `unit_cli::json_import_rejects_digest_mismatch_and_keeps_metadata_md`, feature "JSON sidecar import rejects a digest mismatch" |
| MD-S-010 export then import reproduces statements | `unit_json::export_then_import_reproduces_statements` (commas, quotes, newline, Unicode), feature "JSON sidecar export then import reproduces the statements" |
| MD-S-010 + MD-S-008 `metadata.md` stays canonical | `unit_json::rejected_import_leaves_record_unchanged`, `unit_cli::json_export_then_import_round_trips_through_metadata_md` (object holds only the file and `metadata.md`), feature "JSON sidecar does not replace metadata.md" |
| MD-S-006 no MODS or SQL | feature "Interchange excludes MODS and SQL" |

## Coverage

[ADR 017](../../../../../../resources/architecture/017-test-runners-and-reporting.md) defines no Rust line-coverage tool, so no coverage percentage is reported. Every public item of `src/json.rs` and both new CLI paths are exercised by the tests in the table above.

## TDD: failing run before implementation

`cargo test -p filesquid-metadata --locked` before `src/json.rs` existed, then `--test unit_cli`:

```text
error[E0432]: unresolved import `filesquid_metadata::json`
error: could not compile `filesquid-metadata` (test "unit_json") due to 1 previous error
error: could not compile `filesquid-metadata` (test "features") due to 1 previous error

test help_lists_json_interchange ... FAILED
test json_requires_id_and_file ... FAILED
test json_import_rejects_other_object_id ... FAILED
test json_import_rejects_digest_mismatch_and_keeps_metadata_md ... FAILED
test json_export_then_import_round_trips_through_metadata_md ... FAILED
test result: FAILED. 7 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Gates

Run from `apps/filesquid/`:

- `cargo fmt --all --check`: clean
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: clean
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`: clean
- `bash scripts/verify_tests.sh` from the repository root: passed (Clippy, workspace tests, each module crate alone, rustdoc, ESLint, `tsc` and Vitest for both UI packages: 37 and 5 Vitest tests)
- `bash scripts/pre-commit.sh` from the repository root: passed. Step 8 (`verify_docs.sh`) first failed on a broken link already on `main` (`resources/tasks/Task-ui-mockups-v1-Plan.md:42` linked to `resources/ui-mockups/v1/README.md`, which does not exist yet); on owner request that link was turned into plain text on this branch.

The three skipped feature scenarios (WASM build, byte-based extraction, Metadata panel) belong to MD-0.4 and UI-F2.4 and have no step definitions yet; they were skipped before this task too. The JUnit report is written to `tests/allure-results/cucumber-junit.xml`.

## Mocked Environment Variables

None.

## Raw test output

`cargo test -p filesquid-metadata --locked` (temporary directory shown as `$TMPDIR`):

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/filesquid_metadata-23dd843ad09b6319)

running 3 tests
test tests::name_is_unknown ... ok
test tests::title_is_allowed ... ok
test tests::accrual_is_rejected_on_file ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/filesquid-metadata.rs (target/debug/deps/filesquid_metadata-49f977226d9de7db)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/features.rs (target/debug/deps/features-88809ee59408df9c)
Feature: Reusable filesquid-metadata engine
  Scenario: Core API is namespaced metadata
filesquid-metadata — stand-alone metadata CRUD (via filesquid-ocfl)

Usage:
  filesquid-metadata <command> [args] [--object <dir>]

Commands:
  extract <file>          Extract EXIF/ID3 from a media file into metadata.md
  get [prefix:term]       Print populated statements (or one CURIE)
  set <prefix:term> <val> Set a statement and write metadata.md
  export csv|xmp|json <path>  Write CSV, XMP or JSON sidecar interchange
  import csv|xmp|json <path>  Read CSV, XMP or JSON sidecar into metadata.md
  validate <prefix:term>  Check a dcterms: term against the /terms/ register
  help                    Show this help

JSON sidecars also take --id <object-id> --file <logical-path>: the OCFL
object identifier and the logical file whose SHA-256 digest binds the
sidecar. Import rejects a sidecar whose identifier or digest differs.

Canonical store: <object>/metadata.md (YAML front matter + Markdown body).
exported json to $TMPDIR/filesquid-metadata-features-62720-1.json
exported json to $TMPDIR/filesquid-metadata-features-62720-2.json
   ✔  When I inspect the filesquid-metadata public API
exported json to $TMPDIR/filesquid-metadata-features-62720-3.json
exported json to $TMPDIR/filesquid-metadata-features-62720-4.json
imported json from $TMPDIR/filesquid-metadata-features-62720-2.json
   ✔  Then dcterms register and statement operations should be available
   ✔  And FileSquid catalog, recipe, C2PA, and collection-snapshot types should be absent
  Scenario: Unknown dcterms local names are rejected
   ✔  When I author dcterms:name on a file
   ✔  Then the write should be rejected
  Scenario: Accrual terms are rejected on a file
   ✔  When I author dcterms:accrualMethod on a file
   ✔  Then the write should be rejected
  Scenario: Canonical store is metadata.md
   ✔  When I set dcterms:title Harbour on an object
   ✔  Then metadata.md YAML front matter should contain dcterms title Harbour
   ✔  And catalogs should be rebuildable from that file
  Scenario: Persistence uses filesquid-ocfl
   ✔  When I inspect filesquid-metadata library dependencies
   ✔  Then filesquid-ocfl should be present
   ✔  And axum, tauri, libvips, and filesquid-core should be absent
  Scenario: CLI is the stand-alone UI
   ✔  When I run filesquid-metadata help
   ✔  Then I should see extract, get, set, export, import, and validate
  Scenario: CSV headers are prefix:term
   ✔  When I import a CSV with headers id,dcterms:title,dcterms:name
   ✔  Then dcterms:name should be rejected
  Scenario: Interchange excludes MODS and SQL
   ✔  When I inspect filesquid-metadata interchange
   ✔  Then CSV, JSON sidecar, XMP sidecar, and embedded extract should be available
   ✔  And MODS and SQL should be absent
  Scenario: JSON sidecar binds statements to the object and file digest
   ✔  Given an OCFL object with a logical file and dcterms:title Harbour
   ✔  When I export the object's JSON sidecar
   ✔  Then the sidecar should be one UTF-8 JSON document for that object
   ✔  And it should carry the object identifier and the SHA-256 digest of the file
   ✔  And each statement should have a namespace, term, and value
  Scenario: JSON sidecar export then import reproduces the statements
   ✔  Given an OCFL object with a logical file and dcterms:title Harbour
   ✔  When I export the object's JSON sidecar
   ✔  And I import that sidecar into an empty object holding the same file
   ✔  Then the imported metadata.md should hold the same statements
  Scenario: JSON sidecar import rejects a digest mismatch
   ✔  Given an OCFL object with a logical file and dcterms:title Harbour
   ✔  When I export the object's JSON sidecar
   ✔  And I import that sidecar into an object whose file has different bytes
   ✔  Then the import should be rejected for a digest mismatch
   ✔  And that object's metadata.md should be unchanged
  Scenario: JSON sidecar import applies the /terms/ register
   ✔  When I import a JSON sidecar holding dcterms:name
   ✔  Then the JSON import should be rejected
  Scenario: JSON sidecar import rejects an unregistered namespace
   ✔  When I import a JSON sidecar holding rico:title
   ✔  Then the JSON import should be rejected naming the prefix rico
  Scenario: JSON sidecar does not replace metadata.md
   ✔  Given an OCFL object with a logical file and dcterms:title Harbour
   ✔  When I export the object's JSON sidecar
   ✔  Then metadata.md should remain the canonical store in the object
  Scenario: Stand-alone FOSS product
   ✔  When I inspect filesquid-metadata product identity
   ✔  Then it should remain installable without the FileSquid app
   ✔  And it should build and publish without FileSquid application crates
  Scenario: Engine builds for the browser
   ?  When I build filesquid-metadata for wasm32-unknown-unknown with default features off
      Step skipped: tests/features/reusable_metadata_engine_F-052.feature:84:5
  Scenario: Extraction reads bytes from filesquid-ocfl
   ?  When I extract metadata from a logical file in an OCFL object
      Step skipped: tests/features/reusable_metadata_engine_F-052.feature:88:5
  Scenario: Metadata panel plugs into a file browser sidebar
   ?  Given the filesquid-ocfl file browser with a selected file
      Step skipped: tests/features/reusable_metadata_engine_F-052.feature:92:5
[Summary]
1 feature
18 scenarios (15 passed, 3 skipped)
47 steps (44 passed, 3 skipped)
     Running tests/unit/cli.rs (target/debug/deps/unit_cli-d11ed69630b3870e)

running 12 tests
test help_lists_json_interchange ... ok
test unknown_command_errors ... ok
test help_prints_commands ... ok
test validate_name_fails ... ok
test validate_title_ok ... ok
test json_requires_id_and_file ... ok
test set_and_get_round_trip ... ok
test export_csv_contains_header ... ok
test csv_export_still_ignores_json_flags ... ok
test json_import_rejects_other_object_id ... ok
test json_import_rejects_digest_mismatch_and_keeps_metadata_md ... ok
test json_export_then_import_round_trips_through_metadata_md ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/unit/csv.rs (target/debug/deps/unit_csv-52a4d2d43fcccfb9)

running 10 tests
test unknown_dcterms_header_rejected ... ok
test dcterms_title_header ... ok
test unterminated_quote_is_rejected ... ok
test line_skips_id_column ... ok
test quoted_whitespace_value_is_kept ... ok
test physically_blank_lines_are_skipped ... ok
test quoted_cell_maps_to_its_own_column ... ok
test round_trip_keeps_newlines ... ok
test round_trip_keeps_quotes ... ok
test round_trip_keeps_commas ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/unit/isolation.rs (target/debug/deps/unit_isolation-a9d0726dcc5e8a78)

running 2 tests
test excludes_hosts_codecs_and_filesquid_core ... ok
test depends_on_filesquid_ocfl ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/unit/json.rs (target/debug/deps/unit_json-07e5452f363b569f)

running 19 tests
test digest_algorithm_is_sha256 ... ok
test import_rejects_accrual_term_on_file ... ok
test import_merges_into_existing_record ... ok
test import_maps_dc_onto_dcterms ... ok
test array_of_objects_is_rejected ... ok
test import_accepts_uppercase_digest_hex ... ok
test export_then_import_reproduces_statements ... ok
test export_binds_object_id_and_digest ... ok
test export_is_utf8_without_escaping_unicode ... ok
test export_writes_namespace_term_value_statements ... ok
test import_rejects_other_digest_algorithm ... ok
test import_rejects_other_object_id ... ok
test import_rejects_unknown_dcterms_term ... ok
test import_rejects_unregistered_namespace_by_name ... ok
test malformed_json_is_rejected ... ok
test missing_digest_is_rejected ... ok
test rejected_import_leaves_record_unchanged ... ok
test sha256_matches_fips_vector ... ok
test import_rejects_digest_mismatch ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/unit/markdown.rs (target/debug/deps/unit_markdown-3aab8d5a2cedc7f6)

running 3 tests
test quotes_and_backslashes_survive_metadata_md ... ok
test line_breaks_survive_metadata_md ... ok
test yaml_front_matter_round_trip ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/unit/register.rs (target/debug/deps/unit_register-02af71ec754c10bc)

running 10 tests
test close_and_unmatched_terms_do_not_alias ... ok
test dc_title_maps_to_dcterms_title ... ok
test creator_is_accepted ... ok
test one_to_one_map_has_22_rows_of_registered_terms ... ok
test renamed_equivalents_map ... ok
test one_to_one_map_matches_expected_table ... ok
test one_to_one_map_is_injective_both_ways ... ok
test title_maps_to_schema_name_both_ways ... ok
test unknown_dc_name_does_not_map ... ok
test unknown_term_is_rejected ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/unit/store.rs (target/debug/deps/unit_store-a90c2da24b417c2e)

running 2 tests
test save_and_load_object_dir ... ok
test csv_line_break_survives_save_and_load ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests filesquid_metadata

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## PR #4 review fixes

Cubic review of PR #4 raised eight findings; seven are fixed here.

- `flag_value` no longer takes a recognised flag as another flag's value: `--id --file photo.jpg` now reports the missing `--id` value.
- `--id` and `--file` are flags only for `export` and `import`; `set dcterms:title --file` stores the literal `--file`.
- A `--file` logical path that resolves through a symlink to outside the object is rejected before hashing.
- `csv_export_still_ignores_json_flags` now passes `--id` and `--file`; `missing_digest_is_rejected` keeps `digestAlgorithm` and omits only `digest`.
- The document-shape example above is now valid JSON, and the product vision's `filesquid-metadata` specification lists JSON sidecar interchange and the new default dependencies.

Deferred: hashing streams the logical file instead of reading it whole. `filesquid_ocfl::read_logical_file` returns the whole file; the streaming reader is OCFL-S-004 in the open `filesquid-ocfl` read task (global Task 0.3), and MD-0.2 moves interchange onto it.

```text
$ cargo test -p filesquid-metadata --locked --test unit_cli --test unit_json

running 15 tests
test help_lists_json_interchange ... ok
test help_prints_commands ... ok
test json_rejects_a_flag_as_the_id_value ... ok
test set_and_get_round_trip ... ok
test json_requires_id_and_file ... ok
test unknown_command_errors ... ok
test validate_name_fails ... ok
test validate_title_ok ... ok
test set_keeps_json_flag_names_as_literal_values ... ok
test export_csv_contains_header ... ok
test json_rejects_a_symlink_that_leaves_the_object ... ok
test json_import_rejects_other_object_id ... ok
test csv_export_still_ignores_json_flags ... ok
test json_import_rejects_digest_mismatch_and_keeps_metadata_md ... ok
test json_export_then_import_round_trips_through_metadata_md ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 19 tests
test digest_algorithm_is_sha256 ... ok
test array_of_objects_is_rejected ... ok
test export_is_utf8_without_escaping_unicode ... ok
test export_then_import_reproduces_statements ... ok
test import_accepts_uppercase_digest_hex ... ok
test import_maps_dc_onto_dcterms ... ok
test import_rejects_other_digest_algorithm ... ok
test export_writes_namespace_term_value_statements ... ok
test import_rejects_accrual_term_on_file ... ok
test import_rejects_digest_mismatch ... ok
test import_rejects_other_object_id ... ok
test export_binds_object_id_and_digest ... ok
test import_merges_into_existing_record ... ok
test import_rejects_unknown_dcterms_term ... ok
test import_rejects_unregistered_namespace_by_name ... ok
test malformed_json_is_rejected ... ok
test missing_digest_is_rejected ... ok
test rejected_import_leaves_record_unchanged ... ok
test sha256_matches_fips_vector ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
