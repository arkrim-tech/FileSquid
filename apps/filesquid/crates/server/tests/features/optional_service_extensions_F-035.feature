# SRS-F-035: Optional service extensions
Feature: Optional service extensions
  As a library owner
  I want optional service extensions with defined scope, terms, data access and cancellation
  So that the product matches the SRS

  Scenario: Happy path for F-035
    Given an optional service extension is enabled on a supported profile
    When I review it before enrolling
    Then I should see its scope, terms, data access and cancellation policy

  Scenario: Limitation is visible for F-035
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-035
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-035
  # - prd.md F-035
