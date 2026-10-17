# F-062: Drive-style views, sorting, selection and layout preferences
# SRS-F-062: Drive-style views, sorting, selection and layout preferences
# UI-S-006: Default browser views, sorting, selection, item actions and preferences
Feature: Drive-style views, sorting, selection and layout preferences
  As a user coming from Google Drive or Nextcloud
  I want the familiar views, sort orders, selection and layout choices in the FileSquid default UI
  So that I can work the way I already know and have my choices kept

  Background:
    Given I am signed in to the FileSquid default UI on a host with a library I may read
    And the library holds folders and files with different names, sizes and modified times

  # Views
  Scenario: The sidebar offers the eight views and the deleted-files entry
    When I open the file browser
    Then the sidebar should list home, all files, personal library, recent, favorites, shares, tags and deleted files under the label table's names
    And the sidebar should show a view filter field at the top and storage used and settings at the bottom

  Scenario Outline: Each view is a bounded, stably ordered query
    Given the "<view>" view holds more items than one page
    When I open the "<view>" view and page through it twice
    Then every page should come from the API with a cursor and no item should repeat or disappear between pages

    Examples:
      | view          |
      | home          |
      | all files     |
      | personal      |
      | recent        |
      | favorites     |
      | shares        |
      | tags          |
      | deleted files |

  Scenario: Shares view distinguishes with me, by me and by link
    Given one file shared with me, one I shared with a colleague and one I shared by public link
    When I open the shares view
    Then each file should show which of the three kinds of share it carries

  # Sorting
  Scenario: Column headers sort the list and reverse on a second click
    When I click the "modified" column header
    Then the list should be ordered by latest OCFL version created time, descending
    When I click the "modified" column header again
    Then the order should reverse

  Scenario: Grid layout sorts through a sort menu
    Given the grid layout is active
    When I choose "name" in the sort menu and then reverse it
    Then the tiles should be ordered by name, descending

  Scenario: Last modified by me and last opened by me use only my own activity
    Given another user modified and opened a file after I did
    When I sort the view by "last modified by me"
    Then the order should follow my own modifications
    When I sort the view by "last opened by me"
    Then the order should follow my own openings
    And neither order should reveal when or whether the other user touched the file

  Scenario: Folders on top or mixed with files
    When I choose "folders on top"
    Then every folder should appear before the first file
    When I choose "mixed with files"
    Then folders and files should interleave in the sort order

  Scenario: Fixed-order views say so
    When I open the recent view
    Then the view should state that its order is fixed
    And no disabled sort control should be shown

  # Selection and actions
  Scenario: Row and header checkboxes select items and open a bulk action bar
    When I tick two row checkboxes
    Then a bulk action bar should show "2 selected" with favorite, move or copy, download and delete
    When I tick the header checkbox
    Then every item in the view should be selected and the bar should state the effective scope

  Scenario: Per-item menu and right-click offer the same actions
    When I open the actions menu of a file
    Then it should offer favorite, details, manage tags, rename, move or copy, download, share and delete
    When I right-click the same file
    Then the same menu should open

  Scenario: Reveal in file manager appears only on the desktop host
    Given the UI is connected to filesquid-desktop
    When I open the actions menu of a file
    Then it should offer reveal in file manager
    Given the UI is connected to filesquid-server
    When I open the actions menu of a file
    Then it should not offer reveal in file manager

  Scenario: Drag-to-folder moves and the menu offers a folder picker
    When I drag a file onto a folder
    Then the file should move into that folder as a new OCFL version
    When I choose move or copy from the actions menu
    Then a folder picker should open

  Scenario: Copy creates a new file with provenance; move commits a new version
    When I copy a file into another collection
    Then a new file with its own FileId should exist whose provenance names the source file
    And the source file's bytes should still live in exactly one collection
    When I move a file into another folder
    Then the file should have a new OCFL version and the same FileId

  Scenario: Multi-item download streams one ZIP within declared caps
    Given the ZIP download caps are 1,000 members and 10 GiB per archive
    And three selected files total 300 MiB
    When I download the three selected files
    Then one ZIP archive should stream to me without the host buffering it whole
    And each member should carry the outgoing metadata policy
    And a manifest entry should list any member that was omitted or unavailable
    When I select 1,001 items and download
    Then the UI should refuse, state the 1,000-member cap and offer to narrow the selection
    When I select 20 items totalling 12 GiB and download
    Then the UI should refuse, state the 10 GiB cap and offer to narrow the selection

  # Layout, preview and preferences
  Scenario: List is the default layout and shows the stated columns
    When I open the file browser for the first time
    Then the list layout should be active
    And rows should show a checkbox, name, size and modified, a share control and an actions menu
    And a footer should summarise files, folders and total size

  Scenario: One New control
    When I open the New control
    Then it should offer upload files, upload folder and new folder in one menu
    When I drag files from my file manager onto the list
    Then they should upload into the current folder

  Scenario: A pending empty folder is never persisted
    When I create a new folder and add nothing to it
    Then the folder should show as pending
    And no OCFL version should have been written for it

  Scenario: Clicking a name previews or offers download
    When I click an image name
    Then an in-app preview should open
    When I click a file whose format has no preview
    Then the UI should offer download and say why no preview is available

  Scenario: Hover card is optional and respects touch and reduced motion
    Given the hover card setting is on
    When I hover a file icon with a pointer
    Then a card should show thumbnail, size, modified time and last editor
    Given the operating system requests reduced motion
    When I hover a file icon
    Then no card should show
    When I turn the hover card setting off and hover a file icon
    Then no card should show

  Scenario: Preferences follow the user across hosts
    Given I set the start view to home, sort by size descending, folders on top, grid layout and the hover card off on the desktop host
    When I sign in to the same library through filesquid-server
    Then the browser should open on the home view with the same sort, placement and layout
    And the hover card should stay off

  Scenario: Storage used is reported per library and per collection
    When I open the storage used indicator
    Then I should see bytes used for the library and for each collection, including version history
    And I should be able to sort a view by size to find large files

  # Source
  # - resources/prd.md F-062
  # - resources/srs.md SRS-F-062
  # - apps/filesquid/ui/resources/srs.md UI-S-006
