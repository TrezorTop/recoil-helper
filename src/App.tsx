import "./App.module.css";

import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export const App = () => {
  const [activePattern, setActivePattern] = useState<string>();

  useEffect(() => {
    listen<string>("pattern-selected", (value) => {
      setActivePattern(value.payload);
    });
  }, []);

  return <div>active pattern: {activePattern}</div>;
};
