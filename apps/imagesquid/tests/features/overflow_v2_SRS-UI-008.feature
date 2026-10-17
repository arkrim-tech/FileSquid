# SRS-UI-008: Unmatched v2 controls
Feature: Unmatched v2 controls
  As a library owner
  I want to reach quieter tools
  So that the product matches the SRS

  Scenario: Vault is reachable
    Given I am on the v3 library
    When I open More
    Then I should reach the local vault

  Scenario: Task monitor is reachable
    Given I am on the v3 library
    When I open More
    Then I should reach the task monitor

  Scenario: Missing overflow is a defect
    Given More omits a listed unmatched control
    When I audit the menu
    Then that omission should fail this requirement

  # Source
  # - srs.md SRS-UI-008
  # - prd.md corresponding F-IDs
  # - ui-mockups v3/v3.1
