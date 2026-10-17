# EU AI Act — FileSquid publication and disclosure analysis

**Status:** Adopted as the source for product-vision U-19–U-24. Not implemented software or legal advice. **Prepared:** 2026-09-19. Revised 2026-10-15: names updated to the FileSquid-first model ([ADR 029](../architecture/029-filesquid-unified-product-and-distributions.md)).

Unlike the US FRONTIER Act (H.R. 9925, a proposed federal regime for large frontier-model labs, not a content-authenticity or photo-app statute), the EU AI Act **does** create a disclosure duty that maps onto FileSquid’s existing paid publication work — and that is the only change that passes the revenue test used for this analysis. Do not turn FileSquid into a high-risk biometric or GPAI-compliance product.

---

## What the EU AI Act is

**Regulation (EU) 2024/1689** (the Artificial Intelligence Act) is already law. It entered into force on 1 August 2024. The transparency rules that matter for photos and video — **Article 50** — became applicable on **2 August 2026**. They were **not** postponed by the Digital Omnibus on AI (Regulation (EU) 2026/1744). A grace period applies only to **Article 50(2) machine-readable marking** for generative systems already on the market before 2 August 2026: those providers have until **2 December 2026**. Content generated before 2 August 2026 need not be labelled retroactively.

It is a risk-based statute for **providers** (who place an AI system on the market or put it into service under their own name, paid or free) and **deployers** (who use an AI system under their authority). It is **not** a lab-only catastrophic-risk bill. It applies to EU-established operators and to **third-country** providers and deployers **if the output is used in the Union**.

Two exclusions are decisive for FileSquid:

- **Personal, non-professional use** is outside deployer obligations (Article 2(10); Article 3(4)). A household editing holiday photos is not a deployer.
- **Free and open-source** systems are generally out of scope — **except** high-risk systems, prohibited practices, and systems that fall under **Article 5 or Article 50** (Article 2(12)). Open-sourcing FileSquid does **not** waive AI-content marking or deepfake disclosure if FileSquid is a provider of an Article 50 system.

The Commission and the AI Board have confirmed a voluntary **Code of Practice on Transparency of AI-generated Content** as an adequate way to demonstrate Article 50(2), (4) and (5) compliance. The EU also published a set of **labelling icons**. Signatories get a predictable path; non-signatories must prove adequacy to national market-surveillance authorities.

---

## Specific requirements (especially disclosure)

Article 50 has four duties. They are **not** interchangeable.

| Duty | Who | What | Human-visible? |
|---|---|---|---|
| **50(1)** | Provider | If the system interacts directly with people, tell them it is AI, unless obvious to a reasonably well-informed person | Yes, at first interaction |
| **50(2)** | Provider of generative systems, including GPAI | Mark synthetic audio/image/video/text in a **machine-readable** format and make it **detectable** as AI-generated or manipulated; techniques must be effective, interoperable, robust and reliable as far as technically feasible | No. This is the watermark / signed-metadata / fingerprint layer |
| **50(3)** | Deployer of emotion recognition or **biometric categorisation** | Inform natural persons **exposed** to the system; GDPR still applies | Yes |
| **50(4)** | Deployer | If image/audio/video is a **deepfake**, disclose that it was artificially generated or manipulated. Separate rule for AI text published to inform the public on matters of public interest (unless human review + editorial responsibility) | **Yes.** Commission FAQ: visible or audible labels, understandable without special tools. **Machine-readable marks from 50(2) do not satisfy 50(4)** |

**Deepfake** (Article 3(60)): AI-generated or manipulated image, audio or video that resembles existing persons, objects, places, entities or events and would **falsely appear authentic or truthful**. Three cumulative tests: resemblance, an existing (or plausibly existing) subject, and a false appearance of authenticity in context. Movie VFX that the audience does not take as real is often out; a realistic synthetic CEO video is in.

**50(2) carve-out:** assistive **standard editing** that does not substantially alter the input or its semantics (crop, exposure, spell-check). Generative fill, person removal, face swap, and similar material changes stay in.

**50(4) carve-outs:** law-enforcement authorisation; for artistic/creative/satirical/fictional works, disclosure is limited to an appropriate notice that does not hamper enjoyment of the work.

**50(5):** information under 50(1)–(4) must be **clear, distinguishable, and accessible at first exposure**.

---

## Penalties for non-compliance of AI disclosure

Article 50 is listed **by name** in **Article 99(4)(g)** (as amended by the 2026 Omnibus).

