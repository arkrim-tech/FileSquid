# SRS-F-053: Virtual exhibit
Feature: Virtual exhibit
  As a curator
  I want a public exhibit of already-public photos
  So that I can gather social-feed and cross-library public files without copying originals

  Scenario: Happy path for F-053
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-053
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-053
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-053
  # - prd.md F-053
