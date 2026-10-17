# SRS-F-020: Semantic search, OCR and visual understanding
Feature: Semantic search, OCR and visual understanding
  As a library owner
  I want semantic search, ocr and visual understanding
  So that the product matches the SRS

  Scenario: Happy path for F-020
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-020
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-020
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-020
  # - prd.md F-020
