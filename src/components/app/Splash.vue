<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { onBeforeMount } from "vue";

interface Parameter {
  id_parameter: number;
  name: string;
  value: string;
}

import { ref } from "vue";

const parameters = ref<Parameter[]>([]);
const language = ref<Parameter>({
  id_parameter: 0,
  name: "null",
  value: "null",
});
onBeforeMount(async () => {
  try {
    parameters.value = await invoke<Parameter[]>("get_parameters");
    language.value = parameters.value[0];
    console.log("Parameters loaded: ", parameters.value);
  } catch (error) {
    console.error("Failed at getting parameters: ", error);
  }
});
</script>

<template>
  <div class="splash flex column center">
    <img class="logo" src="/public/simplify.svg" />
    <div class="progress-container">
      <div class="progress-bar"></div>
    </div>
    <br />
    <span class="dots text-container text-align-left">
      {{ "Carregando" }}
      <span>.</span><span>.</span><span>.</span>
    </span>
  </div>
</template>

<style scoped>
.splash {
  width: 100vw;
  height: 100vh;
}

.logo {
  height: 50%;
  width: auto;
}

.text-container {
  width: clamp(60%, 80%, 400px);
  height: clamp(2%, 5%, 15px);
}
</style>
