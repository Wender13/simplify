<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';

const showBtn = import.meta.env.DEV && !window.__TAURI__;
const isFullscreen = ref(false);

const toggleFS = () => {
  if (!document.fullscreenElement) {
    document.documentElement.requestFullscreen();
  } else {
    document.exitFullscreen();
  }
};

const handleFSChange = () => {
  isFullscreen.value = !!document.fullscreenElement;
};

onMounted(() => {
  if (showBtn) document.addEventListener('fullscreenchange', handleFSChange);
});

onUnmounted(() => {
  if (showBtn) document.removeEventListener('fullscreenchange', handleFSChange);
});
</script>

<template>
  <button 
    v-if="showBtn" 
    @click="toggleFS" 
    class="fs-toggle-btn"
    title="Alternar tela cheia"
  >
    <!-- Ícone de Minimizar (Restaurar) -->
    <svg v-if="isFullscreen" xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="4 14 10 14 10 20"></polyline>
      <polyline points="20 10 14 10 14 4"></polyline>
      <line x1="14" y1="10" x2="21" y2="3"></line>
      <line x1="3" y1="21" x2="10" y2="14"></line>
    </svg>
    
    <!-- Ícone de Maximizar -->
    <svg v-else xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="15 3 21 3 21 9"></polyline>
      <polyline points="9 21 3 21 3 15"></polyline>
      <line x1="21" y1="3" x2="14" y2="10"></line>
      <line x1="3" y1="21" x2="10" y2="14"></line>
    </svg>
  </button>
</template>

<style scoped>
.fs-toggle-btn {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 9999;
  
  display: flex;
  align-items: center;
  justify-content: center;
  
  width: 44px;
  height: 44px;
  padding: 0;
  
  background-color: var(--surface);
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 12px;
  
  cursor: pointer;
  transition: all 0.2s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.fs-toggle-btn:hover {
  background-color: var(--primary-soft);
  color: var(--primary);
  border-color: var(--primary);
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.1);
}

.fs-toggle-btn:active {
  transform: translateY(0);
}
</style>