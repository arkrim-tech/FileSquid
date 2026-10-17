# SRS-F-022: Memories and storytelling
Feature: Memories and storytelling
  As a library owner
  I want memories and storytelling
  So that the product matches the SRS

  Scenario: Happy path for F-022
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Create a memory from an album
    Given an album "Coastal Days" with twelve photos
    When I choose Create Memory from the album
    Then a memory with origin "from album" should exist with those photos in album order
    And the Memories view filtered to Created should list it

  Scenario: Favorite a memory without favoriting its photos
    Given a memory whose photos are not favorites
    When I mark the memory as a favorite from its tile
    Then the memory should be a favorite
    And none of its source photos should be favorites
    When I open the memory and toggle favorite during playback
    Then the memory should no longer be a favorite

  Scenario: Limitation is visible for F-022
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-022
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-022
  # - prd.md F-022
