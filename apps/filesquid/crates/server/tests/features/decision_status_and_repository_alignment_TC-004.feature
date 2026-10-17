# SRS-TC-004: Decision status and repository alignment
Feature: Decision status and repository alignment
  As a developer
  I want to keep the stack isolation
  So that the product matches the SRS

  Scenario: Constraint holds for TC-004
    Given I inspect the crate graph
    When I build hosted and desktop targets
    Then the MUST NOT dependencies should be absent

  Scenario: Violation is a defect for TC-004
    Given a forbidden dependency appears
    When CI runs
    Then the build should fail

  Scenario: Documented exception for TC-004
    Given a platform adapter is declared
    When I view F-040
    Then the exception should be explicit

  # Source
  # - srs.md SRS-TC-004
  # - prd.md TC-004
