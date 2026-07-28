import { defineStore } from "pinia";

export enum MainComponent {
  Dashboard = "dashboard",
}

export const useMainStore = defineStore("app", {
  state: () => ({
    currentComponent: MainComponent.Dashboard,
  }),
  actions: {
    async bootstrap() {
      // check auth, config, etc.
      this.currentComponent = MainComponent.Dashboard;
    },
  },
});
