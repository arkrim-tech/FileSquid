# FileSquid Photo Migration Playbook

**Status:** Draft user guide and engine requirements

This playbook explains how to move a photo and video collection out of a hosting platform and into a FileSquid library, and what FileSquid's migration engine guarantees while it does so. The first half is written for the person doing the migration. The second half records the engine requirements (`MIG-###`) and the source routes (`SRC-##`) that the ImageSquid distribution presents.

Platform export tools change their menus and formats without notice. Treat the steps below as the shape of each route and confirm the current path on the platform's own help pages before a large migration.

## What a migration promises

1. Copy supported media and collection information without modifying the source.
2. Preserve verified progress across interruptions and restarts.
3. Preserve source-provided originals, available metadata and relationships.
4. Explain unsupported information and transformations.
5. Account for every inventoried item in a completion report.

Migration completion is distinct from feature parity. An original and an edited rendition can both be preserved even when the source application's editable adjustment history cannot be reproduced.

**Out of scope**

- Continuous two-way synchronization with any platform.
- Automatic deletion of source libraries or cancellation of subscriptions.
- Reproduction of proprietary editing, recognition, sharing or memory-generation features.
- Repairing a corrupt original when no valid alternate copy exists.

## How a migration runs

Every route goes through the same seven steps. FileSquid shows progress for each step separately.

| Step | What happens | What you do |
|---|---|---|
| 1. Inspect and estimate | FileSquid identifies the source, destination and scope, and estimates items, bytes, temporary storage and cloud downloads. | Check free space, authorization and missing archives. Choose whether sensitive collections are included. |
| 2. Persist inventory | A durable manifest records every expected item, its identifiers, metadata references and checkpoints. | Nothing. The manifest is what lets a migration resume after a crash. |
| 3. Pilot | A representative sample is imported: ordinary images, large videos, edits, paired media, RAW, older dates, Unicode names. | Review preservation and appearance before committing to the full transfer. |
| 4. Bounded transfer | Extract, transfer, verify, register in the catalog, release temporary data. Concurrency and staging are capped. | Leave the computer on, or let a server job run. Pause and resume at will. |
| 5. Reconstruct collections | Sidecars, paired resources and album memberships are resolved as their targets arrive. | Nothing. |
| 6. Reconcile | Every expected item and relationship is accounted for. A live connector can run a follow-up scan. | Export again later if the source has changed since the snapshot. |
| 7. Review and finish | Verified counts and bytes, exclusions, unresolved issues and unsupported metadata, with a retry-unresolved action. | Confirm originals open offline in the new library. Clean up the source only after an independent backup. |

Source deletions never delete destination copies. Source cleanup is a separate, user-controlled step after a successful migration and backup.

## What is preserved

| Information | Treatment |
|---|---|
| Originals | Exact source-provided bytes. Quality lost before export cannot be restored. |
| Edited appearance | A separate rendition linked to its original. |
| Editing history | Available opaque adjustment data is retained without any compatibility promise. |
| Live and motion photos, RAW and JPEG pairs | Components and their associations are kept together, not split into separate logical photos. |
| Dates and locations | Available values, time zones and provenance are kept. Uncertain fallbacks are flagged. |
| Titles, descriptions, keywords | Embedded and sidecar values are mapped with documented precedence rules. |
| Albums and folders | Names, hierarchy, membership and ordering where the export provides them. |
| Favorites, hidden and archived states | Mapped only where reliable, with exclusions explained. |
| People and pets | Supplied names and associations are imported where supported; otherwise recognition is regenerated. |
| Smart albums and saved searches | Supported rules are recreated, or a membership snapshot is kept. |
| Memories and creations | Exported media is preserved; dynamic experiences are not recreated. |
| Sharing and comments | Descriptive data is preserved; destination sharing needs new permissions. |
| Unknown sidecars | Retained unchanged for later interpretation. |

Storage deduplication and logical identity are separate. Identical bytes can share storage without merging distinct source records or losing album membership.

## Platform guides

Each guide covers where the media lives, how to get an export, what the export contains and what FileSquid does with it.

### Apple Photos and iCloud Photos

**Where the media is.** In the Photos library on a Mac, in iCloud Photos, or both. A library set to "Optimize Mac Storage" holds reduced-size copies locally and keeps the originals in iCloud.

