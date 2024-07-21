import config from "../../src-tauri/resources/config.json";
import { invoke } from "@tauri-apps/api";

export type Config = {
  patterns: Patterns;
};

type Patterns = {
  [key: string]: Step[];
};

type Step = {
  dx: number;
  dy: number;
  duration: number;
};

export const BackendService = {
  getConfig: (): Config => {
    return config;
  },

  setActivePattern: async (name: string) => {
    await invoke("set_active_pattern", {
      patternName: name,
    });
  },
};
