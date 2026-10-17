# SRS-F-001: Independent local library and ownership
Feature: Independent local library and ownership
  As a library owner
  I want a library I can copy and reconstruct without FileSquid
  So that ownership does not depend on a running service

  Scenario: Closed-app copy reconstructs organization
    Given I have a managed local library with a completed OCFL snapshot
    When I close FileSquid and copy that snapshot with ordinary filesystem tools
    Then I should reconstruct supported organization without the operational database or a running FileSquid service

  Scenario: Changes stay pending until the snapshot is durable
    Given I import or change files
    When the OCFL snapshot is not yet durable
    Then those changes should remain visibly pending

  Scenario: Inspection does not authorize in-place inventory edits
    Given I can inspect and copy a completed snapshot
    When I view that snapshot outside FileSquid
    Then that access should not authorize in-place edits of OCFL inventories or signed originals

  # Source
  # - srs.md SRS-F-001
  # - prd.md F-001
