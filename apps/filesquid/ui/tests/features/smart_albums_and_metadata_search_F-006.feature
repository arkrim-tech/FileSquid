# SRS-F-006: Smart albums and metadata search
Feature: Smart albums and metadata search
  As a library owner
  I want smart albums and metadata search
  So that the product matches the SRS

  Scenario: Happy path for F-006
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Search matches namespaced descriptive keys
    Given files with dcterms:creator "George" and schema:certification "WERT level1"
    When I search for dcterms:creator "George"
    Then matching files should appear
    And a Smart Album predicate on schema:certification should accept the populated value

  Scenario: Quick filters include unfiled and are combinable
    Given a library with files in collections and files in no collection or virtual collection
    When I open the Filter menu
    Then it should offer Photos, Videos, Favorites, Edited, Screenshots, configured keywords, Unfiled and Manage Keywords
    When I choose Unfiled and Favorites together
    Then only favorite files in no collection or virtual collection should be shown
    And the active filter set should be visible

  Scenario: Contributed-by-me quick filter appears only in a shared library
    Given I am a participant in a shared library with contributions from two people
    When I open the Filter menu
    Then it should offer "Contributed by me"
    When I choose it
    Then only files I contributed should be shown
    Given I open a library that is not shared
    Then the Filter menu should not offer "Contributed by me"

  Scenario: Limitation is visible for F-006
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-006
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-006
  # - prd.md F-006
