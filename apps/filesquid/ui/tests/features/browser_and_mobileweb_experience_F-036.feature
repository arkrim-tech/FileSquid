# SRS-F-036: Browser and mobile-web experience
Feature: Browser and mobile-web experience
  As a library owner
  I want browser and mobile-web experience
  So that the product matches the SRS

  Scenario: Happy path for F-036
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-036
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-036
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-036
  # - prd.md F-036
