<script setup lang="ts">
import type { TaskViewModel } from '@/api/types.gen';
import TaskStatusIcon from '@/components/TaskStatusIcon.vue';
import ErrorBar from '@/components/ErrorBar.vue';
import InlineEditor from '@/components/InlineEditor.vue';
import { renameTask, setCancelled, setDone, setPending } from '@/api'
import client from '@/services/axios'
import { ref } from 'vue';
import { useMutation, useQueryClient } from '@tanstack/vue-query';
import FloppyDiskIcon from './icons/IconFloppyDisk.vue';
import IconButton from './IconButton.vue';

const props = defineProps<{item: TaskViewModel}>()

const currentStatus = ref(props.item.status)
const statusKey = ref(0)
const isEditingTitle = ref(false)
const editTitle = ref(props.item.title)

const lastError = ref("");
const lastErrorUpd = ref(0);

const queryClient = useQueryClient();

function showError(message: string) {
  lastError.value = message;
  lastErrorUpd.value++;
}

const { mutate: updateTaskStatus } = useMutation({
  mutationFn: (status: TaskViewModel['status']) => {
    const params = { axios: client, path: { task_id: props.item.id } };

    let res;

    if (status === 'Done') {
      res = setDone(params);
    } else if (status === 'Cancelled') {
      res = setCancelled(params);
    } else if (status === 'Pending') {
      res = setPending(params);
    } else {
      throw new Error(`Invalid status ${status}`);
    }

    currentStatus.value = status;

    return res;
  },
  onSettled: () => {
    statusKey.value++;
  },
  onSuccess: (data) => {
    switch (data.status) {
      case 200:
        lastError.value = "";
        queryClient.invalidateQueries({ queryKey: ['tasks'] });
        return;
      case 409:
        showError("A pending task with the same title already exists. Please choose a different title.");
        break;
      case 400:
        showError(`Malformed request: ${data.data}`);
        break;
      case 500:
        showError("Server error occurred while adding the task. Please try again later.");
        break;
      default:
        showError(`Unexpected response from server (code ${data.status}). Please try again later.`);
    }

    currentStatus.value = props.item.status;
  },
  onError: (err) => {
    showError(`Failed to add task: ${err instanceof Error ? err.message : String(err)}`);
  }
});

const { mutate: doRenameTask } = useMutation({
  mutationFn: (title: string) => {
    return renameTask({
      axios: client,
      path: { task_id: props.item.id },
      body: { title: title.trim() },
    });
  },
  onSuccess: (data) => {
    switch (data.status) {
      case 200:
        lastError.value = "";
        isEditingTitle.value = false;
        queryClient.invalidateQueries({ queryKey: ['tasks'] });
        return;
      case 409:
        showError("A pending task with the same title already exists. Please choose a different title.");
        break;
      case 400:
        showError(`Malformed request: ${data.data}`);
        break;
      case 500:
        showError("Server error occurred while renaming the task. Please try again later.");
        break;
      default:
        showError(`Unexpected response from server (code ${data.status}). Please try again later.`);
    }
  },
  onError: (err) => {
    showError(`Failed to rename task: ${err instanceof Error ? err.message : String(err)}`);
  }
});

function beginEditTitle() {
  editTitle.value = props.item.title
  isEditingTitle.value = true
}

function submitCurrentInlineTitle() {
  handleTitleEdited(editTitle.value)
}

function handleTitleEdited(name: string) {
  doRenameTask(name);
}

function handleStatusChanged(status: TaskViewModel['status']) {
  updateTaskStatus(status);
}
</script>

<template>
  <div class="todo-item">
    <div class="todo-contents">
      <template v-if="!isEditingTitle">
        <TaskStatusIcon :key="statusKey" :status="currentStatus" @status-changed="handleStatusChanged" />
        <div :class="'title ' + props.item.status" @click="beginEditTitle">
          {{ props.item.title }}
        </div>
      </template>
      <template v-else>
        <IconButton class="edit-submit-button" @click="submitCurrentInlineTitle">
          <FloppyDiskIcon class="is-editing-icon is-editing" />
        </IconButton>
        <InlineEditor
          :class="['title', props.item.status, 'is-editing']"
          v-model:value="editTitle"
          @edited="handleTitleEdited"
        />
      </template>
    </div>
    <ErrorBar :lastError="lastError" :update-key="lastErrorUpd" />
  </div>
</template>

<style scoped>
.todo-item:not(:last-child) {
  border-bottom: 1px solid var(--base-line);
}

.edit-submit-button {
  border: none;
  background: transparent;
  padding: 0;
  margin: 0;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.is-editing-icon {
    height: 24px;
    width: 24px;
    margin: 4px;
}

.is-editing {
    color: var(--base-purple);
}

.todo-contents {
  padding: 1rem;
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;
}

.title {
  flex-grow: 1;
}

.title.Cancelled {
  text-decoration: line-through;
  color: var(--base-red);
}

.title.Done {
  text-decoration: line-through;
  color: var(--base-green);
}
</style>
