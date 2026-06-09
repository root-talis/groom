<script setup lang="ts">
import PendingIcon from '@/components/icons/IconPending.vue'
import DoneIcon from '@/components/icons/IconDone.vue'
import CancelIcon from '@/components/icons/IconCancel.vue'
import type { Status } from '@/api/generated/models'

const allowedStatuses = defineModel<Status[]>('allowedStatuses', { required: true })

const filters = [
  { status: 'Done' as Status, icon: DoneIcon, class: 'done' },
  { status: 'Pending' as Status, icon: PendingIcon, class: 'pending' },
  { status: 'Cancelled' as Status, icon: CancelIcon, class: 'cancelled' },
] as const

function isAllowed(status: Status) {
  return allowedStatuses.value.includes(status)
}

function toggle(status: Status) {
  if (isAllowed(status)) {
    allowedStatuses.value = allowedStatuses.value.filter((s) => s !== status)
  } else {
    allowedStatuses.value = [...allowedStatuses.value, status]
  }
}
</script>

<template>
  <div id="filters" class="filters-bar">
    <span class="filter-label">Show status:</span>
    <div class="filter-buttons">
      <button
        v-for="filter in filters"
        :key="filter.status"
        type="button"
        class="filter-button"
        :class="[filter.class, { pressed: isAllowed(filter.status) }]"
        :aria-pressed="isAllowed(filter.status)"
        :aria-label="`Filter ${filter.status}`"
        @click="toggle(filter.status)"
      >
        <component :is="filter.icon" class="status-icon" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.filters-bar {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin: 1rem 1rem 0;
  padding: 0.75rem 1rem;
  border: 1px solid var(--base-window);
  background: var(--base-background);
}

.filter-label {
  color: var(--base-foreground);
  opacity: 0.7;
}

.filter-buttons {
  display: inline-flex;
  border: 1px solid var(--base-window);
  border-radius: 0.25rem;
  overflow: hidden;
}

.filter-button {
  border: none;
  border-right: 1px solid var(--base-window);
  background: var(--base-background);
  padding: 0.35rem 0.5rem;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 0;
  opacity: 0.35;
  transition: opacity 0.15s ease, background-color 0.15s ease, color 0.15s ease;
}

.filter-button:last-child {
  border-right: none;
}

.filter-button.pressed {
  opacity: 1;
}

.filter-button.pressed.pending {
  background: var(--base-yellow);
}

.filter-button.pressed.done {
  background: var(--base-green);
}

.filter-button.pressed.cancelled {
  background: var(--base-red);
}

.filter-button:hover,
.filter-button:focus {
  opacity: 0.85;
}

.filter-button.pressed:hover,
.filter-button.pressed:focus {
  opacity: 1;
}

.status-icon {
  height: 24px;
  width: 24px;
}

.filter-button.pending .status-icon {
  color: var(--base-yellow);
}

.filter-button.done .status-icon {
  color: var(--base-green);
}

.filter-button.cancelled .status-icon {
  color: var(--base-red);
}

.filter-button.pressed .status-icon {
  color: var(--base-background);
}
</style>
