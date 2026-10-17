# CORE-S-001: Identifiers for repository, library, collection, virtual collection, file, and FileVersion MUST be opaque strings.
Feature: Opaque domain identifiers
  As a host crate
  I want opaque string identifiers
  So that library and file identity does not depend on paths or capture dates

  Scenario: Library identifier round-trips as an opaque string
    Given an opaque identifier string "lib-opaque-1"
    When I construct a library id
    Then the library id should equal "lib-opaque-1"

  Scenario: File identifier is not a photo type
    Given an opaque identifier string "file-opaque-1"
    When I construct a file id
    Then the file id should equal "file-opaque-1"
    And the item type should be "file"

  Scenario: Empty identifier is rejected
    Given an opaque identifier string ""
    When I construct a library id
    Then identifier construction should fail

  Scenario: Repository, collection, virtual collection, virtual exhibit, and file version identifiers exist
    Given an opaque identifier string "id-1"
    When I construct repository, collection, virtual collection, virtual exhibit, and file version ids
    Then each constructed identifier should equal "id-1"

  Scenario: A repository may host more than one library
    Given a repository "repo-1"
    When I attach libraries "lib-a" and "lib-b" to that repository
    Then both libraries should belong to "repo-1"

  # Source
  # - apps/filesquid/crates/core/resources/srs.md CORE-S-001: Identifiers for library and file MUST be opaque strings
  # - resources/prd.md F-001: Independent local library and ownership
  # - resources/prd.md F-049: Product component terminology and item type
