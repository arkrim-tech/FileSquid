# SRS-F-005: Favorites, metadata and keywords
Feature: Favorites, metadata and keywords
  As a library owner
  I want favorites, metadata and keywords
  So that the product matches the SRS

  Scenario: Happy path for F-005
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Title caption and keywords write Dublin Core terms
    Given I have selected a file
    When I set the title to "Harbour", the caption to "Evening light", and a keyword "boats"
    Then the descriptive record should include dcterms:title "Harbour"
    And dcterms:description "Evening light"
    And dcterms:subject "boats"
    And the managed original filename should be unchanged

  Scenario: Limitation is visible for F-005
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-005
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-005
  # - prd.md F-005
