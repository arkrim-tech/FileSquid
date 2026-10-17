# docs/

This directory is reserved for **user and developer manuals**: published how-to guides, API/user documentation, and other material meant to be read as a product/site, including hosting with [GitHub Pages](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site) (typically this `docs/` folder as the publishing source).

Use documentation generators and site conventions here (for example Markdown that a static-site generator or GitHub Pages will render). Do **not** put product-engineering files here.

When user manuals are written, they MUST include an **AI Connections** page covering the standing disclosure, inspectable explanation, and per-job preview required by [PRD F-045](../resources/prd.md#f-045-byo-ai-connections-and-per-task-model-choice) and [FE-067](../resources/product-vision.md#ai-the-user-chooses).

| Kind of file | Where it belongs |
|---|---|
| User/developer manuals, GitHub Pages | `docs/` (this directory) |
| Product vision, PRD, SRS, architecture narrative, ADRs, templates, references, reports | [`resources/`](../resources/) |
| Agent task Plan / ToDos / Walkthrough | `apps/<module>/resources/tasks/` (product documentation: `resources/tasks/`) |

Agents MUST follow [`.agents/rules/docs-resources-split.md`](../.agents/rules/docs-resources-split.md).
