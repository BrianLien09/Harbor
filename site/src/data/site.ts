export const site = {
  name: "Harbor",
  tagline: "macOS 風格的 Dock，為 Windows 打造。",
  description:
    "免費、開源的 Windows 10／11 macOS 風格 Dock。Fork 自 Muelle 並完成繁體中文在地化，提供神燈動畫、視窗預覽、AI 用量與選用的配件電量指示。",
  url: "https://brianlien09.github.io/Harbor",
  ogImage: "/og-image.png",
  ogImageWidth: 1200,
  ogImageHeight: 630,
  locale: "zh_TW",
  /** <html lang> 與 JSON-LD 的語系；全站文案為繁體中文（台灣）。 */
  lang: "zh-Hant-TW",
  repository: "https://github.com/BrianLien09/Harbor",
  releases: "https://github.com/BrianLien09/Harbor/releases",
  releasesApi: "https://api.github.com/repos/BrianLien09/Harbor/releases",
  download: "https://github.com/BrianLien09/Harbor/releases/latest/download/Harbor-Setup.exe",
  issues: "https://github.com/BrianLien09/Harbor/issues",
  version: "0.1.21",
  license: "MIT",
} as const;

/** The person who maintains this fork. */
export const author = {
  name: "BrianLien09",
  shortName: "BrianLien09",
  role: "Fork 維護者",
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
