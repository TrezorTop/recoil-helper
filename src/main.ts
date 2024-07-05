import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { invoke } from "@tauri-apps/api";

createApp(App).mount("#app");

const response = await invoke("greet", { name: "World" });

console.log(response)
