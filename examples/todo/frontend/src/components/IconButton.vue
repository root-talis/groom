<script setup lang="ts">
const props = defineProps<{ class?: string | Array<string> }>()
const emit = defineEmits<{
  (e: 'click', event: PointerEvent): void
}>()
</script>

<template>
    <button type="button" :class="['icon-button', ...(props.class ?? [])]" @click="e => emit('click', e)">
        <slot />
    </button>
</template>

<style scoped>
.icon-button {
  border: none;
  background: transparent;
  padding: 0.5rem;
  margin: -0.5rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 100vh;
  transition: background-color 0.2s ease;
}

.icon-button:hover, .icon-button:focus, .icon-button:active {
  background-color: rgba(0, 0, 0, 0.06);
}

@media (prefers-color-scheme: dark) {
  .icon-button:hover, .icon-button:focus, .icon-button:active {
    background-color: rgba(128, 128, 128, 0.06);
  }
}
</style>