**Export routes.**

- *Export from the Photos app.* Select items or albums, then use File > Export > Export Unmodified Original. Enable IPTC-as-XMP sidecars and Live Photo video components. Export edited versions separately with File > Export > Export Photos. Exporting a whole library in batches is slow but gives you exact originals plus sidecars.
- *Native connector.* On a Mac, FileSquid can inventory the Photos library through the system photo framework, with your permission, and request originals in bounded batches. Cloud-only originals are downloaded as needed. This route sees albums, favorites and keywords that a flat folder export loses.
- *Download a copy from Apple.* Apple's privacy portal can deliver a copy of your iCloud Photos as a set of ZIP archives, and can transfer a copy directly to a few other services. Delivery takes days and the archives arrive in parts.

**What the export contains.** HEIC, JPEG, RAW and video originals; Live Photos as a still plus a video file; optional XMP sidecars with titles, keywords and descriptions; edited versions as separate files. Albums survive only through the native connector or by exporting album by album.

**FileSquid handling.** Sidecars are matched to their originals, Live Photo pairs are kept together, and edited exports are linked to their originals as renditions. A flat export folder is never assumed to be one album.

### Google Photos

**Where the media is.** In the Google Photos library. Items shared through partner sharing or shared albums may belong to another account.

**Export routes.**

- *Google Takeout.* Choose Google Photos, optionally pick albums, and choose an archive format and part size. Large libraries arrive as many archive parts. This is the whole-library route.
- *Selected items.* The Google Photos picker lets you hand FileSquid a chosen set of items. It is a convenience for small imports, not a library export.
- *Service-to-service transfer.* Google can transfer a copy of your photos to a few named services. Transfers exclude some metadata and unsaved creations, and FileSquid is not an eligible destination.

**What the export contains.** Media files organised into album folders and year folders, each with a JSON sidecar carrying the capture time, description, location and people labels. Edited versions carry an `-edited` suffix. Long file names are truncated, so a sidecar does not always share its photo's exact name. Some originals have had metadata stripped on upload.

**FileSquid handling.** All archive parts are treated as one source. Sidecars are matched by documented rules and flagged when ambiguous. The sidecar capture time is preferred over the archive download time. Raw sidecars are kept.

### Microsoft OneDrive and Windows Photos

**Where the media is.** Windows Photos displays local folders, OneDrive folders and iCloud folders together. Find where the bytes live before choosing a route.

**Export routes.**

- *Local folders.* Point FileSquid at the folders. Folder structure and embedded metadata are preserved.
- *Synchronized OneDrive folders.* Files On-Demand shows placeholders for files that are not on disk. FileSquid detects placeholders and fetches the real bytes, then copies into its own storage so the new library does not depend on the synchronized folder.
- *Direct OneDrive connector.* An optional, user-authorized connection to OneDrive that records resource identities and versions and supports validated resumable downloads.

**FileSquid handling.** A file visible in File Explorer is not proof that it is local. Application-only albums and favorites are not inside the files and are imported separately where available. The same photo reachable through OneDrive and iCloud is imported once.

### Flickr

**Where the media is.** In your Flickr photostream and albums.

**Export route.** From account settings, request your Flickr data. Flickr prepares two kinds of archives: account data (JSON files describing albums, tags, descriptions, locations, comments and favorites) and media archives containing the original uploads.

**What the export contains.** Original-resolution files, one JSON document per photo with title, description, tags, geolocation, privacy and dates, and an albums document listing membership and order.

**FileSquid handling.** Photo JSON is matched to media by the photo identifier embedded in the file name. Albums are rebuilt from the albums document. Comments are kept as descriptive data.

### Instagram and Threads

**Where the media is.** In your posts, stories, reels and archived items. Instagram and Threads share one export tool through Accounts Center.

**Export route.** In Accounts Center, open your information and permissions and request a download of your information. Choose the profiles, select media types, pick JSON format and the highest media quality, and set the date range. The archive is delivered as one or more downloads.

**What the export contains.** Media re-encoded by the platform, not your original camera files, with a JSON index that gives each post's timestamp, caption and media list.

