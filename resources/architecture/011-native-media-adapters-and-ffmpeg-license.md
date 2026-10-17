# Architecture Decision Record - FileSquid

011-native-media-adapters-and-ffmpeg-license.md

## Title

LibRaw, libheif and LGPL FFmpeg as desktop media adapters (C2PA moved to `filesquid-c2pa` by ADR 026)

## Status

Accepted

Accepted: 2026-09-16T22:24:57Z. Revised: 2026-09-22T04:34:37Z. Revised: 2026-10-15T05:24:52Z. Amended by [ADR 026](./026-filesquid-provenance-engine.md): C2PA moves out of the desktop media adapters into the from-scratch `filesquid-c2pa` client engine; the official `c2pa` crate is a CI oracle only.

## Context / Requirement Reference

[ADR 006](./006-libvips-native-stills-not-imagemagick.md) chose libvips and forbade ImageMagick, leaving RAW, HEIF, video and C2PA SDK as later ADRs. The vision listed codecs as undecided. Distribution of patented codecs and GPL FFmpeg would change the open-source license of the installer.

## Decision

Desktop native adapters, all out-of-process, all bundled in per-OS installers (not PATH):

| Need | Choice |
|---|---|
| Stills thumbnails/display | libvips ([ADR 006](./006-libvips-native-stills-not-imagemagick.md)) |
| RAW development | LibRaw (or a documented successor) behind an adapter |
| HEIF/HEIC | libheif where legally distributable; otherwise OS decoder + visible capability |
| Video/audio | Isolated **FFmpeg** worker |
| C2PA read/write/validate | `filesquid-c2pa` client engine, implemented from scratch ([ADR 026](./026-filesquid-provenance-engine.md)); the official [`c2pa`](https://crates.io/crates/c2pa) crate is an interoperability oracle in CI only |

**FFmpeg license for the default installer:** build and ship an **LGPL** FFmpeg **without** `--enable-gpl` and **without** `--enable-nonfree`. Do not bundle GPL encoders (including libx264) in that default. Prefer OS/hardware decoders/encoders where present. A GPL FFmpeg variant, if ever offered, MUST be a separately licensed artifact, not the default.

Patent-encumbered formats (HEVC, some AAC) are a **separate** distribution/patent problem from copyleft. Bundling GPL FFmpeg can change FileSquid’s **open-source license**. Shipping our own HEVC/AAC decoder can create **patent** obligations even when the FFmpeg build is LGPL. Prefer OS decoders for those formats. FileSquid MUST NOT claim a format is supported in a binary we cannot legally distribute. The FOSS checklist and the patent checklist are both required; one does not substitute for the other.

This ADR freezes the adapter names and the LGPL-default rule. Exact FFmpeg configure flags, the linked inventory, and FOSS/patent counsel review remain **future work** (below). The gate is **before a public desktop binary** or any “download FileSquid” distribution, not before the first internal prototype.

## Rationale

One stills engine, one RAW library, one video worker — all in Rust workers — and one C2PA engine (`filesquid-c2pa`, [ADR 026](./026-filesquid-provenance-engine.md)). LGPL default keeps the core app from silently becoming GPL.

## Assumptions

- Counsel will review the actual linked inventory before distribution.
- Browser viewing still uses prepared derivatives ([ADR 008](./008-browser-wasm-scope.md)); these native adapters are desktop.

## Alternatives Considered

- **GPL FFmpeg default:** Rejected for license contagion of the default installer.
- **GStreamer instead of FFmpeg:** Viable; rejected to keep one video worker stack.
- **No HEIF:** Would fail common camera imports; OS decoder remains the legal fallback.

## Consequences / Implications

Easier: named adapters. Harder: legal review of a real installer, per-OS HEIF, hardware-encoder matrix. Model/ONNX packs are **not** part of this stack ADR ([FE-049](../product-vision.md#ai-the-user-chooses) BYO).

**Future work — FOSS license and patent check (before a public desktop binary).** Counsel (open-source plus codec/patent for the places of distribution) MUST review the **actual linked inventory** of a candidate installer, not this ADR alone:

- Is this FFmpeg build really LGPL, including every enabled demuxer/decoder?
- Does bundling libheif, LibRaw, libvips plus FFmpeg force GPL or extra notices?
- Patent risk for HEVC/AAC if we ship our own decoder instead of the OS.
- The public component/license matrix [U-13](../product-vision.md#platform-ownership-and-access) and [U-38](../product-vision.md#one-api-hosts-and-distributions) require before calling the app open source ([standards and licensing](../product-vision.md#standards-laws-and-regulations)).

## Related Decisions / Notes

- [ADR 006](./006-libvips-native-stills-not-imagemagick.md), [ADR 015](./015-prov-o-jsonld-and-c2pa-sdk.md)
- [FFmpeg legal](https://www.ffmpeg.org/legal.html), [U-13](../product-vision.md#platform-ownership-and-access), [U-33](../product-vision.md#one-api-hosts-and-distributions)
