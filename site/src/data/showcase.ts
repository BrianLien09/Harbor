import type { ImageMetadata } from "astro";
import downloads from "@/assets/icons/downloads.webp";
import fileExplorer from "@/assets/icons/file-explorer.webp";
import microsoftPhotos from "@/assets/icons/microsoft-photos.webp";
import microsoftStore from "@/assets/icons/microsoft-store.webp";
import microsoftTeams from "@/assets/icons/microsoft-teams.webp";
import recycleBin from "@/assets/icons/recycle-bin.webp";
import settings from "@/assets/icons/settings.webp";
import windowsMediaPlayer from "@/assets/icons/windows-media-player.webp";
import windowsTerminal from "@/assets/icons/windows-terminal.webp";
import { figures } from "@/data/stats";
import { site } from "@/data/site";

/* ---------------------------------------------------------------- Anatomy */

/** A tile of the annotated dock: an app icon, or one of the two drawn tiles. */
export type AnatomyTile =
  | { kind: "app"; icon: ImageMetadata; label: string; open?: boolean }
  | { kind: "start" }
  | { kind: "window" };

/** One labelled group of the dock; labels alternate above and below the glass. */
export interface AnatomyGroup {
  label: string;
  detail: string;
  side: "top" | "bottom";
  tiles: AnatomyTile[];
  /** A thin divider is drawn before this group, as in the real dock. */
  divider?: boolean;
}

export const anatomyIntro = {
  eyebrow: "Dock",
  title: "每樣東西都在該在的位置。",
  lead: "應用程式、檔案與開啟中的視窗，都收在螢幕底部的一排毛玻璃架上，每個部分的用法都跟你在 Mac 上習慣的一樣。",
} as const;

export const anatomy: AnatomyGroup[] = [
  { label: "開始", detail: "Windows 開始功能表，永遠排第一", side: "top", tiles: [{ kind: "start" }] },
  {
    label: "釘選的應用程式",
    detail: "拖曳即可重新排序，開啟中的會有小圓點",
    side: "bottom",
    tiles: [
      { kind: "app", icon: fileExplorer, label: "檔案總管", open: true },
      { kind: "app", icon: microsoftStore, label: "Microsoft Store" },
      { kind: "app", icon: windowsTerminal, label: "終端機", open: true },
      { kind: "app", icon: settings, label: "設定" },
    ],
  },
  {
    label: "最近使用",
    detail: "最近用過的三個應用程式",
    side: "top",
    tiles: [
      { kind: "app", icon: microsoftPhotos, label: "相片", open: true },
      { kind: "app", icon: windowsMediaPlayer, label: "媒體播放器" },
      { kind: "app", icon: microsoftTeams, label: "Teams" },
    ],
  },
  {
    label: "下載",
    detail: "檔案可以直接拖進來",
    side: "bottom",
    divider: true,
    tiles: [{ kind: "app", icon: downloads, label: "下載" }],
  },
  { label: "最小化", detail: "可選擇以即時縮圖顯示最小化的視窗", side: "top", tiles: [{ kind: "window" }] },
  {
    label: "資源回收筒",
    detail: "把檔案拖進來就能刪除",
    side: "bottom",
    tiles: [{ kind: "app", icon: recycleBin, label: "資源回收筒" }],
  },
];

/* -------------------------------------------------------------- Customize */

export const customizeIntro = {
  eyebrow: "設定",
  title: "調成你喜歡的樣子。",
  lead: "真正的設定視窗，預設就是繁體中文（也能切換英文或西班牙文）。每個選項都能在這裡試，包括選用的滑鼠與耳機電量指示；配件偵測在你開啟前都不會啟動。",
} as const;

/** The preview dock in the Customize scene. */
export const previewIcons: ImageMetadata[] = [
  fileExplorer,
  microsoftStore,
  windowsTerminal,
  microsoftPhotos,
  settings,
  downloads,
  recycleBin,
];

/** Same defaults as the app (`crates/harbor-config`). */
export const settingsDefaults = {
  iconSize: 48,
  iconSizeMin: 32,
  iconSizeMax: 96,
  magnification: true,
  magnifiedSize: 77,
  magnifiedSizeMax: 160,
  autoHide: false,
  menuBar: true,
  menuBarAutoHide: false,
  aiUsage: true,
  accessoryBattery: false,
  genie: true,
  showMinimized: false,
  autostart: true,
  mascot: true,
  autoUpdate: true,
} as const;

