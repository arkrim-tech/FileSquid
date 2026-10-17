# SRS-F-014: AI-assisted cleanup and styles
Feature: AI-assisted cleanup and styles
  As a library owner
  I want ai-assisted cleanup and styles
  So that the product matches the SRS

  Scenario: Happy path for F-014
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Extend generates beyond the frame and marks the result as AI-manipulated
    Given a photo open in the studio and a connected F-045 task route that supports outpainting
    When I choose Extend in the Tools group and request a 16:9 frame
    Then a preview should appear with a cancel control
    When I apply the result
    Then the original should be unchanged
    And the retained rendition should carry a machine-readable AI-manipulated mark
    And the C2PA/PROV-O history should record the Extend action with model and version
    And the viewer and inspector should label the rendition AI-manipulated

  Scenario: Reframe falls back to geometric perspective without a model
    Given a photo open in the studio and no connected task route that supports Reframe
    When I choose Reframe in the Tools group
    Then the geometric perspective controls of F-012 should be offered
    And the result should not be labelled AI-manipulated

  Scenario: Extend failure leaves no partial rendition
    Given a photo open in the studio and a task route that fails during generation
    When I choose Extend and the generation fails
    Then an explicit generation-failure state should be shown
    And no retained rendition or AI mark should have been created

  Scenario: Limitation is visible for F-014
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-014
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  # Source
  # - srs.md SRS-F-014
  # - prd.md F-014
