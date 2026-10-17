# SRS-F-011: Non-destructive editing workflow
Feature: Non-destructive editing workflow
  As a library owner
  I want non-destructive editing workflow
  So that the product matches the SRS

  Scenario: Happy path for F-011
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-011
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-011
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-011
  # - prd.md F-011