**FileSquid handling.** Each post becomes a file or a group of files with the caption and timestamp as descriptive metadata. The export is flagged as platform-re-encoded so the limit is visible in the completion report.

### Facebook

**Where the media is.** In your photos, albums and posts.

**Export routes.**

- *Download your information.* The same Accounts Center tool as Instagram. Select photos and videos, choose JSON format and the highest quality.
- *Transfer a copy of your photos.* Facebook can send a copy of your photos and videos to a few named destinations. FileSquid is not one of them, so a transfer to a service you already use may still be a useful first step before exporting from there.

**What the export contains.** One folder per album with re-encoded media and a JSON document giving descriptions and creation timestamps.

**FileSquid handling.** Albums are rebuilt from the folder layout and JSON. Media is flagged as platform-re-encoded.

### X (formerly Twitter)

**Where the media is.** Attached to your posts.

**Export route.** From account settings, request an archive of your data. The archive is prepared over a day or more and delivered as a ZIP.

**What the export contains.** A browsable HTML viewer, a data folder with posts as JavaScript-wrapped JSON, and a media folder of images and videos named after the post identifier. Media is re-encoded and reduced in resolution.

**FileSquid handling.** Media is matched to posts by identifier, and the post text and timestamp become descriptive metadata. The export is flagged as platform-re-encoded.

### Bluesky

**Where the media is.** In your repository on your personal data server, with images stored as separate blobs.

**Export route.** From account settings, export your data. Bluesky delivers your repository as a CAR file that contains your posts and records but not the image bytes. FileSquid's AT Protocol connection can fetch the blobs your repository references, with your authorization.

**What the export contains.** Posts with timestamps, text, alt text and image references. Images retrieved through the connection are the platform's resized uploads, not your originals.

**FileSquid handling.** Images are matched to posts by content identifier, alt text becomes a description, and the export is flagged as platform-re-encoded.

### Other platforms

| Platform | Export route | What you get |
|---|---|---|
| Dropbox | Download folders from the web app or copy from the synchronized folder | Your original files and folder structure |
| Amazon Photos | Download from the web or desktop app, by folder or album | Original files; albums only as download groupings |
| SmugMug | Per-gallery original download, or a full account download request | Original files grouped by gallery |
| Snapchat | Request your data from account settings, including Memories | Re-encoded media with timestamps |
| TikTok | Request your data from account settings | Post metadata as JSON; videos as links or files depending on the option chosen |
| Pinterest | Request your data from privacy settings | Pin metadata; images referenced by URL |
| Mastodon | Request your archive from import and export settings | Posts as ActivityPub JSON with attached media files |
| Messaging apps (WhatsApp, Signal, Telegram) | Export a chat with media, or copy the media folder | Re-encoded media with chat timestamps |

Platforms that deliver originals (Dropbox, Amazon Photos, SmugMug) are treated like local folders. Platforms that deliver re-encoded media are flagged in the completion report.

## Source route register

| ID | Route | Delivers originals |
|---|---|---|
| SRC-01 | Apple Photos export folders | Yes |
| SRC-02 | Native Mac Photos connector | Yes |
| SRC-03 | Google Takeout archives | Yes, where metadata was not stripped on upload |
| SRC-04 | Extracted Google Takeout folders | Same as SRC-03 |
| SRC-05 | Google Photos selected-item import (optional) | Yes |
| SRC-06 | Supported provider-to-provider transfer into another route (optional) | Depends on the destination route |
| SRC-07 | Windows local collections displayed by Microsoft Photos | Yes |
| SRC-08 | Synchronized OneDrive collections with placeholder detection | Yes |
| SRC-09 | Direct OneDrive connector (optional) | Yes |
| SRC-10 | Flickr data archives | Yes |
| SRC-11 | Instagram and Threads downloads | No, platform re-encoded |
| SRC-12 | Facebook downloads | No, platform re-encoded |
| SRC-13 | X data archives | No, platform re-encoded |
| SRC-14 | Bluesky repository export plus blob retrieval through the AT Protocol connection | No, platform re-encoded |
| SRC-15 | Generic folder and archive import (Dropbox, Amazon Photos, SmugMug, messaging exports) | Yes for original-delivering sources |
| SRC-16 | Generic platform data archives (Snapchat, TikTok, Pinterest, Mastodon) | No, platform re-encoded or referenced by URL |

