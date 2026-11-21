export const site = {
  name: "Harbor",
  tagline: "The macOS Dock, made for Windows.",
  description:
    "Free, open-source macOS-style dock for Windows 10 and 11. Genie animations, window previews, AI usage and optional accessory battery indicators.",
  url: "https://brianlien09.github.io/Harbor",
  ogImage: "/og-image.png",
  ogImageWidth: 1200,
  ogImageHeight: 630,
    locale: "en_US",
  repository: "https://github.com/BrianLien09/Harbor",
  releases: "https://github.com/BrianLien09/Harbor/releases",
  releasesApi: "https://api.github.com/repos/BrianLien09/Harbor/releases",
  download: "https://github.com/BrianLien09/Harbor/releases/latest/download/Harbor-Setup.exe",
  issues: "https://github.com/BrianLien09/Harbor/issues",
  version: "0.1.18",
  license: "MIT",
} as const;

/** The person who maintains this fork. */
export const author = {
  name: "BrianLien09",
  shortName: "BrianLien09",
  role: "Developer",
  id: "https://brianlien09.github.io/Harbor/#person",
  profile: "https://github.com/BrianLien09",
  github: "https://github.com/BrianLien09",
} as const;

/** Harbor is a fork of Muelle (MIT); the original authors are credited, never presented as the maintainers. */
export const upstream = {
  name: "Muelle",
  repository: "https://github.com/crafter-station/Muelle",
  organization: "Crafter Station",
  organizationUrl: "https://crafter.run",
  author: "Cristian Olivera Chávez",
  authorUrl: "https://github.com/CristianOlivera1",
} as const;