/**
 * 設定應用程式本身的字串（`apps/settings/src/i18n`）。
 * zh 放第一個並作為預設，因為 Harbor 這個 fork 的主要使用者是繁體中文使用者；
 * 中文字串盡量沿用 app 的 zh-TW.json，網站展示才會和實際介面一致。
 */
export const settingsText = {
  zh: {
    title: "Harbor 設定",
    dock: "Dock",
    windows: "視窗",
    menubar: "選單列",
    system: "系統",
    size: "大小",
    small: "小",
    large: "大",
    magnification: "放大效果",
    magnificationHint: "游標經過時，圖示會跟著放大。",
    magnifiedSize: "放大後的大小",
    autoHide: "自動隱藏",
    autoHideHint: "將游標停在 Dock 所在的邊緣即可顯示，任何螢幕都適用。",
    menuBar: "顯示選單列",
    menuBarHint: "時間、電池，以及 Windows 快速設定與通知的入口。",
    menuBarAutoHide: "自動隱藏",
    menuBarAutoHideHint: "將游標停在螢幕頂端邊緣即可顯示。",
    aiUsage: "AI 用量",
    aiUsageHint: "使用你現有的登入工作階段；每 5 分鐘檢查一次，絕不儲存認證資訊。",
    accessoryBattery: "顯示配件電量",
    accessoryBatteryHint: "支援的滑鼠與耳機。關閉時不會查詢裝置。",
    accessoryPreviewHint: "開啟後可預覽已連接的滑鼠與耳機。數值僅為範例，實際支援依裝置而定。",
    genie: "神燈效果",
    genieHint: "開啟時，視窗不會使用 Windows 的最大化與關閉動畫。",
    showMinimized: "顯示最小化的視窗",
    showMinimizedHint: "縮圖會與應用程式圖示分開顯示。",
    autostart: "登入時開啟 Harbor",
    mascot: "在特殊時刻出現螃蟹",
    mascotHint: "AI 方案變更、充電與省電模式時，會有一隻螃蟹來提示。把游標移到牠身上看看。",
    autoUpdate: "自動安裝更新",
    autoUpdateHint: "每天安裝一次新版本。",
    language: "語言",
    languageAuto: "自動",
    updates: "更新",
    updatesCurrent: `Harbor ${site.version} 已是最新版本。`,
    updatesChecking: "正在檢查更新…",
    updatesCheck: "檢查",
  },
  en: {
    title: "Harbor Settings",
    dock: "Dock",
    windows: "Windows",
    menubar: "Menu bar",
    system: "System",
    size: "Size",
    small: "Small",
    large: "Large",
    magnification: "Magnification",
    magnificationHint: "Icons grow as the pointer passes over them.",
    magnifiedSize: "Magnified size",
    autoHide: "Automatically hide and show the Dock",
    autoHideHint: "It reappears when you touch the bottom edge of the screen.",
    menuBar: "Show the menu bar",
    menuBarHint: "Time, battery, and access to the Windows quick settings and notifications.",
    menuBarAutoHide: "Automatically hide and show the menu bar",
    menuBarAutoHideHint: "It reappears when you touch the top edge of the screen.",
    aiUsage: "Show AI provider usage",
    aiUsageHint: "While a supported AI client is open, an island in the menu bar shows its available plan limits.",
    accessoryBattery: "Show accessory battery",
    accessoryBatteryHint: "Battery levels for supported mice and headsets. While off, no accessories are monitored or queried.",
    accessoryPreviewHint: "Turn this on to preview a connected mouse and headset. The values are examples; device support varies.",
    genie: "Genie effect",
    genieHint: "While it is on, windows do not use the Windows animations for maximizing or closing.",
    showMinimized: "Show minimized windows",
    showMinimizedHint: "As thumbnails on the right side of the Dock, instead of inside their app's icon.",
    autostart: "Open Harbor at login",
    mascot: "Crab on special moments",
    mascotHint: "A pixel crab comes out when an AI plan resets or runs out, when you plug in the charger, or when energy saver starts. Hover over it.",
    autoUpdate: "Install updates automatically",
    autoUpdateHint: "Once a day, a couple of minutes after you turn on the computer (or every 24 hours if it stays on), Harbor installs new versions by itself.",
    language: "Language",
    languageAuto: "Automatic",
    updates: "Updates",
    updatesCurrent: `Harbor ${site.version} is up to date.`,
    updatesChecking: "Checking for updates…",
    updatesCheck: "Check",
  },
  es: {
    title: "Ajustes de Harbor",
    dock: "Dock",
    windows: "Ventanas",
    menubar: "Barra superior",
    system: "Sistema",
    size: "Tamaño",
    small: "Pequeño",
    large: "Grande",
    magnification: "Ampliación",
    magnificationHint: "Los iconos crecen al pasar el puntero sobre ellos.",
    magnifiedSize: "Tamaño ampliado",
    autoHide: "Ocultar y mostrar el Dock automáticamente",
    autoHideHint: "Reaparece al tocar el borde inferior de la pantalla.",
    menuBar: "Mostrar la barra superior",
    menuBarHint: "Hora, batería y acceso a los ajustes rápidos y las notificaciones de Windows.",
    menuBarAutoHide: "Ocultar y mostrar la barra superior automáticamente",
    menuBarAutoHideHint: "Reaparece al tocar el borde superior de la pantalla.",
    aiUsage: "Mostrar uso de proveedores de IA",
    aiUsageHint: "Mientras un cliente de IA compatible está abierto, una isla en la barra superior muestra sus límites disponibles.",
    accessoryBattery: "Mostrar batería de accesorios",
    accessoryBatteryHint: "Batería de ratones y auriculares compatibles. Desactivado, no detecta ni consulta accesorios.",
    accessoryPreviewHint: "Actívalo para ver un ratón y unos auriculares conectados. Los valores son ejemplos; la compatibilidad depende del dispositivo.",
    genie: "Efecto genio",
    genieHint: "Mientras está activo, las ventanas no usan las animaciones de Windows al maximizar ni al cerrar.",
    showMinimized: "Mostrar ventanas minimizadas",
    showMinimizedHint: "Como miniaturas a la derecha del Dock, en lugar de dentro del icono de su app.",
    autostart: "Abrir Harbor al iniciar sesión",
    mascot: "Cangrejo en momentos especiales",
    mascotHint: "Un cangrejo pixelado aparece cuando un plan de IA se reinicia o se agota, al enchufar el cargador o al activarse el ahorro de energía. Pasa el puntero por encima.",
    autoUpdate: "Instalar actualizaciones automáticamente",
    autoUpdateHint: "Una vez al día, un par de minutos después de encender el ordenador (o cada 24 horas si sigue encendido), Harbor instala las versiones nuevas por sí solo.",
    language: "Idioma",
    languageAuto: "Automático",
    updates: "Actualizaciones",
    updatesCurrent: `Harbor ${site.version} está al día.`,
    updatesChecking: "Buscando actualizaciones…",
    updatesCheck: "Buscar",
  },
} as const;

