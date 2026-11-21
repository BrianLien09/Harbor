import { site } from "@/data/site";

/** SignPath Foundation requires this exact credit wherever signing is mentioned. */
export const signingCredit = "Free code signing provided by SignPath.io, certificate by SignPath Foundation.";

/**
 * Who may change the code and who approves each signed release, as the
 * SignPath Foundation code signing policy asks. Keep it in sync with the
 * people who have write access to the repository.
 */
export const signingTeam = {
  committers: [{ name: "BrianLien09", href: "https://github.com/BrianLien09" }],
  reviewers: [{ name: "BrianLien09", href: "https://github.com/BrianLien09" }],
  approvers: [{ name: "BrianLien09", href: "https://github.com/BrianLien09" }],
} as const;

/** The one request Harbor makes over the network. */
export const updateManifest = `${site.releases}/latest/download/latest.json`;
