# F-054: C2PA module
# SRS-F-054: C2PA module
Feature: Reusable filesquid-c2pa engine
  As a developer adding C2PA to my own application
  I want to integrate filesquid-c2pa as one FOSS library
  So that I can read, validate, mint and edit C2PA manifests without installing the FileSquid app

  Scenario: Read a manifest store
    Given a JPEG with an embedded C2PA manifest store
    When I read it with filesquid-c2pa
    Then the active manifest, its claim and its assertions should be returned

  Scenario: Validate against a caller-supplied trust list
    Given a signed asset and a trust list supplied by the caller
    When I validate it with filesquid-c2pa
    Then the result should use C2PA validation status codes
    And an untrusted signing certificate should be reported, not ignored

  Scenario: Mint and sign through a caller-supplied signer
    Given an unsigned PNG and a test signer
    When I mint a manifest with a created action and sign it
    Then the embedded manifest should validate in filesquid-c2pa
    And the official c2pa oracle should read the same manifest

  Scenario: Edits never change a signed manifest in place
    Given a signed asset
    When I record an edit
    Then a new manifest should reference the prior asset as a parent ingredient

  Scenario: Unsupported formats are reported
    Given an asset in a format without a handler
    When I read it with filesquid-c2pa
    Then the result should say the format is unsupported

  Scenario: Stand-alone dependencies
    When I inspect filesquid-c2pa library dependencies
    Then filesquid-ocfl, axum, tauri, sqlite, libvips, ffmpeg, and filesquid-core should be absent from default dependencies
    And no runtime dependency on the official c2pa crate should exist

  Scenario: Engine builds for the browser
    When I build filesquid-c2pa for wasm32-unknown-unknown with default features off
    Then the build should succeed

  Scenario: Content Credentials panel plugs into a file browser sidebar
    Given a selected signed file in a host file browser
    When the host registers the filesquid-c2pa Content Credentials panel
    Then the panel should show the signer, assertions, and validation status

  Scenario Outline: Content Credentials panel shows each credential state distinctly
    Given a selected <state> file in a host file browser
    When the host registers the filesquid-c2pa Content Credentials panel
    Then the panel should show the <state> credential state
    And the panel should not show any other credential state

    Examples:
      | state       |
      | unsigned    |
      | signed      |
      | invalid     |
      | unavailable |

  # Source
  # - resources/prd.md F-054
  # - resources/srs.md SRS-F-054
  # - resources/architecture/026-filesquid-provenance-engine.md
  # - resources/architecture/028-filesquid-ui-packages-and-panel-composition.md
