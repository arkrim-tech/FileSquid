---
type: Reference
title: RFC 9562 — Universally Unique IDentifiers (UUIDs)
description: 128-bit UUID format, versions 1–8, variant bits, hex-and-dash text, urn:uuid, CSPRNG, and opacity rules. Obsoletes RFC 4122.
resource: https://www.rfc-editor.org/rfc/rfc9562.html
tags:
- rfc
- uuid
- identifiers
- standards
status: stable
generated:
  by: process:kb-synthesizer
  at: '2026-09-23T04:16:51Z'
sources:
- id: rfc9562
  resource: https://www.rfc-editor.org/rfc/rfc9562.html
  title: RFC 9562 Universally Unique IDentifiers (UUIDs)
  author: process:ietf
kb-title: RFC 9562 — Universally Unique IDentifiers (UUIDs)
source-url: https://www.rfc-editor.org/rfc/rfc9562.html
spec-version: '9562'
last-updated: '2026-09-23T04:16:51Z'
domain: Identifiers, Standards
---

# RFC 9562: Universally Unique IDentifiers (UUIDs)

**Purpose:** Agent-facing synthesis of [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html) (Standards Track, May 2024). Use this file for UUID layout, version/variant bits, text form, and generation constraints. Load the RFC via Progressive Disclosure for full field diagrams, IANA registries, and remaining test vectors.[^rfc9562]

**TL;DR:** A UUID is **128 bits**, no central registry. This document’s variant is **10xx**. Version is the high nibble of octet 6. Text form is **hex-and-dash** (`8-4-4-4-12`). URN is `urn:uuid:` plus that hex. **UUIDv4** is 122 random bits plus version/variant. Treat values as **opaque**. Use a **CSPRNG**. Do **not** treat possession of a UUID as a security capability. This RFC **obsoletes RFC 4122**.

FileSquid newly generated domain IDs use UUIDv4 per [ADR 021](../../resources/architecture/021-uuid-v4-identifiers.md).

---

## Novice Mental Model

| Everyday idea | RFC 9562 equivalent |
|---------------|---------------------|
| 16-byte unique ID | UUID / GUID — 128 bits |
| Which layout family | **Variant** — bits 64–65 of octet 8 |
| Which algorithm | **Version** — bits 48–51 of octet 6 |
| Random ID | **UUIDv4** — 122 random bits (Source: §5.4) |
| Time-sortable ID | **UUIDv7** Unix ms, or **UUIDv6** reordered Gregorian (Source: §5.6, §5.7) |
| ID from a name | **UUIDv5** SHA-1 (prefer over **UUIDv3** MD5) (Source: §5.3, §5.5) |
| Custom layout | **UUIDv8** — only version/variant are fixed (Source: §5.8) |
| Text in APIs | hex-and-dash string (Source: §4) |
| URI | `urn:uuid:<hex-and-dash>` (Source: §4, RFC 8141) |
| All zeros / all ones | Nil UUID / Max UUID (Source: §5.9, §5.10) |

**Key insight:** Version and variant bits **overwrite** random or hash bits after the 128-bit field is filled. Uniqueness is practical, not a shared global registry. (Source: §4.1, §4.2, §6.8)

---

## Status and Scope

- Internet Standards Track. Obsoletes RFC 4122. (Source: header, Abstract)
- Requirement words follow BCP 14 ([RFC 2119](./rfc-2119-req-terms.md), RFC 8174) only when they appear in all capitals. (Source: §3.1)
- Aligned with ITU-T X.667 / ISO/IEC 9834-8. Does not override DCE. (Source: §1)
- Generating a UUID and merely storing one are different jobs. (Source: §2.1)

---

## Format (Source: §4)

- Size: 16 octets (128 bits). Bit index 0–127; octet index 0–15.
- Default field encoding: **most significant byte first** (network / big-endian).
- Microsoft COM GUIDs may mix endianness when saved; that caveat is out of scope. (Source: §4)
- UUIDs MAY be binary or integers. For URNs and application text, use hex-and-dash.

### Text ABNF (Source: §4)

```
UUID     = 4hexOctet "-" 2hexOctet "-" 2hexOctet "-" 2hexOctet "-" 6hexOctet
hexOctet = HEXDIG HEXDIG
```

- Hex letters MAY be upper, lower, or mixed (RFC 5234 §2.3).
- Example: `f81d4fae-7dec-11d0-a765-00a0c91e6bf6`
- Same value as URN: `urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6`
- Curly-brace `{uuid}` forms exist in some implementations; they are informational, not this ABNF. (Source: §4)

### Variant (Source: §4.1)

MSB bits of octet 8:

