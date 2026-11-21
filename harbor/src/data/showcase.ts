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
  eyebrow: "The Dock",
  title: "Everything in its place.",
  lead: "Your apps, your files and your open windows, in one glass shelf at the bottom of the screen. Every part of it works like you expect from a Mac.",
} as const;

export const anatomy: AnatomyGroup[] = [
  { label: "Start", detail: "The Windows Start menu, always first", side: "top", tiles: [{ kind: "start" }] },
  {
    label: "Pinned apps",
    detail: "Drag to reorder. A dot marks the open ones",
    side: "bottom",
    tiles: [
      { kind: "app", icon: fileExplorer, label: "File Explorer", open: true },
      { kind: "app", icon: microsoftStore, label: "Microsoft Store" },
      { kind: "app", icon: windowsTerminal, label: "Terminal", open: true },
      { kind: "app", icon: settings, label: "Settings" },
    ],
  },
  {
    label: "Recent apps",
    detail: "The last three you used",
    side: "top",
    tiles: [
      { kind: "app", icon: microsoftPhotos, label: "Photos", open: true },
      { kind: "app", icon: windowsMediaPlayer, label: "Media Player" },
      { kind: "app", icon: microsoftTeams, label: "Teams" },
    ],
  },
  {
    label: "Downloads",
    detail: "Drop files straight in",
    side: "bottom",
    divider: true,
    tiles: [{ kind: "app", icon: downloads, label: "Downloads" }],
  },
  { label: "Minimized", detail: "Optional live thumbnails of minimized windows", side: "top", tiles: [{ kind: "window" }] },
  {
    label: "Recycle Bin",
    detail: "Drop files to delete them",
    side: "bottom",
    tiles: [{ kind: "app", icon: recycleBin, label: "Recycle Bin" }],
  },
];

/* -------------------------------------------------------------- Customize */

export const customizeIntro = {
  eyebrow: "Settings",
  title: "Make it yours.",
  lead: "A real settings window, in English or Spanish. Try each change here, including optional mouse and headset battery indicators. Accessory monitoring stays off until you enable it.",
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

/** The Settings app's own strings (`apps/settings/src/i18n`), in both languages. */
export const settingsText = {
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

export type SettingsKey = keyof (typeof settingsText)["en"];

/* ----------------------------------------------------------------- Safety */

export const safetyIntro = {
  eyebrow: "Reversible",
  title: "Your taskbar is never gone. Just tucked away.",
  lead: "Harbor hides the Windows taskbar while it runs and gives it back the moment it stops. Flip the switch and see.",
} as const;

/** The taskbar shown when Harbor is off. */
export const taskbarIcons: ImageMetadata[] = [fileExplorer, microsoftStore, windowsTerminal, microsoftPhotos, settings];

export const safetyFacts = [
  { title: "Quit any time", body: "Right-click the dock and choose Quit Harbor. The taskbar is back at once." },
  { title: "Even if it stops", body: "A tiny guardian process watches the dock and restores the taskbar if it ever stops." },
  { title: "Uninstall cleanly", body: "Windows Settings → Apps → Harbor. Windows is left exactly as it was." },
] as const;

/* --------------------------------------------------------------- Download */

export const download = {
  name: site.name,
  tagline: site.tagline,
  meta: [
    `Version ${site.version}`,
    "Windows 10 & 11 · 64-bit",
    `${figures.installer.value} ${figures.installer.unit}`,
    `Free · ${site.license}`,
  ],
  cta: "Download for Windows",
  releases: "All versions on GitHub",
  steps: [
    { title: "Open the installer", body: "Per user. No admin rights, no wizard." },
    { title: "If SmartScreen asks", body: "Harbor is new, so Windows may not know it yet: More info → Run anyway." },
    { title: "That's it", body: "The dock appears and starts with Windows from now on." },
  ],
  source: {
    summary: "Build it from source",
    note: "Needs Windows 10 (1903+) or 11, Rust (MSVC) and the Visual Studio C++ build tools.",
    commands: [`git clone ${site.repository}`, "cd Harbor", "cargo run --release -p harbor"],
  },
} as const;
