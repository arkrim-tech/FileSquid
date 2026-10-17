# SRS-UI-006: Mobile web companion
Feature: Mobile web companion
  As a browser user
  I want to browse without desktop
  So that the product matches the SRS

  Scenario: Browse uploaded content
    Given I have unlocked the browser app
    When I open the library
    Then I should see available files

  Scenario: No background backup promise
    Given I close the mobile browser
    When long-running capture was in progress
    Then the product should not claim continuous backup after close

  Scenario: Cache loss is honest
    Given browser storage is cleared
    When I reopen the app
    Then I should see recovery, not a silent empty library

  # IS-S-011 (FR-01)
  Scenario: Sync Now starts ingestion from the main screen
    Given I have a remembered photo folder with granted access
    When I tap "Sync Now"
    Then a photo sync should start
    And tapping "Sync Now" again should not start a second sync

  # IS-S-012 (FR-02)
  Scenario: Sync on launch only with granted access
    Given my remembered photo folder reports permission "granted"
    When I open the app
    Then a photo sync should start without any action from me

  Scenario: Launch with prompt permission offers one-tap resume
    Given my remembered photo folder reports permission "prompt"
    When I open the app
    Then no permission prompt or picker should open
    And I should see "Resume photo access"

  # IS-S-013 (FR-03), IS-S-020 (FR-10)
  Scenario Outline: Access path follows detected capabilities
    Given the browser <directory> directory access
    When I tap "Sync Now"
    Then the app should use the <path>

    Examples:
      | directory     | path                     |
      | provides      | remembered folder        |
      | does not have | multi-select file picker |

  # IS-S-014 (FR-04)
  Scenario: Folder chosen once is reused
    Given directory access is available
    And I chose a photo folder in an earlier sync
    When I tap "Sync Now"
    Then I should not be asked to choose the folder again

  Scenario: Revoked folder access is reported
    Given my remembered photo folder reports permission "denied"
    When I tap "Sync Now"
    Then I should be told folder access was lost
    And I should be offered to re-grant or choose the folder again

  # IS-S-015 (FR-05)
  Scenario: Only photos after the last sync are considered
    Given my last successful sync completed at "2026-10-16T12:00:00Z"
    And the folder has 3 photos modified before and 2 after that time
    When I tap "Sync Now"
    Then 2 photos should be ingested

  Scenario: A lost sync record does not duplicate photos
    Given every photo in my folder is already in the library
    And the sync record has been cleared
    When I tap "Sync Now"
    Then no photo should be ingested twice

  Scenario: A partial sync does not advance the sync time
    Given a sync is interrupted before every photo is verified
    When I reopen the app
    Then the last successful sync time should be unchanged

  # IS-S-016 (FR-06)
  Scenario: Last sync time is always shown
    Given I have never synced
    When I open the main screen
    Then I should see "Not synced yet"
    And it should be separate from the Archives status

  # IS-S-017 (FR-07)
  Scenario: New photo count before ingestion
    Given 14 new photos and 3 already-imported photos are found
    When the sync starts
    Then I should see "14 new photos found"
    And progress should count toward 14

  # IS-S-018 (FR-08)
  Scenario: Picker-only platform explains per-session selection
    Given the browser does not have directory access
    When I open the sync screen
    Then I should be told photos are selected each sync session
    And "Sync Now" should open the multi-select picker in one tap
    And already-imported photos I pick should be skipped without asking

  # IS-S-019 (FR-09)
  Scenario: Captured photo goes to the library
    Given camera capture is available
    When I take a photo with "Take Photo"
    Then it should be added to the current library without another step
    And until its upload is verified it should show "Not yet in library"

  # IS-S-021 (FR-11)
  Scenario: Unsupported sync options are not shown
    Given the browser does not have directory access or camera capture
    When I open the main screen
    Then folder selection, re-grant and sync-on-launch controls should be absent
    And "Take Photo" should be absent

  # Source
  # - srs.md SRS-UI-006
  # - prd.md corresponding F-IDs
  # - ui-mockups v3/v3.1
