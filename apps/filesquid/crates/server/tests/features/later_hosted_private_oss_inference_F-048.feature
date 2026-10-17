# SRS-F-048: Later hosted private OSS inference
Feature: Later hosted private OSS inference
  As a library owner
  I want later hosted private oss inference
  So that the product matches the SRS

  Scenario: Happy path for F-048
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-048
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-048
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-048
  # - prd.md F-048
