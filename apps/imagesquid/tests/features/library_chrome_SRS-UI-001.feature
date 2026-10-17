# SRS-UI-001: Library chrome
Feature: Library chrome
  As a library owner
  I want to browse my library
  So that the product matches the SRS

  Scenario: Browse the library grid
    Given I have an unlocked library with files
    When I open the library surface
    Then I should see albums, galleries, Archives sync status, search, and a file grid

  Scenario: Sidebar and search follow the canonical screenshot
    Given I am on the library surface
    Then the sidebar groups should be Library, Collections, Albums, Exhibits and Smart Queries in that order
    And the search placeholder should read "Search photos, collections, dates…"
    And the Smart Queries should include AI Synthetic, Hardware Certified and Unsigned

  Scenario: Provenance skim is on by default
    Given I open All Photos
    Then every tile should show an AI or RW origin chip
    When I toggle the CR chip
    Then the tiles should show no origin chips

  Scenario: C2PA floatover shows the origin badge
    Given provenance skim is on
    When I hover a signed tile
    Then a floatover should show the badge Real or AI, Captured, Camera, Signed by, C2PA and View full details

  Scenario: More reveals unmatched tools
    Given I am on the library surface
    When I open More
    Then I should reach vault, archive root, migration, publish, and task monitor

  Scenario: Empty library is labeled
    Given I have a new empty library
    When I open the library surface
    Then I should see an empty state, not a silent blank grid

  # Source
  # - srs.md SRS-UI-001
  # - prd.md corresponding F-IDs
  # - apps/imagesquid/resources/ui-mockups/imagesquid-ui-mockups/screen1-light-mode.png (canonical)
