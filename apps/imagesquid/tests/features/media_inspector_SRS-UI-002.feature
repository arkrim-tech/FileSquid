# SRS-UI-002: Media inspector
Feature: Media inspector
  As a library owner
  I want to inspect provenance
  So that the product matches the SRS

  Scenario: Open inspector for a selected file
    Given I have selected a file
    When I open the inspector
    Then I should see C2PA overview, cryptographic credentials, and inspection assertions
    And populated exif and dcterms rows with namespace prefixes
    And no empty /terms/ keys as blank rows

  Scenario: Signed and unsigned stay distinct
    Given one file has a valid signed C2PA manifest and another has only a prepared unsigned record
    When I inspect each file
    Then signed and unsigned states should be distinct
    And neither should be presented as EU AI Act certification

  Scenario: Invalid and unavailable checks are distinct
    Given a file with a failed C2PA binding and a file whose remote check is unavailable
    When I open the inspector
    Then invalid signature and unavailable-check states should be distinct from unsigned

  Scenario: Inspector is composed from filesquid panels
    Given a selected photo
    When I open the inspector
    Then Overview and Descriptive should come from the filesquid-metadata Metadata panel
    And C2PA should come from the filesquid-c2pa Content Credentials panel
    And Archive should come from the filesquid-ocfl Details and Versions panels
    And panel labels should use ImageSquid wording

  # Source
  # - srs.md SRS-UI-002
  # - prd.md F-043 F-046
  # - architecture/028-filesquid-ui-packages-and-panel-composition.md
