# CORE-S-002: The item type is file.
Feature: Canonical item type
  As a host crate
  I want a single item type
  So that Photo, Image, and Video are never minted as domain types

  Scenario: The only item type is file
    When I inspect the portable item type
    Then the item type should be "file"

  Scenario: Photo, image, and video are not domain item types
    When I deserialize "photo" as an item type
    Then item type deserialization should fail
    When I deserialize "image" as an item type
    Then item type deserialization should fail
    When I deserialize "video" as an item type
    Then item type deserialization should fail

  # Source
  # - apps/filesquid/crates/core/resources/srs.md CORE-S-002: The item type is file
  # - resources/prd.md F-049: Product component terminology and item type
