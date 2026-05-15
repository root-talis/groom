<script setup lang="ts">
import { onBeforeUnmount, ref, watch } from 'vue'

const props = defineProps<{ lastError: string, updateKey?: number|null }>()
const visible = ref(false)
let hideTimeout: ReturnType<typeof setTimeout> | null = null

watch(
  () => [props.lastError, props.updateKey],
  ([error, updateKey]) => {
    if (hideTimeout) {
      clearTimeout(hideTimeout)
      hideTimeout = null
    }

    visible.value = Boolean(error)

    if (error) {
      hideTimeout = setTimeout(() => {
        visible.value = false
      }, 3000)
    }
  },
  { immediate: true }
)

onBeforeUnmount(() => {
  if (hideTimeout) {
    clearTimeout(hideTimeout)
  }
})
</script>

<template>
  <div class="error" :class="{ shown: visible }" aria-live="assertive">
    {{ props.lastError }}
  </div>
</template>

<style scoped>
.error {
  color: var(--base-red);
  overflow: hidden;
  max-height: 0;
  opacity: 0;
  padding: 0 1rem;
  margin-top: 0;
  border-top: 1px dotted transparent;
  transition: max-height 0.4s ease, opacity 0.4s ease, padding 0.4s ease, margin-top 0.4s ease, border-color 0.4s ease;
}

.error.shown {
  opacity: 1;
  max-height: 4rem;
  padding: 0.5rem 1rem;
  margin-top: 0.5rem;
  border-top-color: var(--base-red);
}
</style>
