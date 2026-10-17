# SRS-F-032: Accounts, entitlements, billing and cancellation
Feature: Accounts, entitlements, billing and cancellation
  As a customer
  I want clear charges and continued ownership after cancellation
  So that paid services do not hold my local library hostage

  Scenario: Happy path for F-032
    Given I have a paid entitlement
    When I view charges and cancel
    Then local library access and export should continue
    And hosted capacity should follow the published entitlement rules

  Scenario: Limitation is visible for F-032
    Given a platform cannot complete a payment handoff
    When I view billing
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-032
    Given a billing operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-032
  # - prd.md F-032
