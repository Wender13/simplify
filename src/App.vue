<script setup lang="ts">
// ---------Debbuging---------
import ToggleFullScreen from "@/components/debug/ToggleFullScreen.vue";

import { computed } from "vue";
// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

import { useAppStore, AppScreen } from "@/stores/app.store.ts";

import Main from "@/components/app/Main.vue";
import Splash from "@/components/app/Splash.vue";
import Login from "@/components/app/Login.vue";
import FirstSetting from "@/components/app/FirstSetting.vue";
import Notification from "@/components/overlay/Notification.vue";
import Toast from "@/components/overlay/Toast.vue";

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
  <!-- ---------Debbuging--------- -->
  <ToggleFullScreen />
  <!-- ---------Debbuging--------- -->
  <component :is="currentScreen" />
  <Notification />
  <Toast />
</template>

<style>
* {
  background-color: var(--bg);
  color: var(--text);

  margin: 0;
  padding: 0;
}
</style>