SRC-01 to SRC-09 are the routes the PRD's supported-migration-routes requirement specifies. SRC-10 to SRC-16 are candidate routes only: no requirement, acceptance criteria or plan task covers them yet, and they MUST NOT be described as supported until one does.

Every connector publishes a capability matrix per supported source or export version. Interfaces, permissions and export formats are revalidated when a route is implemented.

## When something goes wrong

| Failure | FileSquid response |
|---|---|
| Network loss or restart | Checkpoints are kept; verified remaining work resumes. |
| Expired access | Access is renewed, then the source is validated before continuing. |
| Full disk | The job pauses safely and explains the capacity needed. |
| Missing drive | The job waits for the correct volume or source to return. |
| Rate limiting | Back off and reduce concurrency. |
| Checksum mismatch | Retry from a valid source; quarantine persistent failures. |
| Missing archive part | Completed work is preserved and the missing input is identified. |
| Source changed or deleted | A new version or unavailability is recorded; the destination copy is kept. |

## Platform responsibilities

| Platform | Role |
|---|---|
| Mac desktop | Native Apple connector, folders and archives, long-running transfer |
| Windows and Linux desktop | Folders and archives, OneDrive routes, long-running transfer |
| Desktop browser | Selected uploads, resumable upload where supported, server-job control and review |
| Mobile web | Monitoring, issue review and small selected imports |
| Server workers | Authorized transfers and ciphertext-compatible verification independent of browser lifetime |

On the desktop, closing the window preserves checkpoints so work resumes at the next launch; it continues in the background only when the user has enabled that setting. A browser does not run work after it is closed; resumable state is preserved and source access is reacquired when it reopens. A server job continues on its own.

## Engine requirements

These requirements apply to every route. Numerical performance targets are selected before implementation and are acceptance criteria, not performance claims.

### Dependencies

| ID | Requirement |
|---|---|
| MIG-001 | Stable identities for catalog items, resources, albums, sources and migration jobs. |
| MIG-002 | Immutable original storage, with separate linked renditions and associated resources. |
| MIG-003 | A durable catalog supporting repeatable imports without unintended duplicates. |
| MIG-004 | Metadata storage retaining source values, provenance and mapping decisions. |
| MIG-005 | Album membership independent of physical file location. |
| MIG-006 | Background execution appropriate to desktop or server environments. |
| MIG-007 | Defined handling for unsupported media and metadata. |

Source connectors MUST be replaceable without changing the transfer engine or the catalog identity model.

### Functional

| ID | Requirement |
|---|---|
| MIG-008 | Select source, destination and scope; distinguish source libraries, accounts and exports. |
| MIG-009 | Estimate logical items, physical resources, bytes and storage needs where discoverable. |
| MIG-010 | Identify missing inputs, cloud-only assets and connector limitations. |
| MIG-011 | Offer explicit sensitive-collection inclusion and a representative pilot import. |
| MIG-012 | Persist a manifest and job state before substantial transfer. |
| MIG-013 | Retain identities, roles, source versions, metadata references, checkpoints and errors. |
| MIG-014 | Resume existing jobs and preserve state across application and computer restart. |
| MIG-015 | Document fallback identity matching and expose ambiguous matches. |
| MIG-016 | Preserve exact original bytes, linked renditions and supported paired resources. |
| MIG-017 | Preserve metadata provenance, uncertain dates and raw sidecars. |
| MIG-018 | Report unsupported media and fields without silently discarding them. |
| MIG-019 | Reconstruct supported hierarchy, membership, ordering and organizational states. |
| MIG-020 | Resolve relationships across archives and separate deduplication from logical identity. |
| MIG-021 | Require new sharing authorization; never inherit source access automatically. |
| MIG-022 | Recover independently at item and resource level and skip verified work. |
| MIG-023 | Resume within files where supported; otherwise restart only the affected processing unit. |
| MIG-024 | Persist confirmed offsets and parts and reconcile them with destination state. |
| MIG-025 | Support pause, resume, cancellation and bounded retries with backoff. |
| MIG-026 | Pause safely for missing drives, exhausted space or required authorization. |
| MIG-027 | Validate source identity after renewed links; never combine incompatible versions. |
| MIG-028 | Retry only unresolved items on request. |
| MIG-029 | Hash original resources and verify destination integrity before marking complete. |
| MIG-030 | Use suitable source or chunk checksums; never assume an ETag is a file hash. |
| MIG-031 | Reconcile crashes between media writes and catalog updates. |
| MIG-032 | Quarantine repeated failures and preserve actionable diagnostics. |
| MIG-033 | Reconcile expected content and relationships; support follow-up scans where available. |
| MIG-034 | Show discovery, extraction, transfer, verification and reconstruction progress separately. |
| MIG-035 | Display estimates honestly and revise them as discovery proceeds. |
| MIG-036 | Produce readable and machine-readable reports with an outcome for every item. |
| MIG-037 | Distinguish full success from completion with unresolved issues. |
| MIG-038 | Keep source media untouched; document temporary-data retention after cancellation. |
| MIG-039 | Identify local, synchronized-cloud and iCloud origins; prevent duplicate cross-route imports. |
| MIG-040 | Detect placeholders and verify destination bytes are independently available offline. |
| MIG-041 | Preserve folders and embedded metadata; validate application-only metadata separately. |
| MIG-042 | Establish destination independence from provider-synchronized folders. |
| MIG-055 | Mark every item from a platform that re-encodes uploads as platform-re-encoded in its provenance and in the completion report. |
| MIG-056 | Treat a platform data archive and its separately retrieved media (for example a Bluesky repository and its blobs) as one source with one inventory. |

