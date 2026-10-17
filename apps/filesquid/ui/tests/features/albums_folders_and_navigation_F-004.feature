# SRS-F-004: Albums, folders and navigation
Feature: Albums, folders and navigation
  As a library owner
  I want albums, folders and navigation
  So that the product matches the SRS

  Scenario: Happy path for F-004
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Adding a photo to a second album creates a hand-picked virtual collection
    Given a file in collection "Coastal Days"
    When I choose Add To "Family Time"
    Then "Family Time" should be a hand-picked virtual collection containing the file
    And the file should still live in collection "Coastal Days"
    And the UI should state once in this session that the second membership is a virtual collection

  Scenario: Hand-picked virtual collection order and cover persist
    Given a hand-picked virtual collection with files in order "a,b,c"
    When I drag "c" before "a" and choose "c" as the cover
    And I restart the application
    Then the order should be "c,a,b"
    And the cover should be "c"

  Scenario: Undo restores an album removal by name
    Given three selected files in collection "Coastal Days"
    When I remove them from the collection
    Then the Edit menu should offer "Undo Remove 3 Files from Collection"
    When I choose Undo from the keyboard
    Then the three files should be members of "Coastal Days" again
    And the removal and its inverse should both be in the activity journal and the version history

  Scenario: Undo restores a deleted collection
    Given a collection "Scenic Views" with five members
    When I delete the collection and confirm
    Then the Edit menu should offer "Undo Delete Collection"
    When I choose Undo
    Then "Scenic Views" should exist with the same five members in the same order

  Scenario: Limitation is visible for F-004
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-004
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-004
  # - prd.md F-004
