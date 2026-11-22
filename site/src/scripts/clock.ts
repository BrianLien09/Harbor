// 和 app 選單列一致：12 小時制、上午／下午寫在前面（例如「上午 9:41」）。
const timeFormat = new Intl.DateTimeFormat("zh-TW", { hour: "numeric", minute: "2-digit", hour12: true });
const dateFormat = new Intl.DateTimeFormat("zh-TW", { weekday: "short", day: "numeric", month: "short" });

let started = false;

/** Keeps every fake menu bar clock in step with the machine, ticking on the minute. */
export function initClock() {
  if (started) return;
  started = true;

  const update = () => {
    const now = new Date();
    document.querySelectorAll<HTMLElement>("[data-clock]").forEach((node) => {
      node.textContent = timeFormat.format(now);
    });
    document.querySelectorAll<HTMLElement>("[data-date]").forEach((node) => {
      node.textContent = dateFormat.format(now);
    });
  };

  const tick = () => {
    update();
    window.setTimeout(tick, 60_000 - (Date.now() % 60_000) + 50);
  };
  tick();

  // Timers are throttled in background tabs: catch up the moment the tab is visible again.
  document.addEventListener("visibilitychange", () => {
    if (!document.hidden) update();
  });
}