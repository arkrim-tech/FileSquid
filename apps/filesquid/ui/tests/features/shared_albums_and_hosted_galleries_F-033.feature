# SRS-F-033: Shared albums and hosted galleries
Feature: Shared albums and hosted galleries
  As a library owner
  I want shared albums and hosted galleries
  So that the product matches the SRS

  Scenario: Happy path for F-033
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-033
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-033
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-033
  # - prd.md F-033
