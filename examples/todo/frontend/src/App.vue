<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import MainMenu from './components/MainMenu.vue'
</script>

<template>
  <RouterView />
</template>

<style scoped>
main {
  flex-grow: 1;
}

@media (min-width: 1440px) {
  main {
    border-left: 1px solid var(--base-window);
    border-right: 1px solid var(--base-window);
  }
}
</style>
