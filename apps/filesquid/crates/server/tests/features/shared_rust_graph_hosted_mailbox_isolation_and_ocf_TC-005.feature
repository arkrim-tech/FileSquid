# SRS-TC-005: Shared Rust graph, hosted mailbox isolation and OCFL writer
Feature: Shared Rust graph, hosted mailbox isolation and OCFL writer
  As a developer
  I want each host to link only what its privacy boundary allows
  So that the product matches the SRS and ADR 032

  Scenario: The ciphertext-only hosted service links no private processing
    Given I inspect the crate graph of the *.filesquid.tech hosted backup and sync service
    When I build it
    Then it should not link private application logic, native codecs, filesquid-ocfl (any backend), filesquid-metadata, or filesquid-c2pa

  Scenario: Violation is a defect for TC-005
    Given a forbidden dependency appears in the hosted service graph
    When CI runs
    Then the hosted build should fail

  Scenario: Every host serves the one FileSquid API
    Given filesquid-api in apps/filesquid/crates/api
    When I inspect the filesquid-desktop and filesquid-server graphs
    Then both hosts should depend on filesquid-api
    And that boundary should match PRD TC-005, ADR 030 and ADR 032

  Scenario: Modules run in the desktop host and the self-operated server
    Given filesquid-ocfl, filesquid-metadata, and filesquid-c2pa
    When I inspect the filesquid-desktop, filesquid-server and browser-host graphs
    Then they may depend on all three modules
    And the ciphertext-only hosted service should not depend on any of them
    And that boundary should match PRD TC-005, ADR 029 and ADR 032

  # Source
  # - resources/srs.md SRS-TC-005
  # - resources/prd.md TC-005
  # - resources/architecture/032-hosting-and-privacy-model.md
