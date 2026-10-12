import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { setupBrowserTauriMock } from "./utils/tauriMock";

setupBrowserTauriMock();

createApp(App).mount("#app");