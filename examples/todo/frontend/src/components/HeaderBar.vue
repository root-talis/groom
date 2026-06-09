<script setup lang="ts">
import { RouterLink } from 'vue-router'
import TaskChecklistIcon from './icons/IconTaskChecklist.vue'
import AddIcon from './icons/IconAdd.vue'
</script>

<template>
    <header>
        <a class="menu-item" href="#">
            <TaskChecklistIcon />
        </a>
        <a class="menu-item" href="#">
            <AddIcon />
        </a>
    </header>
</template>

<style scoped>
    header {
        background: var(--base-yellow);
        color: var(--base-background);

        display: flex;
        justify-content: flex-start;
        align-items: center;
        gap: 1rem;

        min-height: 4rem;
        padding-left: 1rem;
    }

    .menu-item {
        display: flex;
        align-items: center;
        height: 2rem;
        width: 2rem;
        color: var(--base-background);
        opacity: 0.6;
    }

    .menu-item:hover {
        opacity: 1.0;
    }

    svg {
        color: var(--base-background);
    }

    @media (min-width: 1024px) {
        header {
            width: 4rem;

            flex-direction: column;
            align-items: center;
            justify-content: flex-start;

            padding-top: 1rem;
            padding-bottom: 1rem;
            padding-right: calc(var(--section-gap) / 2);
            padding-left: 0;
        }
    }

    @media (min-width: 1440px) {
        header {
            border-radius: 1rem 0 0 1rem;
        }
    }
</style>