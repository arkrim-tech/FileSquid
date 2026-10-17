# SRS-F-024: System sharing and extension interfaces
Feature: System sharing and extension interfaces
  As a library owner
  I want system sharing and extension interfaces
  So that the product matches the SRS

  Scenario: Happy path for F-024
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: The share menu is a small list of enabled destinations plus Save as
    Given a selected file and three enabled destinations
    When I open the share control
    Then I should see those three destinations and "Save as" and nothing else
    And each destination should show the F-046 policy that will apply

  Scenario: The user chooses and orders share destinations
    Given the share settings
    When I disable "Messaging" and move "Email" to the top
    And I open the share control
    Then "Email" should be first and "Messaging" should be absent
    And the choice should persist after restart

  Scenario: Limitation is visible for F-024
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-024
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-024
  # - prd.md F-024
