<script setup lang="ts">
import type { TrackedProject } from '~/composables/useProjects'

const { getTrackedProjects, untrackProject, trackProjectByOwnerName } = useProjects()

const {
  data: tracked,
  pending,
  error,
  refresh,
} = await useAsyncData<TrackedProject[]>('tracked-projects', () => getTrackedProjects(), {
  server: false,
  default: () => [],
})

const isEmpty = computed(() => !pending.value && !error.value && tracked.value.length === 0)

const newRepo = ref('')
const importing = ref(false)
const importError = ref<string | null>(null)

async function addRepo() {
  importError.value = null
  const parts = newRepo.value.trim().split('/')
  if (parts.length !== 2 || !parts[0] || !parts[1]) {
    importError.value = 'Enter a repo as owner/name'
    return
  }
  importing.value = true
  try {
    await trackProjectByOwnerName(parts[0], parts[1])
    newRepo.value = ''
    await refresh()
  }
  catch (e) {
    importError.value = 'Could not import repository. Check the owner/name and try again.'
  }
  finally {
    importing.value = false
  }
}

async function remove(projectId: string) {
  await untrackProject(projectId)
  await refresh()
}
</script>

<template>
  <div>
    <h1 class="text-3xl font-bold">Your Dashboard</h1>
    <p class="mt-2 text-gray-600">
      Projects you are tracking and their latest Asia-readiness scores.
    </p>

    <section class="mt-8">
      <div class="rounded-lg border bg-white p-4">
        <label for="add-repo" class="text-sm font-medium text-gray-700">Add repository</label>
        <div class="mt-2 flex gap-2">
          <input
            id="add-repo"
            v-model="newRepo"
            type="text"
            placeholder="owner/name"
            class="flex-1 rounded-md border border-gray-300 px-3 py-2 text-sm focus:border-blue-500 focus:outline-none focus:ring-1 focus:ring-blue-500"
            @keydown.enter="addRepo"
          >
          <button
            type="button"
            :disabled="importing"
            class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60"
            @click="addRepo"
          >
            {{ importing ? 'Importing…' : 'Add repo' }}
          </button>
        </div>
        <p v-if="importError" class="mt-2 text-sm text-red-600">
          {{ importError }}
        </p>
      </div>

      <div v-if="pending" class="mt-6 text-gray-500">
        Loading tracked projects…
      </div>

      <div
        v-else-if="error"
        class="mt-6 rounded-lg border border-red-200 bg-red-50 p-4 text-red-700"
        role="alert"
      >
        <p class="font-medium">Could not load tracked projects</p>
        <p class="mt-1 text-sm">
          The API may be unavailable. Try again later.
        </p>
      </div>

      <div v-else-if="isEmpty" class="rounded-lg border bg-white p-8 text-center text-gray-500">
        <p class="font-medium">No tracked projects yet</p>
        <p class="mt-1 text-sm">
          Browse the <NuxtLink to="/" class="text-blue-600 hover:underline">trending feed</NuxtLink> and start tracking projects.
        </p>
      </div>

      <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        <ProjectCard
          v-for="project in tracked"
          :key="project.id"
          :project="project"
        >
          <template #footer>
            <button
              type="button"
              class="mt-3 w-full rounded-md border border-red-200 bg-red-50 px-3 py-1.5 text-sm font-medium text-red-700 hover:bg-red-100"
              @click="remove(project.id)"
            >
              Stop tracking
            </button>
          </template>
        </ProjectCard>
      </div>
    </section>
  </div>
</template>
