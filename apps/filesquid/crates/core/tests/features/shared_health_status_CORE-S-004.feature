# CORE-S-004: Health status is shared by hosts.
Feature: Shared host health status
  As Axum and Tauri hosts
  I want one versioned health payload
  So that hosts share a contract without depending on each other

  Scenario: Health payload is versioned and ok
    When I request portable health status
    Then the health status should be "ok"
    And the product should be "filesquid"
    And the health schema version should be 1

  # Source
  # - apps/filesquid/crates/core/resources/srs.md CORE-S-004: Health status is shared by hosts
  # - resources/prd.md F-037: Setup, updates and diagnostics
  # - resources/srs.md SRS-F-037: Setup, updates and diagnostics
