/**
 * A distribution is a UI layer over the FileSquid API: brand, vocabulary and views
 * (ADR 029, ADR 031). It contains no backend code.
 */
import type { LabelTable } from "./theme/labels";

/** Brand and vocabulary a distribution passes to FileSquid components. */
export interface Distribution {
  /** Stable identifier, for example "filesquid" or "imagesquid". */
  id: string;
  /** Product name shown in the shell header and window title. */
  productName: string;
  /** Logo image shown next to the product name. */
  logo: { src: string; alt: string };
  /** Visible nouns keyed by internal names. */
  labels: LabelTable;
}
