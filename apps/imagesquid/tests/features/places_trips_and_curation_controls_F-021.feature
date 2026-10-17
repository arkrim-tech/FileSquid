# SRS-F-021: Places, trips and curation controls
Feature: Places, trips and curation controls
  As a library owner
  I want places, trips and curation controls
  So that the product matches the SRS

  Scenario: Happy path for F-021
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-021
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-021
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-021
  # - prd.md F-021
