# F-066: File and folder sharing roles, links and shares view
# SRS-F-066: File and folder sharing roles, links and shares view
# SV-S-005, SV-S-006: Share records, roles, links, ownership transfer and administrator switches
Feature: File and folder sharing roles, links and shares view
  As a filesquid-server user
  I want to share a file or folder with people, groups or by link with a clear role, and see what is shared with me, by me and publicly
  So that collaboration is explicit and nothing is exposed by accident

  Background:
    Given a filesquid-server with users Ana, Ben and Cara and the group "Design"
    And Cara is a member of "Design"
    And Ana owns the folder "Campaign" containing "brief.pdf" and "logo.png"

  # Roles
  Scenario: Viewer can read and download only
    When Ana shares "brief.pdf" with Ben as viewer
    Then Ben should be able to read and download "brief.pdf"
    And Ben should be refused when commenting, renaming or uploading a new version

  Scenario: Commenter adds comments
    When Ana shares "brief.pdf" with Ben as commenter
    Then Ben should be able to read, download and comment
    And Ben should be refused when renaming or uploading a new version

  Scenario: Editor commits versions, renames and moves within the share
    When Ana shares the folder "Campaign" with the group "Design" as editor
    And Cara, a member of "Design", uploads a new version of "logo.png" and renames "brief.pdf"
    Then both changes should commit as new OCFL versions
    When Cara tries to move "logo.png" outside "Campaign"
    Then the move should be refused
    When Cara tries to delete "logo.png" from the library
    Then the deletion should be refused because only the owner may delete

  Scenario: Every call is deny-by-default
    Given no share exists on "logo.png"
    When Ben requests "logo.png"
    Then the response should be 403
    And the attempt should be journaled

  Scenario: Re-sharing is an owner-controlled option
    When Ana shares "brief.pdf" with Ben as viewer without re-share
    And Ben tries to share "brief.pdf" with Cara
    Then the share should be refused
    When Ana enables re-share on that share
    And Ben shares "brief.pdf" with Cara as viewer
    Then Cara should be able to read "brief.pdf"

  # Links
  Scenario: Public links require an explicit action and apply the outgoing policy
    When Ana creates a public link for "logo.png" with a password and an expiry
    Then the UI should disclose that anyone holding the link and password may read the file
    When an anonymous visitor downloads through the link
    Then the delivered bytes should carry the outgoing metadata policy
    And the download should be journaled and appear in Ana's file activity

  Scenario: Expired or revoked links stop working
    Given a public link that expired an hour ago
    When a visitor opens it
    Then the response should be 410
    When Ana revokes an active link and a visitor opens it
    Then the response should be 410

  Scenario: An internal link grants no new access
    When Ana copies the internal link of "brief.pdf" and sends it to Cara, who has no share
    And Cara opens it
    Then the response should be 403

  # Access listing and views
  Scenario: Who has access is listed with the share that grants it
    Given "brief.pdf" is shared with Ben as viewer and with "Design" as editor
    When Ana lists who has access
    Then the list should name Ben through his direct share and Cara through "Design"

  Scenario: The shares view separates with me, by me and by link
    Given "brief.pdf" is shared with Ben as viewer and a public link exists for "logo.png"
    When Ben opens the shares view
    Then "brief.pdf" should appear under "shared with me"
    When Ana opens the shares view
    Then "brief.pdf" should appear under "shared by me" and "logo.png" under "by link"

  # Ownership transfer
  Scenario: Ownership transfer is offered, accepted and journaled
    When Ana offers ownership of "brief.pdf" to Ben
    Then the file should be in a pending-transfer state findable with "pendingowner:me" by Ben
    When Ben accepts
    Then Ben should own "brief.pdf" and both steps should be journaled
    And the file's bytes should not have left their collection's storage location without a verified copy

  # Administrator switches
  Scenario Outline: An administrator can switch a sharing capability off per server
    Given the administrator switches "<capability>" off
    When Ana opens the Sharing panel for "brief.pdf"
    Then the "<capability>" control should be hidden
    And the panel should say the administrator disabled it
    And the switch change should be journaled

    Examples:
      | capability   |
      | sharing      |
      | public links |
      | re-sharing   |
      | download     |

  Scenario: Billing administration grants no share or read access
    Given Dana holds the billing administrator role only
    When Dana requests "brief.pdf" or tries to create a share on it
    Then both responses should be 403

  # Encrypted hosts
  Scenario: Shares on encrypted hosts use recipient keys without re-share or a master key
    Given an encrypted library shared with the group "Design"
    When a member rotates their device keys
    Then the member should keep access without Ana re-sharing
    And no server master key should exist

  # Default UI
  Scenario: The Sharing panel is independent and badges follow the browser
    Given "brief.pdf" is shared with Ben as viewer and a public link exists for "logo.png"
    When I mount the Sharing panel alone in a sidebar slot
    Then it should render internal shares, others with access, the internal link and public-link creation without any other panel present
    When I return to the file browser
    Then "brief.pdf" should show a shared badge and "logo.png" a link badge

  # Source
  # - resources/prd.md F-066
  # - resources/srs.md SRS-F-066
  # - apps/filesquid/crates/server/resources/srs.md SV-S-005, SV-S-006
  # - apps/filesquid/ui/resources/srs.md UI-S-010
