import { defineStore } from "pinia";

export enum AppScreen {
  Splash = "splash",
  FirstSetting = "firstSetting",
  Login = "login",
  Main = "main",
}

export const useAppStore = defineStore("app", {
  state: () => ({
    currentScreen: AppScreen.Splash,
  }),
  actions: {
    async bootstrap() {
      // check auth, config, etc.
      this.currentScreen = AppScreen.Splash;
    },
  },
});