| Infringement | Ceiling |
|---|---|
| Prohibited practices (Article 5) | **€35 million or 7%** of worldwide annual turnover, **whichever is higher** |
| **Article 50 transparency / disclosure**, plus listed provider/deployer/importer duties | **€15 million or 3%** of worldwide annual turnover, **whichever is higher** |
| Incorrect, incomplete or misleading information to authorities | **€7.5 million or 1%**, whichever is higher |

For **SMEs including start-ups**, Article 99(6) reverses the rule: the fine is the **lower** of the fixed amount and the percentage. Small mid-caps get the same “lower of” treatment for the 15M/3% and 7.5M/1% bands. Percentages are on **total worldwide** turnover of the undertaking, not EU turnover and not product revenue. Member States set national procedure; fines must still be effective, proportionate and dissuasive, taking gravity, duration, intent, size, cooperation and mitigation into account.

**Who is actually exposed:**

- A household user is not a deployer. Article 50(4) does not fine them for an unlabeled private album.
- A **photographer, agency, newsroom, campaign, museum, or brand** using AI to produce a realistic image of a real person or place for publication **is** a deployer. They face the 15M/3% band if they fail to put a **human-perceivable** disclosure on the content at first exposure.
- FileSquid, if it is an SME, would itself face the **lower** of 15M and 3% of its own (small) turnover for *its* Article 50 duties — not 3% of a customer’s turnover.
- The **customer’s** 15M/3% exposure is the commercial pressure. That is the demand side.

Enforcement of Article 50 is mainly **national market-surveillance authorities**. The AI Office’s role is limited (GPAI systems where the same entity provides model and system, or systems in designated VLOPs/VLOSEs).

The Commission FAQ states the same numbers: fines can reach **€15 million or 3%**, with proportionality for SMEs and SMCs.

---

## How this sits on FileSquid today

Adopted product (vision + decision memo):

- Free C2PA record from every import, including disclosable **AI actions** (F-043).
- Paid third-party **C2PA + provenance-linked watermark** on a selected publication derivative (F-044). Using it is optional; shipping the capability is required.
- Default sharing **scrubs metadata** (enabled, empty retain-list).
- BYO AI: user pays the model provider; FileSquid does not resell inference (F-045).
- Honest limit already written: do **not** market a signature as proof the scene is real, that the image is not AI-generated, or that publication satisfies every law.

| AI Act duty | FileSquid today | Gap |
|---|---|---|
| 50(2) machine-readable mark | C2PA + watermark on **paid** publication; free records may be unsigned/prepared | Labs/GPAI providers own 50(2) for their generators. FileSquid- or ImageSquid-branded generative fill (if shipped) could make FileSquid a **provider**. BYO does not automatically make FileSquid the GPAI provider |
| 50(4) visible deepfake label | **Not a first-class share/export control.** C2PA/watermark are not a visible label. Commission: 50(2) marks **do not** satisfy 50(4) | This is the product gap |
| Preserve incoming generator marks | Import C2PA ingredients are retained; **default scrubbing can strip them on share** | Professional EU publish may need to **keep** AI marks while still scrubbing GPS/camera serials |
| 50(1) “you are talking to AI” | Only if FileSquid ships a conversational agent as the interlocutor | Prompt-assisted memories/assistant: disclose if the interaction is with the model, not a human |
| 50(3) biometric categorisation | People/pets clustering. Household use is personal/non-professional. A studio tagging **clients** may need to inform exposed persons; GDPR already flagged | Legal/UX review, not a SKU |
| High-risk / prohibited biometrics | Private library organisation is not public-space remote ID. Emotion inference at work/school is prohibited | Do **not** add emotion recognition or untargeted face scraping |

FileSquid is **not** a GPAI provider and should stay that way. FRONTIER-style lab duties still do not apply.

---

## Revenue test: what to enhance, and what not to

Unlike FRONTIER, **one** Article 50 requirement can widen appeal and support a paid job that already exists.

### Keep and sharpen (passes the revenue test)

**1. A professional “publish with disclosure” profile on top of F-044.**  
EU deployers (photographers, agencies, newsrooms, campaigns, GLAM, advertisers) now need, at first public exposure:

1. a **visible or audible** AI-generated/manipulated label (50(4); EU icons are an available option), and  
2. a **machine-readable** mark they did not strip (50(2), usually from the generator, plus FileSquid’s C2PA/watermark if FileSquid is in the production chain).

