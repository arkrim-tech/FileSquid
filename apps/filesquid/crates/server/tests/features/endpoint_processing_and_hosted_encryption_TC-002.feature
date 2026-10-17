# SRS-TC-002: Endpoint processing and hosted encryption
Feature: Endpoint processing and hosted encryption
  As a developer
  I want to keep the stack isolation
  So that the product matches the SRS

  Scenario: Constraint holds for TC-002
    Given I inspect the crate graph
    When I build hosted and desktop targets
    Then the MUST NOT dependencies should be absent

  Scenario: Violation is a defect for TC-002
    Given a forbidden dependency appears
    When CI runs
    Then the build should fail

  Scenario: Documented exception for TC-002
    Given a platform adapter is declared
    When I view F-040
    Then the exception should be explicit

  # Source
  # - srs.md SRS-TC-002
  # - prd.md TC-002
