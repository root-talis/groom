<script setup lang="ts">
import { computed, ref, triggerRef, watch, type Ref } from 'vue'
import { useQuery, useMutation, useQueryClient } from '@tanstack/vue-query'
import { listTasks, addTask, type TaskViewModel } from '@/api'
import client from '@/services/axios'
import TodoItem from '@/components/TodoItem.vue'
import AddIcon from '@/components/icons/IconAdd.vue'
import ErrorBar from '@/components/ErrorBar.vue'
const title: Ref<string> = ref("");

const queryClient = useQueryClient()

const { data, isLoading, refetch } = useQuery({
  queryKey: ['tasks'],
  queryFn: () => listTasks({
    axios: client,
    query: {
      sort_by: title.value.trim() ? 'status' : 'id',
      order: title.value.trim() ? 'asc' : 'desc',
      title: title.value.trim() || null,
    }
  }),
})

const items = computed(() => {
  return data.value?.data ?? []
})

const lastError = ref("");
const lastErrorUpd = ref(0);

function showError(message: string) {
  lastError.value = message;
  lastErrorUpd.value++;
}

const { mutate: doAddTask, isPending: isAdding, isError: isAddError, error  } = useMutation({
  mutationFn: () => addTask({
    axios: client,
    body: { title: title.value },
  }),
  onSuccess: (data) => {
    switch (data.status) {
      case 200:
        title.value = "";
        lastError.value = "";
        queryClient.invalidateQueries({ queryKey: ['tasks'] });
        break;
      case 409:
        showError("A pending task with the same title already exists. Please choose a different title.");
        break;
      case 400:
        showError(`Malformed request: ${data.error || "unspecified error"}.`);
        break;
      case 500:
        showError("Server error occurred while adding the task. Please try again later.");
        break;
      default:
        showError(`Unexpected response from server (code ${data.status}). Please try again later.`);
    }
  },
  onError: (err) => {
    showError(`Failed to add task: ${err instanceof Error ? err.message : String(err)}`);
  }
})

let timeout: number|null = null;

const titleWatcher = watch(title, () => {
  if (timeout) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(() => {
    refetch();
  }, 500);
})

function search(event: Event) {
  event.preventDefault();
  if (isAdding.value) {
    return;
  }

  refetch()
}

function add(event: Event) {
  event.preventDefault();

  if (isAdding.value || !title.value.trim()) {
    return;
  }

  doAddTask()
}

</script>

<template>
  <main>
    <div id="bar">
      <form @submit="add">
        <input v-model="title" :disabled="isAdding" placeholder="A small step..."/>
        <a href="#" :class="{button: true, disabled: isAdding || !title.trim()}" @click="add">
          <AddIcon />
        </a>
      </form>
      <ErrorBar :lastError="lastError" :update-key="lastErrorUpd" />
    </div>
    <div id="results">
      <div id="todos">
        <div v-if="isLoading" class="todo-item-banner">
          Loading...
        </div>
        <div v-else-if="items.length === 0" class="todo-item-banner">
          <span class="label-info">No items found.</span>
        </div>
        <TodoItem v-else v-for="item in items" :key="item.id" :item="item" />
      </div>
    </div>
  </main>
</template>

<style scoped>
  main {
    display: flex;
    flex-direction: column;  
  }

  #bar {
    border-bottom: 1px solid var(--base-window);
  }

  form {
    padding: 1rem;

    display: flex;
    justify-content: stretch;
  }

  input {
    border: 1px solid var(--base-window);
    background: var(--base-background);
    color: var(--base-foreground);
    padding: 0.3rem 0.5rem;
    flex-grow: 1;
  }

  a.button {
    display: flex;
    align-items: center;
    padding: 0.3rem 0.5rem;

    background: var(--base-green);
    color: var(--base-background);
    opacity: 0.8;
  }

  a.button.disabled {
    opacity: 0.5;
    background: var(--base-window);
    color: var(--base-foreground);
    cursor: not-allowed;
  }

  a.button > svg {
    height: 1.5rem;
    width: 1.5rem;
  }

  a.button:hover, a.button:focus, a.button:active {
    opacity: 1.0;
  }

  #results {
    flex-grow: 1;

    overflow: auto;
  }

  #todos {
    background: var(--base-background);
    margin: 1rem;
    border: 1px solid var(--base-window);
  }

  .todo-item-banner {
    padding: 1rem;
  }
</style>
