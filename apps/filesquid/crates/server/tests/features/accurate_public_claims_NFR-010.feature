# SRS-NFR-010: Accurate public claims
Feature: Accurate public claims
  As a library owner
  I want public claims to describe what was tested accurately
  So that the product matches the SRS

  Scenario: Happy path for NFR-010
    Given a public claim about a supported profile
    When I compare it with the recorded evidence
    Then it should describe the tested migration scope, encryption trust boundaries, hosting provider and geography, and cancellation policy accurately

  Scenario: Hypotheses are distinguished for NFR-010
    Given a claim that has not yet been measured
    When I read it
    Then it should be labelled as a hypothesis, not a measured result

  Scenario: Measured results report denominators for NFR-010
    Given a claim based on a measured result
    When I read it
    Then it should state the denominator the result was measured against

  Scenario: Failure reporting for NFR-010
    Given a claim no longer matches the evidence
    When I inspect diagnostics
    Then the mismatch should be explicit, not silent

  # Source
  # - srs.md SRS-NFR-010
  # - prd.md NFR-010