| MSB0–MSB3 pattern | Hex nibble (high) | Meaning |
|---|---|---|
| `0xxx` | 0–7 | Reserved NCS; includes Nil UUID |
| `10xx` | 8, 9, A, B | **This document** |
| `110x` | C–D | Reserved Microsoft |
| `111x` | E–F | Reserved future; includes Max UUID |

For UUIDs in this document, bits 64 and 65 **MUST** be `1` and `0` (variant `10xx`).

### Version (Source: §4.2)

High 4 bits of octet 6 (bits 48–51):

| Ver | Layout |
|---:|---|
| 0 | Unused |
| 1 | Gregorian time + clock seq + node (often MAC) |
| 2 | DCE Security — **out of scope** |
| 3 | Name-based MD5 |
| 4 | Random / pseudorandom |
| 5 | Name-based SHA-1 |
| 6 | Reordered UUIDv1 timestamp (DB locality) |
| 7 | Unix Epoch milliseconds + random |
| 8 | Custom / experimental |
| 9–15 | Reserved |

UUIDv4 hex template: `xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx` with `M = 4` and `N` in `{8,9,A,B}`. (Source: §4.2 Figure 5)

---

## Version Layouts

### UUIDv1 (Source: §5.1)

60-bit timestamp: 100 ns ticks since 1582-10-15 00:00:00 UTC. 14-bit clock sequence. 48-bit node (IEEE 802 MAC or random).

- Clock set backwards or Node ID change: clock sequence **MUST** change. **MAY** increment if previous known; otherwise **SHOULD** be random.
- Clock sequence **MUST** be originally initialized to a random number. Initial value **MUST NOT** correlate to Node ID.
- No IEEE address: random/pseudorandom node **MUST** be used (§6.9, §6.10).
- Local time instead of UTC is allowed if consistent; **not recommended**.

### UUIDv2 (Source: §5.2)

DCE Security. Definition is outside this RFC.

### UUIDv3 (Source: §5.3)

MD5(Namespace ID ∥ canonical name octets). Overwrite version=`0011` and variant=`10`. Where possible, UUIDv5 **SHOULD** be used instead of UUIDv3.

### UUIDv4 (Source: §5.4)

Fill 128 bits from random data, then overwrite version=`0100` and variant=`10`. **MAY** generate exactly 122 random bits (`random_a` 48 + `random_b` 12 + `random_c` 62) and insert version/variant.

Randomness guidelines: §6.9.

Test vector (Source: Appendix A.3):

| | Value |
|---|---|
| 16 random bytes | `919108F752D133205BACF847DB4148A8` |
| After ver/var overwrite | `919108f7-52d1-4320-9bac-f847db4148a8` |

### UUIDv5 (Source: §5.5)

SHA-1(Namespace ID ∥ canonical name). Use leftmost 128 bits; discard remaining 32. Overwrite version=`0101` and variant=`10`. SHA-256 (or newer) name-based IDs **MUST NOT** use UUIDv5; they **MUST** be UUIDv8 (Source: §5.5, Appendix B.2).

### UUIDv6 (Source: §5.6)

UUIDv1 timestamp bytes reordered most-significant-first for index locality. Clock seq + node positions unchanged. Clock seq and node **SHOULD** be reset to pseudorandom per UUID; **MAY** keep UUIDv1 MAC behavior. Systems without UUIDv1 legacy **SHOULD** use UUIDv7.

### UUIDv7 (Source: §5.7)

48-bit big-endian Unix time in **milliseconds** (leap seconds excluded) + 74 bits excluding version/variant. Implementations **SHOULD** use UUIDv7 instead of UUIDv1 and UUIDv6 if possible.

Optional extra monotonicity within a millisecond, in this order (Source: §5.7, §6.2):

1. Sub-ms fraction (max 12 bits)
2. Seeded counter
3. Random fill of remaining bits

### UUIDv8 (Source: §5.8)

Only version=`1000` and variant=`10` are required. Remaining 122 bits are implementation-defined. Uniqueness is implementation-specific and **MUST NOT** be assumed. **Not** a substitute for UUIDv4.

### Nil and Max (Source: §5.9, §5.10)

| Form | Hex | Variant row |
|---|---|---|
| Nil | `00000000-0000-0000-0000-000000000000` | NCS `0xxx` |
| Max | `FFFFFFFF-FFFF-FFFF-FFFF-FFFFFFFFFFFF` | Future `111x` |

Nil = “no such value”. Max = sentinel such as “end of list”.

---

## Generation Best Practices

### Randomness (Source: §6.9)

Implementations **SHOULD** use a CSPRNG so values are unguessable and collision-resistant, except when no suitable CSPRNG exists in the environment. Reseed CSPRNG state on process forks. See RFC 4086 and RFC 8937.

