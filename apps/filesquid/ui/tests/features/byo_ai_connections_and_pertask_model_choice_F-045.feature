# SRS-F-045: BYO AI connections and per-task model choice
Feature: BYO AI connections and per-task model choice
  As a library owner
  I want byo ai connections and per-task model choice
  So that the product matches the SRS

  Scenario: Happy path for F-045
    Given the feature is enabled on a supported profile
    When I use it as specified in the PRD
    Then the MUST criteria for that feature should hold

  Scenario: Limitation is visible for F-045
    Given a platform cannot provide a capability
    When I view that capability
    Then I should see an explicit unsupported or waiting state

  Scenario: Failure does not corrupt originals for F-045
    Given an operation fails
    When I inspect the library
    Then originals and committed OCFL snapshots should remain

  Scenario: Standing disclosure on AI Connections
    Given I open the AI Connections configuration surface
    Then I should see standing disclosure that FileSquid sends only approved files and text to the configured endpoint
    And I should see that I pay that service
    And I should see that encrypted backup does not see this traffic
    And I should see that a local failure does not send my library elsewhere

  Scenario: Inspectable How AI Connections work explanation
    Given I open the AI Connections configuration surface
    When I open the inspectable explanation titled How AI Connections work
    Then I should see that I connect an OpenAI-compatible endpoint I already run or pay
    And I should see that FileSquid does not operate that server and does not sell inference
    And I should see that a remote endpoint can read what I send
    And I should see that turning a connection off does not erase copies already made there

  Scenario: Per-job overlay for a newly authorized remote scope
    Given I have configured a remote AI connection
    When I start a newly authorized remote task
    Then I should see destination identifier, any extra hop, file count, image size, metadata policy, and payer
    And I should be able to Confirm or Cancel

  # Source
  # - srs.md SRS-F-045
  # - prd.md F-045
  # - product-vision.md FE-067
