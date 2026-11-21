import { site } from "@/data/site";
import { withBase } from "@/lib/url";

export const navigation = [
  { label: "Dock", href: withBase("/#dock") },
  { label: "AI usage", href: withBase("/#ai-usage") },
  { label: "Mascot", href: withBase("/#crab") },
  { label: "Settings", href: withBase("/#customize") },
  { label: "Reversible", href: withBase("/#safety") },
] as const;

export const footerGroups = [
  {
    title: "Product",
    links: [
      { label: "The Dock", href: withBase("/#dock"), external: false },
      { label: "AI usage", href: withBase("/#ai-usage"), external: false },
      { label: "Mascot", href: withBase("/#crab"), external: false },
      { label: "Settings", href: withBase("/#customize"), external: false },
      { label: "Reversible", href: withBase("/#safety"), external: false },
      { label: "Download", href: withBase("/#download"), external: false },
    ],
  },
  {
    title: "Source",
    links: [
      { label: "GitHub", href: site.repository, external: true },
      { label: "Releases", href: site.releases, external: true },
      { label: "Issues", href: site.issues, external: true },
    ],
  },
  {
    title: "Project",
    links: [
      { label: "License", href: `${site.repository}/blob/main/LICENSE`, external: true },
      { label: "Changelog", href: withBase("/changelog"), external: false },
      { label: "About", href: withBase("/about"), external: false },
      { label: "Contribute", href: site.issues, external: true },
      { label: "Privacy", href: withBase("/privacy"), external: false },
    ],
  },
] as const;
