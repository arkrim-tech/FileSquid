# CORE-S-003: This crate MUST NOT link Axum, Tauri, libvips, or OCFL disk I/O.
# SRS-TC-005: Shared Rust graph, hosted mailbox isolation and OCFL writer
Feature: Portable contracts isolation
  As a developer
  I want filesquid-core free of host and codec crates
  So that Axum, Tauri, and WASM can share contracts without pulling native I/O

  Scenario: Core crate does not link forbidden hosts or codecs
    When I inspect the core crate library dependencies
    Then Axum, Tauri, libvips, FFmpeg, and OCFL I/O should be absent
    And axum, tauri, libvips, ffmpeg-sys, filesquid-ocfl, filesquid_ocfl, imagesquid-ocfl, imagesquid_ocfl, filesquid-metadata, and filesquid_metadata should be absent from library dependencies

  Scenario: Public items require rustdoc
    When I inspect the core crate root attributes
    Then missing rustdoc should be denied

  # Source
  # - apps/filesquid/crates/core/resources/srs.md CORE-S-003: This crate MUST NOT link Axum, Tauri, libvips, or OCFL disk I/O
  # - resources/srs.md SRS-TC-005: Shared Rust graph, hosted mailbox isolation and OCFL writer
  # - resources/prd.md TC-005: Shared Rust graph, hosted mailbox isolation and OCFL writer
