/** Figures measured on the product; keep them in sync with the dock's own plan. */
export const figures = {
  memory: { value: "3.1", unit: "MB", label: "Dock 閒置記憶體" },
  cpu: { value: "0", unit: "%", label: "閒置 CPU" },
  installer: { value: "2.4", unit: "MB", label: "安裝檔" },
  admin: { value: "0", unit: "", label: "需要系統管理員權限" },
} as const;

/** The stat strip, in the order the landing shows them. */
export const stats = Object.values(figures);