### Performance

| ID | Requirement and measurement |
|---|---|
| MIG-043 | Define supported logical items, resources, albums and total bytes. |
| MIG-044 | Define maximum individual file, video and archive size. |
| MIG-045 | Bound memory through paging and streaming; measure peak use. |
| MIG-046 | Enforce configurable staging capacity and safe near-full behaviour. |
| MIG-047 | Avoid a full extra local library copy where the route permits staged extraction. |
| MIG-048 | Measure restart-to-resume latency and unnecessarily repeated bytes. |
| MIG-049 | Measure sustained verified throughput under documented hardware and network conditions. |
| MIG-050 | Set UI responsiveness targets under maximum supported load. |
| MIG-051 | Bound concurrency and respond to storage and provider throttling. |
| MIG-052 | Apply backpressure to prevent unbounded queues and staging. |
| MIG-053 | Validate multi-day stability and recovery without resource growth. |
| MIG-054 | Benchmark large relationship and sidecar reconstruction. |

### Transfer integrity

- **Job and resource recovery:** durable completion records avoid restarting the collection.
- **Within-file recovery:** confirmed offsets or parts avoid repeating large files where supported.

| Path | Mechanism |
|---|---|
| Local to local | Temporary file, validated offsets, final hash and atomic rename where supported. |
| Desktop or browser to cloud | Resumable upload sessions (tus or object-store multipart) with persistent state. |
| Provider to importer | Validated HTTP ranges where supported; refreshed links must reference compatible source content. |
| Apple resource extraction | Completed-resource checkpoints; restart the interrupted resource if necessary. |

Resources are hashed with SHA-256 and verified before completion. Corrupted chunks are retried where supported and persistent mismatches quarantined. Filesystem or object-store writes and database commits are not one transaction, so startup reconciliation handles orphaned completed writes. Credentials and keys stay in secure storage, never in manifests. For encrypted destinations, content verification runs at an endpoint that holds the keys; plaintext is never uploaded just to enable server-side processing.

### Acceptance and testing

Failures are injected during extraction, transfer, verification and catalog registration: process termination, computer restart, network loss, expired credentials, missing drives, full disks, corrupted chunks, source mutation, missing archives, late sidecars, repeated imports, and name and time-zone edge cases. For each scenario:

- Completed, verified resources are retained.
- Retries produce no unintended duplicate catalog entries.
- Partial data is never reported as a verified original.
- Every unresolved item remains visible and retryable, or is explicitly unavailable.
- Destination originals are usable without the source account.
- Source media remains unchanged.

**Acceptance standard:** within the declared connector scope and benchmark envelope, a restart preserves verified progress, prevents unintended duplication and leaves every unresolved item accounted for.
