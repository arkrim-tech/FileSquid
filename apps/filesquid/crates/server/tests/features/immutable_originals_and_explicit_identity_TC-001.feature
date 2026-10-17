# SRS-TC-001: Immutable originals and explicit identity
Feature: Immutable originals and explicit identity
  As a developer
  I want to keep the stack isolation
  So that the product matches the SRS

  Scenario: Constraint holds for TC-001
    Given I inspect the crate graph
    When I build hosted and desktop targets
    Then the MUST NOT dependencies should be absent

  Scenario: Violation is a defect for TC-001
    Given a forbidden dependency appears
    When CI runs
    Then the build should fail

  Scenario: Documented exception for TC-001
    Given a platform adapter is declared
    When I view F-040
    Then the exception should be explicit

  # Source
  # - srs.md SRS-TC-001
  # - prd.md TC-001
