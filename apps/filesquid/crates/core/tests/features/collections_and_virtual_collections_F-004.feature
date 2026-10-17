# SRS-F-004: Albums, folders and navigation
# SRS-F-049: Product component terminology and item type
Feature: Collections, folders, and virtual collections
  As a curator
  I want physical collections and virtual memberships
  So that extra albums do not duplicate stored originals

  Scenario: Collection removal retains files
    Given a file whose home collection is "col-home"
    When I remove collection "col-home" from the model
    Then the file identifier should still exist

  Scenario: Nested folders form a hierarchy
    Given a folder "parent" in a library
    When I nest folder "child" under "parent"
    Then the child folder parent should be "parent"

  Scenario: Ordered membership is preserved
    Given a collection with files in order "a,b,c"
    When I inspect membership order
    Then the membership order should be "a,b,c"

  Scenario: Virtual collections are not a second store
    Given a file whose home collection is "col-home"
    When I add virtual membership in "vc-gallery"
    Then the file home collection should be "col-home"
    And the virtual collection should not store bytes

  Scenario: A hand-picked virtual collection keeps its own order without a second copy
    Given a file whose home collection is "col-home"
    When I add the file to hand-picked virtual collection "vc-family" at position 1
    And I add the file to hand-picked virtual collection "vc-coast" at position 3
    Then the file home collection should be "col-home"
    And the membership order of "vc-family" should place the file at position 1
    And the membership order of "vc-coast" should place the file at position 3
    And neither virtual collection should store bytes

  Scenario: Removing a file from a hand-picked virtual collection leaves other memberships alone
    Given a file in home collection "col-home" and hand-picked virtual collections "vc-family,vc-coast"
    When I remove the file from "vc-family"
    Then the file home collection should be "col-home"
    And the file should remain a member of "vc-coast"

  Scenario: An organization command has a compensating inverse
    Given a file in home collection "col-home" and hand-picked virtual collection "vc-family"
    When I remove the file from "vc-family"
    And I apply the compensating inverse of that command
    Then the file should remain a member of "vc-family"
    And both commands should appear in the activity journal
    And each command should have produced its own version

  Scenario: Collection sort and cover are portable fields
    Given a collection titled "Holidays" with date descending sort and cover "file-cover"
    Then the collection sort should be date descending
    And the collection cover should be "file-cover"

  Scenario: A collection belongs to exactly one library
    When I construct collection "col-1" in library "lib-1"
    Then the collection should belong to library "lib-1"

  Scenario: A virtual exhibit belongs to a repository and is public
    Given a repository "repo-1"
    When I attach libraries "lib-a" and "lib-b" to that repository
    And I construct virtual exhibit "ex-1" on that repository
    Then the virtual exhibit should belong to "repo-1"
    And the virtual exhibit should not store bytes
    And virtual exhibit members must be public

  # Source
  # - resources/srs.md SRS-F-004: Albums, folders and navigation
  # - resources/prd.md F-004: Albums, folders and navigation
  # - resources/prd.md F-049: Product component terminology and item type
