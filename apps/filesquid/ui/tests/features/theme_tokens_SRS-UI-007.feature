# SRS-UI-007: Light and dark themes
Feature: Light and dark themes
  As a library owner
  I want to use either theme
  So that the product matches the SRS

  Scenario: Switch to dark theme
    Given I am in light theme
    When I toggle theme
    Then chrome should use dark token roles

  Scenario: Primary action contrast is AA
    Given I view a primary action
    When I inspect contrast against its surface
    Then it should meet WCAG 2.2 AA

  Scenario: Body text contrast is AA
    Given I view body text on the library surface
    When I inspect contrast against its surface
    Then it should meet WCAG 2.2 AA

  # Source
  # - srs.md SRS-UI-007
  # - prd.md NFR-006
