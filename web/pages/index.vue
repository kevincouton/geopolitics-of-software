<script setup lang="ts">
import type { Project } from '~/composables/useProjects'

const { getProjects } = useProjects()

const limit = ref(20)
const offset = ref(0)

const { data: projects, pending, error, refresh } = await useAsyncData<Project[]>(
  'projects',
  () => getProjects({ limit: limit.value, offset: offset.value }),
  { server: false, default: () => [] },
)

watch([limit, offset], () => refresh())

const isEmpty = computed(() => !pending.value && !error.value && projects.value.length === 0)
const canGoNext = computed(() => projects.value.length === limit.value)
const canGoPrevious = computed(() => offset.value > 0)

function nextPage() {
  offset.value += limit.value
}

function previousPage() {
  offset.value = Math.max(0, offset.value - limit.value)
}
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

      <div v-else>
        <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          <ProjectCard
            v-for="project in projects"
            :key="project.id"
            :project="project"
            show-track
          />
        </div>

        <div class="mt-6 flex items-center justify-between">
          <button
            type="button"
            class="rounded-md border bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
            :disabled="!canGoPrevious"
            @click="previousPage"
          >
            Previous
          </button>
          <span class="text-sm text-gray-500">
            Page {{ offset / limit + 1 }}
          </span>
          <button
            type="button"
            class="rounded-md border bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50"
            :disabled="!canGoNext"
            @click="nextPage"
          >
            Next
          </button>
        </div>
      </div>
    </section>
  </div>
</template>
