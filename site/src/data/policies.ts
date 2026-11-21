import { site } from "@/data/site";

/** The one request Harbor makes over the network. */
export const updateManifest = `${site.releases}/latest/download/latest.json`;
