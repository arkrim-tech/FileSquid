/**
 * ImageSquid entry point: FileSquid tokens first, ImageSquid tokens on top, then the
 * shared shell configured as ImageSquid and connected to whichever FileSquid host serves it.
 * The FileSquid Monitor is reachable at `#/monitor` (interim entry until the More menu).
 */
import "@filesquid/ui/theme/tokens.css";
import "./theme/tokens.css";
import {
  bindMonitorRoute,
  createHostClient,
  defineElements,
  sessionTokenStore,
  type FsAppShell,
} from "@filesquid/ui";
import { imagesquid } from "./distribution";

defineElements();
const tokens = sessionTokenStore();
const client = createHostClient(window, () => tokens.get());
const shell = document.querySelector<FsAppShell>("fs-app-shell");
shell?.configure({ distribution: imagesquid, client });
if (shell) bindMonitorRoute(shell, { labels: imagesquid.labels, client, tokens });
