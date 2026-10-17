# SRS-TC-001: Immutable originals and explicit identity
# SRS-F-011: Non-destructive editing workflow
Feature: Immutable originals and versioned recipes
  As a library owner
  I want originals to stay unchanged
  So that edits are recipes and versions rather than mutated bytes

  Scenario: A file has exactly one home collection
    Given a file whose home collection is "col-home"
    When I assign home collection "col-other"
    Then assigning a second home collection should fail
    And the file home collection should be "col-home"

  Scenario: Extra membership is a virtual collection
    Given a file whose home collection is "col-home"
    When I add virtual membership in "vc-extra"
    Then the file home collection should be "col-home"
    And the file should have a virtual membership in "vc-extra"

  Scenario: Original bytes cannot be replaced
    Given an original file resource
    When I replace its stored bytes
    Then replacing original bytes should fail
    And the original digest should be unchanged

  Scenario: A file version references a recipe without mutating the original
    Given an original file resource
    When I create a file version with a recipe operation
    Then the version should reference the recipe
    And the original digest should be unchanged

  # Source
  # - resources/srs.md SRS-F-011: Non-destructive editing workflow
  # - resources/srs.md SRS-TC-001: Immutable originals and explicit identity
  # - resources/prd.md F-011: Non-destructive editing workflow
  # - resources/prd.md TC-001: Immutable originals and explicit identity
