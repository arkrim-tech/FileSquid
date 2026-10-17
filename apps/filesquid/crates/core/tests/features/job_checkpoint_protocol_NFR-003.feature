# SRS-NFR-003: Bounded processing and migration endurance
# ADR 007: Durable jobs and interactive performance
Feature: Job checkpoint protocol
  As a host crate
  I want portable job checkpoint, lease, and receipt types
  So that workers resume from durable in-progress state without treating jobs as OCFL snapshots

  Scenario: A job checkpoint is not an OCFL collection snapshot
    Given an operation id "op-job-1"
    When I construct a job checkpoint
    Then the checkpoint operation id should be "op-job-1"
    And the checkpoint should not be an OCFL collection snapshot

  Scenario: Canonical receipt is durable completion evidence
    Given an operation id "op-job-1"
    When I construct a canonical receipt
    Then the receipt should be durable completion evidence
    And the receipt operation id should be "op-job-1"

  Scenario: Default transfer and CPU-heavy worker caps
    When I inspect job worker caps
    Then the default active transfers should be 4
    And the default CPU-heavy workers should be 2

  Scenario: Staging cap and destination reserve
    When I inspect staging bounds for a 100 GiB destination
    Then the default staging cap should be 10 GiB
    And the reserve should be 5 GiB

  Scenario: Reserve uses the 2 GiB floor on a small destination
    When I inspect staging bounds for a 10 GiB destination
    Then the reserve should be 2 GiB

  Scenario: Attempt cap and retry backoff
    When I inspect retry bounds
    Then the maximum attempts per resource should be 5
    And the retry backoff cap should be 60 seconds
    And the restart resumable bound should be 30 seconds
    And the migration process memory bound should be 2 GiB

  Scenario: An expired lease cannot commit after a replacement owns it
    Given an operation id "op-job-1"
    When a worker holds lease fencing token 1
    And a replacement takes lease fencing token 2
    Then the expired worker with token 1 should not be allowed to commit
    And the replacement with token 2 should be allowed to commit

  Scenario: Exceeding the transfer cap is rejected
    When I admit 5 active transfers
    Then admitting the transfer should fail

  # Source
  # - resources/srs.md SRS-NFR-003: Bounded processing and migration endurance
  # - resources/prd.md NFR-003: Bounded processing and migration endurance
  # - resources/architecture/007-durable-jobs-and-interactive-performance.md
