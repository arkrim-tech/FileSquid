/**
 * The default FileSquid distribution: mounts the shell with FileSquid's brand and the
 * internal vocabulary, talking to whichever host serves this page. `#/monitor` shows the
 * Monitor dashboard; on the server host it asks for the admin token once per session.
 */
import "../theme/tokens.css";
import labels from "../theme/labels.en.json";
import {
  bindMonitorRoute,
  createHostClient,
  defineElements,
  sessionTokenStore,
  type FsAppShell,
} from "../index";

defineElements();
const tokens = sessionTokenStore();
const client = createHostClient(window, () => tokens.get());
const shell = document.querySelector<FsAppShell>("fs-app-shell");
shell?.configure({
  distribution: {
    id: "filesquid",
    productName: "FileSquid",
    logo: { src: "/filesquid-logo.png", alt: "FileSquid logo" },
    labels,
  },
  client,
});
if (shell) bindMonitorRoute(shell, { labels, client, tokens });
