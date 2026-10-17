/**
 * DOM-free theme helpers (`@filesquid/ui/theme`): label and token contracts that
 * distributions check in their own tests without a browser.
 */
export { REQUIRED_LABEL_KEYS, missingLabels, type LabelKey, type LabelTable } from "./labels";
export { REQUIRED_TOKENS, missingTokens } from "./tokens";
