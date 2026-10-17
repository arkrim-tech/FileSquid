# SRS-UI-004: Preservation and migration
Feature: Preservation and migration
  As a migrating owner
  I want to see job progress
  So that the product matches the SRS

  Scenario: Migration shows progress and separate states
    Given a migration job is running
    When I open preservation
    Then I should see progress, a durable job checkpoint, an incoming buffer, and storage allocation
    And processing completion, media verification, and preservation protection should remain separate states

  Scenario: Snapshot is not a job checkpoint
    Given an OCFL snapshot exists
    When I inspect history
    Then snapshots and job checkpoints should be distinct

  Scenario: Interrupted job resumes
    Given a job is interrupted
    When I restart ImageSquid
    Then completed work should not be thrown away

  # Source
  # - srs.md SRS-UI-004
  # - prd.md F-025 F-042
