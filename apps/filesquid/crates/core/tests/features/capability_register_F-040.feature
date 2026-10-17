# SRS-F-040: Compatibility and parity register
# SRS-TC-003: Platform and provider limits
Feature: Capability register types
  As a prospective user
  I want nonempty capability rows that do not hide unavailable children
  So that parity claims stay honest

  Scenario: A register row records status and a nonempty supported set
    When I add a supported capability "jpeg-import" with status "equivalent"
    Then the register should contain "jpeg-import"
    And the capability status should be "equivalent"
    And the supported set should be nonempty

  Scenario: Preservation-only does not count as editable parity
    When I add a supported capability "raw-preview" with status "preservation-only"
    Then preservation-only should not count as editable parity

  Scenario: Limited is distinct from equivalent
    When I add a supported capability "hdr-display" with status "limited"
    Then the capability status should be "limited"

  Scenario: An unavailable child remains visible in the aggregate
    When I add a supported capability "jpeg-import" with status "equivalent"
    And I add an unavailable child capability "heic-edit"
    Then the aggregate should expose unavailable child "heic-edit"

  Scenario: An empty register is rejected
    When I build a capability register with no rows
    Then register construction should fail

  Scenario: A register with only unsupported rows is rejected
    When I add only unsupported rows for a mandatory capability
    Then register construction should fail

  # Source
  # - resources/srs.md SRS-F-040: Compatibility and parity register
  # - resources/prd.md F-040: Compatibility and parity register
  # - resources/prd.md TC-003: Platform and provider limits
