# SRS-UI-003: Non-destructive studio
Feature: Non-destructive studio
  As a library owner
  I want to edit without replacing originals
  So that the product matches the SRS

  Scenario: Commit keeps original and retained history
    Given I have opened a file in the studio
    When I apply a non-destructive adjustment and commit
    Then the original bytes should remain unchanged
    And the committed derivative should keep its non-destructive parameters and edit history

  Scenario: Assistive crop is not AI-generated
    Given I crop a file
    When I save the retained derivative
    Then it should not be labeled AI-generated

  Scenario: AI tool failure is visible
    Given an AI tool fails
    When I attempt to apply it
    Then I should see an error and the original should remain

  # Source
  # - srs.md SRS-UI-003
  # - prd.md F-011 F-014
