import { createApp } from "vue";
import { createPinia } from "pinia";
import { setupI18n, loadLocaleMessages } from "@/i18n/index";

import "@/style/main.css";
import App from "@/App.vue";

const i18n = setupI18n({ locale: "en-US" });
await loadLocaleMessages(i18n, "en-US");

createApp(App).use(createPinia()).use(i18n).mount("#app");
