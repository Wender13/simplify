<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts"></script>

<template>
  <div class="splash flex column center">
    <img class="logo" src="/public/simplify.svg">
    <div class="progress-container">
      <div class="progress-bar"></div>
    </div> <br>
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
  width: clamp(60%,80%,400px);
  height: clamp(2%,5%,15px);
}
</style>
