<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { useMainStore, MainComponent } from "@/stores/main.store.ts";

import Header from "@/components/main/Header.vue";
import Sidebar from "@/components/main/Sidebar.vue";
import Footer from "@/components/main/Footer.vue";
import Dashboard from "@/components/main/Dashboard.vue";
import { computed } from "vue";

const mainStore = useMainStore();
const components = {
  [MainComponent.Dashboard]: Dashboard,
};

const currentMainComponent = computed(
  () => components[mainStore.currentComponent],
);
</script>

<template>
  <Header />
  <Sidebar />
  <component :is="currentMainComponent" />
  <Footer />
</template>

<style scoped></style>
