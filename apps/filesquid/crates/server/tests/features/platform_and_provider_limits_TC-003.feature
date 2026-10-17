# SRS-TC-003: Platform and provider limits
Feature: Platform and provider limits
  As a developer
  I want to keep the stack isolation
  So that the product matches the SRS

  Scenario: Constraint holds for TC-003
    Given I inspect the crate graph
    When I build hosted and desktop targets
    Then the MUST NOT dependencies should be absent

  Scenario: Violation is a defect for TC-003
    Given a forbidden dependency appears
    When CI runs
    Then the build should fail

  Scenario: Documented exception for TC-003
    Given a platform adapter is declared
    When I view F-040
    Then the exception should be explicit

  # Source
  # - srs.md SRS-TC-003
  # - prd.md TC-003
