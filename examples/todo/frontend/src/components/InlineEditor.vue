<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'

const props = defineProps<{ value: string }>()
const emit = defineEmits<{
  (e: 'edited', value: string): void
  (e: 'update:value', value: string): void
}>()

const inputValue = ref(props.value)
const inputRef = ref<HTMLInputElement | null>(null)

watch(
  () => props.value,
  (newValue) => {
    inputValue.value = newValue
  }
)

onMounted(() => {
  inputRef.value?.focus()
  inputRef.value?.select()
})

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter') {
    event.preventDefault()
    emit('edited', inputValue.value)
  }
}

function handleInput(event: Event) {
  inputValue.value = (event.target as HTMLInputElement).value
  emit('update:value', inputValue.value)
}
</script>

<template>
  <input
    ref="inputRef"
    type="text"
    class="inline-editor"
    v-model="inputValue"
    @input="handleInput"
    @keydown="handleKeydown"
  />
</template>

<style scoped>
.inline-editor {
  width: 100%;
  min-width: 0;
  border: none;
  background: transparent;
  font: inherit;
  line-height: inherit;
  padding: 0;
  margin: 0;
  margin-bottom: -1px;
  color: inherit;
  outline: none;
  border-bottom: 1px solid var(--base-purple);
}
</style>
