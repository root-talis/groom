<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import type { Status } from '@/api/types.gen'
import PendingIcon from '@/components/icons/IconPending.vue'
import DoneIcon from '@/components/icons/IconDone.vue'
import CancelIcon from '@/components/icons/IconCancel.vue'

const props = defineProps<{ status: Status }>()
const emit = defineEmits<{
  (e: 'status-changed', status: Status): void
}>()

const localStatus = ref(props.status);
watch(() => props.status, (newVal) => {
  localStatus.value = newVal;
});

const statuses = ['Done', 'Pending', 'Cancelled'] as const
const step = 32 + 12
const dragOffset = ref(0)
const dragging = ref(false)
const startX = ref(0)

const currentIndex = computed(() => statuses.indexOf(localStatus.value))

const trackStyle = computed(() => ({
  transform: `translateX(${-(currentIndex.value * step) + dragOffset.value}px)`,
}))

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max)
}

const dragRange = computed(() => {
  if (currentIndex.value === 0) {
    return [-step, 0]
  }

  if (currentIndex.value === statuses.length - 1) {
    return [0, step]
  }

  return [-step, step]
})

function onPointerMove(event: PointerEvent) {
  if (!dragging.value) return

  const rawDelta = event.clientX - startX.value
  const [min, max] = dragRange.value as [number, number]
  dragOffset.value = clamp(rawDelta, min, max)
}

function endDrag() {
  if (!dragging.value) return

  dragging.value = false
  const threshold = step * 0.45
  const current = currentIndex.value
  let nextIndex = current

  if (dragOffset.value > threshold && current > 0) {
    nextIndex = current - 1
  } else if (dragOffset.value < -threshold && current < statuses.length - 1) {
    nextIndex = current + 1
  }

  dragOffset.value = 0

  if (nextIndex === current) {
    return
  }

  const nextStatus = statuses[nextIndex]
  if (nextStatus === undefined) {
    console.error('Invalid next status index', nextIndex)
    return
  }
   
  localStatus.value = nextStatus
  emit('status-changed', nextStatus)
}

function onPointerUp() {
  endDrag()
  window.removeEventListener('pointermove', onPointerMove)
  window.removeEventListener('pointerup', onPointerUp)
}

function onPointerDown(event: PointerEvent) {
  if (event.button !== 0) return

  dragging.value = true
  startX.value = event.clientX
  dragOffset.value = 0

  const target = event.currentTarget as HTMLElement
  target.setPointerCapture(event.pointerId)
  window.addEventListener('pointermove', onPointerMove)
  window.addEventListener('pointerup', onPointerUp)
}

onBeforeUnmount(() => {
  window.removeEventListener('pointermove', onPointerMove)
  window.removeEventListener('pointerup', onPointerUp)
})
</script>

<template>
  <button
    type="button"
    :class="['status-button', { dragging } ]"
    @pointerdown="onPointerDown"
    aria-label="Change task status"
  >
    <div class="icon-viewport">
      <div class="track" :style="trackStyle">
        <div class="track-inner">
          <DoneIcon    class="status done" />
          <PendingIcon class="status pending" />
          <CancelIcon  class="status cancelled" />
        </div>
      </div>
    </div>
  </button>
</template>

<style scoped>
.status-button {
  border: none;
  background: transparent;
  padding: 0;
  display: inline-flex;
  align-items: center;
  cursor: grab;
  user-select: none;
}

.status-button:active {
  cursor: grabbing;
}

.icon-viewport {
  width: 32px;
  height: 32px;
  overflow: hidden;
}

.track {
  display: flex;
  will-change: transform;
  height: 32px;
  align-items: center;
}

.track-inner {
  display: flex;
  gap: 12px;
  justify-content: space-evenly;
}

@keyframes jiggle {
  0% { transform: translateX(0); }
  25% { transform: translateX(-3px); }
  50% { transform: translateX(0); }
  75% { transform: translateX(3px); }
  100% { transform: translateX(0); }
}

.icon-viewport:hover .track-inner {
  animation: jiggle 0.9s ease-in-out infinite;
}

.status-button.dragging .track-inner {
  animation-play-state: paused;
}

.status {
  height: 24px;
  width: 24px;
  flex-shrink: 0;
  margin: 0 4px;
}

.status.pending {
  color: var(--base-yellow);
}

.status.done {
  color: var(--base-green);
}

.status.cancelled {
  color: var(--base-red);
}
</style>
