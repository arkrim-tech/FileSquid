/**
 * The ImageSquid distribution definition: brand and vocabulary passed to FileSquid
 * components. ImageSquid adds views, never backend code (ImageSquid U-06).
 */
import type { Distribution } from "@filesquid/ui";
import labels from "./theme/labels.en.json";

/** ImageSquid brand and photo-album vocabulary. */
export const imagesquid: Distribution = {
  id: "imagesquid",
  productName: "ImageSquid",
  logo: { src: "/imagesquid-logo.svg", alt: "ImageSquid logo" },
  labels,
};
