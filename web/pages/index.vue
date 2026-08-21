<script setup lang="ts">
import type { Project } from '~/composables/useProjects'

const config = useRuntimeConfig()

const { data: projects, pending, error } = await useFetch<Project[]>('/projects', {
  baseURL: config.public.apiUrl,
  server: false,
  default: () => [],
})

const isEmpty = computed(() => !pending.value && !error.value && projects.value.length === 0)
</script>

<template>
  <div>
    <h1 class="text-3xl font-bold">GitHub Trending Asia Readiness</h1>
    <p class="mt-2 text-gray-600">
      Track how trending projects are positioned for China and Asia.
    </p>

    <section class="mt-8">
      <div v-if="pending" class="text-gray-500">
        Loading trending projects…
      </div>

      <div
        v-else-if="error"
        class="rounded-lg border border-red-200 bg-red-50 p-4 text-red-700"
        role="alert"
      >
        <p class="font-medium">Could not load projects</p>
        <p class="mt-1 text-sm">
          The API may be unavailable. Showing an empty feed for now.
        </p>
      </div>

      <div v-else-if="isEmpty" class="rounded-lg border bg-white p-8 text-center text-gray-500">
        <p class="font-medium">No projects found</p>
        <p class="mt-1 text-sm">Check back once the collector has imported repositories.</p>
      </div>

      <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <ProjectCard
          v-for="project in projects"
          :key="project.id"
          :project="project"
        />
      </div>
    </section>
  </div>
</template>
