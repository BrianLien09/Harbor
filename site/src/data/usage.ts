import claude from "@/assets/usage/claude.svg";
import codex from "@/assets/usage/codex.svg";
import gemini from "@/assets/usage/gemini.svg";
import opencode from "@/assets/usage/opencode.svg";

export interface QuotaExample {
  label: string;
  used: number;
  resets: string;
}

/** One supported AI client with illustrative, not live, quota values. */
export interface UsageProvider {
  name: string;
  logo: ImageMetadata;
  color: string;
  primary: QuotaExample;
  secondary?: QuotaExample;
}

export const usageProviders: UsageProvider[] = [
  {
    name: "Claude Code",
    logo: claude,
    color: "#d97757",
    primary: { label: "5 h", used: 46, resets: "下午 3:10 重設" },
    secondary: { label: "7 天", used: 35, resets: "11/21 重設" },
  },
  {
    name: "Codex",
    logo: codex,
    color: "#87bef5",
    primary: { label: "5 h", used: 28, resets: "下午 4:30 重設" },
    secondary: { label: "7 天", used: 41, resets: "11/21 重設" },
  },
  {
    name: "OpenCode Go",
    logo: opencode,
    color: "#84c4a6",
    primary: { label: "5 h", used: 25, resets: "下午 4:00 重設" },
    secondary: { label: "7 天", used: 40, resets: "11/21 重設" },
  },
  {
    name: "Gemini / Antigravity",
    logo: gemini,
    color: "#6f92ff",
    primary: { label: "Gemini Pro", used: 49, resets: "下午 5:10 重設" },
    secondary: { label: "Gemini Flash", used: 18, resets: "11/21 重設" },
  },
];

export const usageIntro = {
  eyebrow: "AI 用量",
  title: "AI 方案用量，一眼看清。",
  lead: "Harbor 會顯示 Claude Code、Codex、OpenCode Go，以及 Gemini CLI 或 Antigravity 的剩餘額度。可取得時，Gemini Pro 與 Flash 的額度會分開顯示。",
} as const;

export const usageFacts = [
  {
    title: "只在你使用時出現",
    body: "用量島認得四種用戶端：Claude Code、Codex、OpenCode Go，以及 Gemini CLI 或 Antigravity。",
  },
  {
    title: "沿用現有的登入",
    body: "Harbor 直接使用這些工具原本就存在電腦上的登入狀態。供應商有提供時，Gemini Pro 與 Flash 會以不同模型分開顯示。",
  },
  {
    title: "直接向供應商查詢",
    body: "每五分鐘或按下重新整理時查詢一次額度。Antigravity 透過其已驗證的本機服務查詢；查詢在獨立程序中執行，Dock 不會因此卡頓。",
  },
  {
    title: "一個開關就能關閉",
    body: "設定 → 選單列 → AI 用量。Harbor 不會儲存任何認證資訊。",
  },
] as const;
