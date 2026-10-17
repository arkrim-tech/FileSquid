# SRS-NFR-001: Preservation and transactional integrity
Feature: Preservation and transactional integrity
  As a library owner
  I want to meet the quality target
  So that the product matches the SRS

  Scenario: Happy path for NFR-001
    Given I use a declared supported profile
    When I perform the measured workflow
    Then the NFR target should hold or be recorded as not-yet-measured

  Scenario: Bound is visible for NFR-001
    Given a bound is hit
    When the system stops or degrades
    Then I should see a safe path forward

  Scenario: Failure reporting for NFR-001
    Given the target is missed
    When I inspect diagnostics
    Then the miss should be explicit, not silent

  # Source
  # - srs.md SRS-NFR-001
  # - prd.md NFR-001
