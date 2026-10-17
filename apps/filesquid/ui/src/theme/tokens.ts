/**
 * Design tokens: components are styled only through these CSS custom properties.
 * A distribution MUST define every token for light and dark (ADR 031).
 */

/** Tokens every distribution MUST define. */
export const REQUIRED_TOKENS = [
  "--fs-surface",
  "--fs-surface-container",
  "--fs-on-surface",
  "--fs-on-surface-variant",
  "--fs-outline",
  "--fs-primary",
  "--fs-on-primary",
  "--fs-primary-container",
  "--fs-on-primary-container",
  "--fs-error",
  "--fs-font-sans",
  "--fs-font-mono",
  "--fs-radius",
] as const;

/** Remove CSS comments so commented-out declarations do not count as defined. */
function stripComments(css: string): string {
  return css.replace(/\/\*[\s\S]*?\*\//g, "");
}

/** Return the declared custom-property names in one CSS block body. */
function declared(block: string): Set<string> {
  return new Set([...block.matchAll(/(--fs-[a-z0-9-]+)\s*:/g)].map((m) => m[1] ?? ""));
}

/** Extract the bodies of the light (`:root`) and dark blocks of a token stylesheet. */
function blocks(css: string): { light: string; dark: string } {
  const light = [...css.matchAll(/:root\s*\{([^}]*)\}/g)].map((m) => m[1]).join("\n");
  const dark = [...css.matchAll(/\[data-theme="dark"\]\s*\{([^}]*)\}/g)]
    .map((m) => m[1])
    .join("\n");
  return { light, dark };
}

/**
 * Return required tokens missing from the light and dark blocks of a stylesheet.
 *
 * Light tokens live in `:root { … }`; dark tokens in `:root[data-theme="dark"] { … }`
 * (the same values are also applied for `prefers-color-scheme: dark`).
 *
 * @param css Token stylesheet text.
 */
export function missingTokens(css: string): { light: string[]; dark: string[] } {
  const { light, dark } = blocks(stripComments(css));
  const lightNames = declared(light);
  const darkNames = declared(dark);
  const colourOnly = (t: string) =>
    !["--fs-font-sans", "--fs-font-mono", "--fs-radius"].includes(t);
  return {
    light: REQUIRED_TOKENS.filter((t) => !lightNames.has(t)),
    dark: REQUIRED_TOKENS.filter(colourOnly).filter((t) => !darkNames.has(t)),
  };
}