That is almost the current paid offer — “prepare a selected publication output, apply the chosen disclosure policy, add a provenance-linked watermark, return a verifiable package” — plus one missing piece: **human-perceivable disclosure**, not only C2PA.

Who pays: the **deployer**, not the household. Their alternative is an unlabeled deepfake and a 15M/3% ceiling. FileSquid does not need to certify “AI Act compliant.” It needs a **repeatable export** that applies the selected policy, keeps AI marks, adds a visible label, and verifies the outgoing file. That is the same commercial pattern as paid watermark publication, with a clearer EU professional reason to buy it.

**2. Do not let default privacy scrubbing destroy AI marks.**  
Enabled scrubbing with an empty retain-list is correct for GPS and camera serials. It is the wrong default for C2PA manifests and 50(2) marks on a **professional publish** route. A named retain-class for provenance/AI marks, with verification on the actual outgoing bytes (already required for F-046), makes the paid job usable in the EU instead of silently non-compliant.

**3. Free pre-share scan as the funnel, not a paywall.**  
Collection C2PA scans already exist as free scope. Extending them to “this item has AI-action history and the selected share route has no visible disclosure” is a conversion path into F-044. Keep the scan free; charge for the publication package.

**4. Later GLAM.**  
AI-assisted restoration of collection items is a natural IIIF-3 / institutional case: visible disclosure + preserved originals + C2PA. Still gated on a named partner and funded delivery. Article 50 makes that conversation easier; it does not justify pulling institutional work ahead of consumer backup.

These four items **widen appeal** in the EU professional segment without a new company. They ride the already-adopted paid publication SKU. They do **not** require FileSquid-funded inference.

### Do not do (fails the revenue test)

| Idea | Why not |
|---|---|
| Sell “EU AI Act compliance” as a legal product | Vision already forbids claiming publication satisfies every law. A signature is not a conformity assessment |
| Become a GPAI provider / mark every BYO token | The lab owns 50(2) for its generator. FileSquid should preserve marks and help the deployer label, not take on Article 53/55 |
| High-risk biometric product (remote ID, emotion, sensitive categorisation) | Different regulatory class, delayed high-risk dates, GDPR collision, consumer-brand damage. People albums stay local organisation |
| Emotion recognition in Memories | Article 5 risk in work/education contexts; 50(3) if deployed professionally. No revenue case |
| Relabel the whole consumer app as an Article 50 system | Household use is out of deployer scope. Over-claiming creates duty without buyers |
| Rewrite the roadmap around AI Act before backup/recovery | Core paid offer remains encrypted preservation. Article 50 is a **publication-profile** enhancement, not a new foundation |

---

## Assessment

| Question | Answer |
|---|---|
| Does the AI Act apply to FileSquid if it serves EU users or EU-used outputs? | **Yes, in part** — as a possible **provider** of FileSquid-branded generative/manipulation features, and as a tool used by **professional deployers**. Not as a GPAI lab. Household users are not deployers |
| Does Article 50 disclosure carry real fines? | **Yes.** Article 99(4)(g): up to **€15 million or 3%** worldwide turnover (higher of the two for ordinary undertakings; **lower** of the two for SMEs). Visible deepfake labels cannot be replaced by C2PA alone |
| Does that open revenue or wider appeal? | **Yes, narrowly.** Professional EU publish is a better reason to buy F-044 than “nice provenance.” Visible labels + keep-AI-marks-on-export + verified package. Household backup conversion is still unproven |
| Should product-vision/roadmap be rewritten around the AI Act? | **No full rewrite.** Optional: standards-table watch item for Article 50 as a **publication/disclosure** use case; F-044/F-046: visible label + provenance-mark retain-class on professional share. Do not reorder Foundation → backup ahead of this |
| Compared with FRONTIER? | FRONTIER does not cover FileSquid. The AI Act does, on **output transparency**, and that is already FileSquid’s authenticity product |

**Practical sequence (if you want a later doc edit, not done here):** keep consumer ownership and paid encrypted preservation first; implement free AI-action C2PA history; ship a professional publication profile that (1) preserves generator marks, (2) applies a visible first-exposure disclosure, (3) adds the paid watermark/C2PA package, (4) verifies the outgoing file. Market it as a **disclosure and delivery workflow**, never as a legal compliance certificate.

I have not edited product-vision or the roadmap. If you want the standards table and F-044 wording updated to match this, that is a small, revenue-aligned change.