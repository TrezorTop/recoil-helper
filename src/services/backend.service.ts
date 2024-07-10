import { invoke } from "@tauri-apps/api";
import config from "../../config.json";

export type Config = {
  patterns: {
    [key: string]: Pattern[];
  };
};

type Pattern = {
  x: number;
  y: number;
  delay: number;
};

export const BackendService = {
  getConfig: (): Config => {
    return config;
  },

  saveConfig: async (config: Config) => {
    await invoke("save_config", { config });
  },

  reloadConfig: async () => {
    await invoke("reload_config");
  },
};
