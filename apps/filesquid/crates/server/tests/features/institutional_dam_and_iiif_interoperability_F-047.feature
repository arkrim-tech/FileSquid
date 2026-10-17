# SRS-F-047: Institutional DAM and IIIF interoperability
Feature: Institutional DAM and IIIF interoperability
  As a GLAM collection manager
  I want to publish selected collection-system and IIIF profiles
  So that institutional delivery stays gated from consumer Photos parity

  Scenario: Happy path for F-047
    Given institutional gates for F-047 are met
    When I use a supported IIIF or collection-system profile
    Then the MUST criteria for that profile should hold

  Scenario: Limitation is visible for F-047
    Given a collection-system profile is not supported
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Private roots stay private
    Given a private OCFL root
    When I configure institutional delivery
    Then that root should not become a public IIIF endpoint

  # Source
  # - srs.md SRS-F-047
  # - prd.md F-047
