# F-064: File activity, comments and user activity stream
# SRS-F-064: File activity, comments and user activity stream
# API-S-013: Activity stream, per-file activity and comments over the journal
Feature: File activity, comments and user activity stream
  As a user
  I want to see what happened to my files and who did it, comment with the people who can see a file, and follow my own activity
  So that I understand my files' history without being an administrator

  Background:
    Given a filesquid-server with users Ana, Ben, Cara and Dana
    And Ana owns the file "proposal.pdf" and shared it with Ben as commenter and with Dana as viewer
    And Cara may not read "proposal.pdf"
    And the journal holds, on files Ana may read, events by Ana and by Ben of every kind: file changes, favorites, shares, comments and tags
    And the journal holds an event on a file Ana may not read

  # Single source
  Scenario: The stream is derived from the journal
    When Ana renames "proposal.pdf" to "proposal-v2.pdf"
    Then the activity journal should contain the rename with opaque ids
    And Ana's activity stream should show "You renamed proposal.pdf to proposal-v2.pdf" resolved from those ids
    And no second activity store should exist

  Scenario: Users never learn of files through activity they cannot browse
    When Cara opens her activity stream
    Then no entry about "proposal.pdf" should appear
    When Cara requests the per-file activity of "proposal.pdf"
    Then the response should be 403

  # Stream
  Scenario: The stream is filterable, paged and grouped by day
    Given activity by Ana and Ben over the last three days
    When Ana opens her stream with the filter "by others"
    Then only Ben's events on files Ana may read should appear
    And entries should be grouped by day in Ana's zone with actor, action, object and time
    And paging should use a cursor

  Scenario Outline: Each filter narrows the stream
    When Ana applies the filter "<filter>"
    Then at least one entry should appear
    And only "<events>" should appear
    And no entry about the file Ana may not read should appear

    Examples:
      | filter       | events                                               |
      | by me        | events Ana triggered                                 |
      | by others    | events other users triggered                         |
      | file changes | create, modify, delete, rename, move and restore     |
      | favorites    | favorite and unfavorite                              |
      | shares       | share, unshare and public-link downloads             |
      | comments     | comment create, edit and delete                      |
      | tags         | dcterms:subject changes                              |

  Scenario: Events from every enabled module appear
    Given the c2pa module is enabled
    When a C2PA scan completes on Ana's files
    Then Ana's stream should show the scan completion
    And the filter list should offer the c2pa module's event types

  # Per-file activity
  Scenario: Per-file activity lists versions, metadata, shares, downloads and comments
    Given "proposal.pdf" received a new version, a dcterms:title change, a share to Ben, a public-link download and a comment
    When Ana opens the Activity panel for "proposal.pdf"
    Then all five events should be listed in order with actor and relative time

  # Comments
  Scenario: Commenters post, viewers only read
    When Ben comments "Looks good" on "proposal.pdf"
    Then Ana and Dana should see Ben's comment in the Activity panel
    When Dana tries to comment on "proposal.pdf"
    Then the response should be 403 because Dana's share role is viewer
    And the comment should be journaled
    And the comment should be stored as application data recoverable with the file's OCFL object

  Scenario: Authors edit or delete their own comments and owners remove any
    Given Ben commented "Looks good" and Ana commented "Thanks" on "proposal.pdf"
    When Ben edits his comment to "Looks great"
    Then the edit should succeed and be journaled
    When Ana deletes Ben's comment
    Then the deletion should succeed because Ana owns the file
    When Ben tries to delete a comment Ana wrote
    Then the response should be 403

  Scenario: Comments do not leave with shared or published copies unless selected
    Given "proposal.pdf" has two comments
    When Ana publishes a copy through a public link with the default outgoing policy
    Then the delivered bytes and metadata should contain no comment text
    When Ana selects comments in the outgoing policy for a shared album export
    Then the comments should accompany that export

  Scenario: Mentions may notify but never grant access
    When Ben comments "@cara please review" on "proposal.pdf"
    Then Cara may receive a notification according to her preferences
    And Cara still may not read "proposal.pdf"

  # Publishing
  Scenario: No RSS or bearer-link feed exists
    When Ana looks for an activity feed link or an RSS endpoint
    Then none should exist
    And the API contract should contain no feed operation

  Scenario: Publishing to Bluesky is explicit, policy-checked and journaled
    Given Ana connected her AT Protocol account and "logo.png" is in a public virtual exhibit
    When Ana chooses publish to Bluesky on "logo.png"
    Then the outgoing-disclosure preview should show the scrubbed metadata, the FileId filename and the warning that the remote network strips metadata
    When Ana confirms
    Then one post should be created on Ana's account with the policy-applied bytes
    And the publication should be journaled and recorded as a disclosure action on the file's C2PA record
    And Ana's activity stream should show the publication

  Scenario: Private items cannot be published and nothing publishes automatically
    When Ana chooses publish to Bluesky on the private "proposal.pdf"
    Then the action should be refused because the file is not public
    When a new public file is added to Ana's exhibit
    Then no post should be created without an explicit publish request

  Scenario: Credentials stay in the host's secret store
    When Ana connects her AT Protocol account
    Then the credentials should be stored through the host's secret store
    And no journal entry, stdout line or OCFL object should contain them
    When Ana disconnects the account
    Then the stored credentials should be revoked

  # Events
  Scenario: Live events are filtered per subscriber
    Given Ana and Cara are subscribed to the event channel
    When Ben comments on "proposal.pdf"
    Then Ana should receive a comment.update event
    And Cara should receive nothing about it

  # Default UI
  Scenario: The Activity panel is independent and the stream has its own view
    When I mount the Activity panel alone in a sidebar slot
    Then it should render per-file activity and a comment box without any other panel present
    When I open the activity view from the header
    Then the stream should show with the filters and relative times

  # Source
  # - resources/prd.md F-064
  # - resources/srs.md SRS-F-064
  # - apps/filesquid/crates/api/resources/srs.md API-S-013
  # - apps/filesquid/ui/resources/srs.md UI-S-008
