import { site, upstream } from "@/data/site";
import { withBase } from "@/lib/url";

export const navigation = [
  { label: "Dock", href: withBase("/#dock") },
  { label: "AI 用量", href: withBase("/#ai-usage") },
  { label: "吉祥物", href: withBase("/#crab") },
  { label: "設定", href: withBase("/#customize") },
  { label: "可還原", href: withBase("/#safety") },
  { label: "更新紀錄", href: withBase("/changelog") },
] as const;

export const footerGroups = [
  {
    title: "產品",
    links: [
      { label: "Dock", href: withBase("/#dock"), external: false },
      { label: "AI 用量", href: withBase("/#ai-usage"), external: false },
      { label: "吉祥物", href: withBase("/#crab"), external: false },
      { label: "設定", href: withBase("/#customize"), external: false },
      { label: "可還原", href: withBase("/#safety"), external: false },
      { label: "下載", href: withBase("/#download"), external: false },
    ],
  },
  {
    title: "原始碼",
    links: [
      { label: "GitHub", href: site.repository, external: true },
      { label: "Releases", href: site.releases, external: true },
      { label: "問題回報", href: site.issues, external: true },
      { label: "上游 Muelle", href: upstream.repository, external: true },
    ],
  },
  {
    title: "專案",
    links: [
      { label: "授權", href: `${site.repository}/blob/main/LICENSE`, external: true },
      { label: "更新紀錄", href: withBase("/changelog"), external: false },
      { label: "關於", href: withBase("/about"), external: false },
      { label: "參與貢獻", href: site.issues, external: true },
      { label: "隱私權", href: withBase("/privacy"), external: false },
    ],
  },
] as const;
