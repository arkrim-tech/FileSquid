# SRS-UI-005: Paid C2PA publication
Feature: Paid C2PA publication
  As a subscriber
  I want to publish with disclosure
  So that the product matches the SRS

  Scenario: Sheet shows EU-icon disclosure and required fields
    Given I selected files for publication
    When I open the publication sheet
    Then I should see the EU-icon disclosure option
    And I should see scrubbing policy, selected-publisher timestamp, price, and Sign & Publish

  Scenario: Watermark is not the visible disclosure
    Given I complete a paid job
    When I inspect the result
    Then a watermark or C2PA manifest alone should not satisfy visible disclosure

  Scenario: Cancel leaves originals
    Given I cancel publication
    When the job stops
    Then originals should remain unchanged

  # Source
  # - srs.md SRS-UI-005
  # - prd.md F-044
