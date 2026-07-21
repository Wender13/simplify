<script setup lang="ts">
import { useAppStore, AppScreen } from "./stores/app.store.ts";
import Main from "./components/app/Main.vue";
import Splash from "./components/app/Splash.vue";
import Login from "./components/app/Login.vue";
import FirstSetting from "./components/app/FirstSetting.vue";
import { computed } from "vue";
import Notification from "./components/overlay/Notification.vue";
import Toast from "./components/overlay/Toast.vue";
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

const appStore = useAppStore();

const screens = {
  [AppScreen.Splash]: Splash,
  [AppScreen.FirstSetting]: FirstSetting,
  [AppScreen.Login]: Login,
  [AppScreen.Main]: Main,
};

const currentScreen = computed(() => screens[appStore.currentScreen]);
</script>

<template>
  <component :is="currentScreen" />
  <Notification />
  <Toast />
</template>

<style scoped></style>
