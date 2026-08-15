import { defineStore } from "pinia";

import { useSystemStatus } from "@/core/controllers/system_status_controller";

const systemStatusController = useSystemStatus();

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
      await systemStatusController.loadSystemStatus();
      // Invert to === after build the FirstSetting component
      if (systemStatusController.getByName("appConfigured")?.value !== "false") {
        this.currentScreen = AppScreen.FirstSetting;
        return;
      }
      // TODO
      // if (userController.isUserLogged || userController.optedOutOfLogin) {
      //   this.currentScreen = AppScreen.Splash;
      //   return;
      // }
      this.currentScreen = AppScreen.Login;
    },
  },
});