### Opacity (Source: §6.12)

Treat UUIDs as opaque. Avoid parsing unless required (variant/version/timestamp). Simpler, more interoperable, often faster.

### Sorting (Source: §6.11)

UUIDv6 and UUIDv7 sort as opaque raw bytes. Text form is intended to be lexicographically sortable. Layouts assume big-endian; little-endian custom forms use UUIDv8.

### Host identity (Source: §6.10, §8)

Random 48-bit Node ID: **MUST** set the multicast bit (LSB of first Node ID octet) to 1 so it cannot collide with a real unicast MAC. MAC addresses **SHOULD NOT** be used in a UUID.

### Name-based rules (Source: §6.5)

Same namespace + same canonical name **MUST** yield the same UUID. Different names or namespaces should differ with very high probability. “Name” is a canonical octet sequence, not only text.

### Namespace IDs (Source: §6.6)

| Namespace | Namespace ID |
|---|---|
| DNS | `6ba7b810-9dad-11d1-80b4-00c04fd430c8` |
| URL | `6ba7b811-9dad-11d1-80b4-00c04fd430c8` |
| OID | `6ba7b812-9dad-11d1-80b4-00c04fd430c8` |
| X.500 | `6ba7b814-9dad-11d1-80b4-00c04fd430c8` |

`6ba7b813-9dad-11d1-80b4-00c04fd430c8` is **undefined** and **SHOULD NOT** be used. New global Namespace IDs **MUST** follow the increment-`time_low` logic and IANA §7. Custom Namespace IDs **MUST NOT** use that logic; UUIDv4 or UUIDv7 is **RECOMMENDED**.

### Uniqueness and collisions (Source: §6.7, §6.8)

Global uniqueness cannot be guaranteed without shared knowledge. Shared registries are **NOT RECOMMENDED**. Distributed generators **MUST** be willing to rely on each host’s random source. Weigh collision impact (log duplicate vs safety-critical key).

### Time fields (Source: §6.1)

- Non-Gregorian / non-Unix epochs **MUST** use UUIDv8.
- Pad timestamps on the **most significant** (left) bits. Truncate on the **least significant** (right) bits.
- Overrun of a time tick: **MUST NOT** knowingly return duplicates from counter rollover; stall or error.

### Monotonic counters (Source: §6.2)

For batch UUIDv7: a fixed-length counter **MUST** sit immediately after the timestamp. Check that each new UUID is greater than the previous when monotonicity is required.

### DBMS (Source: §6.13)

Where feasible, store the **128-bit binary** value (text is 288 bits). **SHOULD NOT** use name-based UUIDs as primary keys. Prefer time-based surrogate keys over name-based natural keys.

---

## Security (Source: §8)

- **SHOULD NOT** assume UUIDs are hard to guess.
- **MUST NOT** use UUIDs as security capabilities (possession = access).
- **MUST NOT** assume slight modification is easy to detect by inspection.
- MAC addresses **SHOULD NOT** appear in UUIDs; prefer CSPRNG node data.
- If a UUID is used with any security operation, UUIDv4 **SHOULD** be used.
- Embedded timestamps leak creation order, not application meaning.
- MD5: RFC 6151. SHA-1: RFC 6194.

---

## Agent Rules

1. When **minting** a UUID for this RFC’s variant: set version nibble and variant `10xx` (`N` ∈ `8,9,A,B`).
2. When **minting UUIDv4**: CSPRNG 122 random bits (or 128 then overwrite ver/var). Do not use `Math.random` / non-crypto PRNGs if a CSPRNG exists.
3. Canonical **exchange** form: hex-and-dash. Parsers **MUST** accept mixed case; project storage **MAY** normalize to lowercase.
4. OCFL / URN object ids: `urn:uuid:` + hex-and-dash.
5. Do not parse UUIDs for chronology unless the version is time-based (v1/v6/v7) and the task requires it.
6. Do not treat Nil UUID as a generated identity.
7. Do not invent a new version number; custom layouts are UUIDv8.

---

## Why RFC 9562 replaced RFC 4122 (Source: §2.1)

UUIDv4 has poor B-tree locality. UUIDv1 uses a rare 100 ns Gregorian epoch, needs introspection to sort, and can expose MAC addresses. RFC 4122 mixed generate vs store requirements and left errata. UUIDv6/v7/v8 and the best-practice section address those issues. FileSquid still selects **UUIDv4** for domain IDs because they must not encode capture or generation time ([ADR 021](../../resources/architecture/021-uuid-v4-identifiers.md)).

[^rfc9562]: RFC 9562 Universally Unique IDentifiers (UUIDs), May 2024