export type SettingsLang = keyof typeof settingsText;
export type SettingsKey = keyof (typeof settingsText)["zh"];

/* ----------------------------------------------------------------- Safety */

export const safetyIntro = {
  eyebrow: "隨時可還原",
  title: "工作列沒有消失，只是先收起來。",
  lead: "Harbor 執行時會隱藏 Windows 工作列，一結束就立刻還給你。切換開關試試看。",
} as const;

/** The taskbar shown when Harbor is off. */
export const taskbarIcons: ImageMetadata[] = [fileExplorer, microsoftStore, windowsTerminal, microsoftPhotos, settings];

export const safetyFacts = [
  { title: "隨時結束", body: "在 Dock 上按右鍵選擇「結束 Harbor」，工作列馬上回來。" },
  { title: "就算意外停止也沒事", body: "一個小小的守護程序會盯著 Dock，萬一它停止就自動還原工作列。" },
  { title: "乾淨解除安裝", body: "Windows 設定 → 應用程式 → Harbor，Windows 會完全恢復原狀。" },
] as const;

/* --------------------------------------------------------------- Download */

export const download = {
  name: site.name,
  tagline: site.tagline,
  meta: [
    `版本 ${site.version}`,
    "Windows 10／11 · 64 位元",
    `${figures.installer.value} ${figures.installer.unit}`,
    `免費 · ${site.license}`,
  ],
  cta: "下載 Windows 版",
  releases: "GitHub 上的所有版本",
  steps: [
    { title: "開啟安裝檔", body: "只安裝給目前使用者，不需系統管理員權限，也沒有安裝精靈。" },
    { title: "如果 SmartScreen 跳出警告", body: "Harbor 還很新，Windows 可能還不認得它：請選「其他資訊 → 仍要執行」。" },
    { title: "完成", body: "Dock 會出現，之後也會隨 Windows 自動啟動。" },
  ],
  source: {
    summary: "從原始碼建置",
    note: "需要 Windows 10（1903 以上）或 11、Rust（MSVC 工具鏈）與 Visual Studio C++ Build Tools。",
    commands: [`git clone ${site.repository}`, "cd Harbor", "cargo run --release -p harbor"],
  },
} as const;
