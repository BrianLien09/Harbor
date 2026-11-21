import { site } from "@/data/site";

// GitHub Pages serves the site under /Harbor, so every root-relative link needs the base prefix.
const base = import.meta.env.BASE_URL.replace(/\/$/, "");

/** Site-internal path with the base prefix, for href and src. */
export const withBase = (path: string): string => `${base}${path}`;

/** Full URL for canonical links and structured data. `site.url` already ends with the base. */
export const absoluteUrl = (path: string): string => `${site.url}${path === "/" ? "/" : path}`;
