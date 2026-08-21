<script setup lang="ts">
import type { Project, Snapshot } from '~/composables/useProjects'

const route = useRoute()
const owner = computed(() => route.params.owner as string)
const name = computed(() => route.params.name as string)

const { getProject, getProjectSnapshots } = useProjects()

const {
  data: project,
  pending: projectPending,
  error: projectError,
} = await useAsyncData<Project>(
  () => getProject(owner.value, name.value),
  { server: false },
)

const {
  data: snapshots,
  pending: snapshotsPending,
  error: snapshotsError,
} = await useAsyncData<Snapshot[]>(
  () => getProjectSnapshots(owner.value, name.value),
  { server: false, default: () => [] },
)

const { trackProjectByOwnerName } = useProjects()

const tracked = ref(false)
const tracking = ref(false)

async function track() {
  tracking.value = true
  try {
    await trackProjectByOwnerName(owner.value, name.value)
    tracked.value = true
  }
  finally {
    tracking.value = false
  }
}

const githubUrl = computed(
  () => `https://github.com/${owner.value}/${name.value}`,
)

const latestSnapshot = computed<Snapshot | undefined>(() => {
  if (!snapshots.value.length) return undefined
  return snapshots.value[snapshots.value.length - 1]
})

const scoreBreakdown = computed(() => {
  const snap = latestSnapshot.value
  if (!snap) return null
  return [
    { label: 'Documentation', score: snap.docs_score },
    { label: 'Platform', score: snap.platform_score },
    { label: 'Social', score: snap.social_score },
    { label: 'Community', score: snap.community_score },
  ]
})

interface Recommendation {
  label: string
  action: string
}

const recommendations = computed<Recommendation[]>(() => {
  const items: Recommendation[] = []
  const snap = latestSnapshot.value
  const proj = project.value

  if (!proj) return items

  if (!proj.has_chinese_readme) {
    items.push({
      label: 'Documentation',
      action: 'Add a Chinese README to improve accessibility in Asia.',
    })
  }

  if (!proj.description) {
    items.push({
      label: 'Documentation',
      action: 'Add a project description so visitors quickly understand the project.',
    })
  }

  if (!proj.has_gitee_mirror) {
    items.push({
      label: 'Platform',
      action: 'Mirror the repository to Gitee for better availability in China.',
    })
  }

  if (snap) {
    if (snap.docs_score < 60) {
      items.push({
        label: 'Documentation',
        action: 'Improve documentation score by adding Chinese README, description, and topic tags.',
      })
    }
    if (snap.platform_score < 60) {
      items.push({
        label: 'Platform',
        action: 'Improve platform score by mirroring to Gitee and adding topic tags.',
      })
    }
    if (snap.social_score < 60) {
      items.push({
        label: 'Social',
        action: 'Increase social visibility by sharing the project on relevant channels.',
      })
    }
    if (snap.community_score < 60) {
      items.push({
        label: 'Community',
        action: 'Grow community engagement by encouraging issues, forks, and contributions.',
      })
    }
  }

  return items
})

const pending = computed(() => projectPending.value || snapshotsPending.value)
const error = computed(() => projectError.value || snapshotsError.value)
</script>

<template>
  <div>
    <div v-if="pending" class="text-gray-500">
      Loading project details…
    </div>

    <div
      v-else-if="error"
      class="rounded-lg border border-red-200 bg-red-50 p-4 text-red-700"
      role="alert"
    >
      <p class="font-medium">Could not load project</p>
      <p class="mt-1 text-sm">
        The API may be unavailable or the project was not found.
      </p>
    </div>

    <article v-else-if="project">
      <div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <a
            :href="githubUrl"
            target="_blank"
            rel="noopener noreferrer"
            class="text-2xl font-bold text-blue-600 hover:underline"
          >
            {{ project.github_owner }} / {{ project.github_name }}
          </a>
          <p v-if="project.description" class="mt-2 max-w-2xl text-gray-600">
            {{ project.description }}
          </p>
        </div>
        <div class="flex flex-col items-end gap-3">
          <ScoreBadge :score="project.score" />
          <button
            type="button"
            :disabled="tracking || tracked"
            class="rounded-md border border-blue-200 bg-blue-50 px-4 py-2 text-sm font-medium text-blue-700 hover:bg-blue-100 disabled:cursor-not-allowed disabled:opacity-60"
            @click="track"
          >
            {{ tracked ? 'Tracked' : (tracking ? 'Tracking…' : 'Track this project') }}
          </button>
        </div>
      </div>

      <div class="mt-6 flex flex-wrap items-center gap-3 text-sm text-gray-500">
        <span v-if="project.language" class="rounded-full bg-gray-100 px-2 py-0.5 font-medium text-gray-700">
          {{ project.language }}
        </span>
        <span class="flex items-center gap-1">
          <span aria-hidden="true">⭐</span>
          {{ project.stars.toLocaleString() }} stars
        </span>
        <span class="flex items-center gap-1">
          <span aria-hidden="true">🍴</span>
          {{ project.forks.toLocaleString() }} forks
        </span>
      </div>

      <div class="mt-3 flex flex-wrap gap-2">
        <span
          v-if="project.has_chinese_readme"
          class="inline-flex items-center rounded bg-blue-50 px-2 py-1 text-xs font-medium text-blue-700"
        >
          Chinese README
        </span>
        <span
          v-if="project.has_gitee_mirror"
          class="inline-flex items-center rounded bg-purple-50 px-2 py-1 text-xs font-medium text-purple-700"
        >
          Gitee Mirror
        </span>
      </div>

      <section class="mt-8">
        <h2 class="text-xl font-semibold">Score Breakdown</h2>
        <div
          v-if="scoreBreakdown"
          class="mt-4 grid gap-4 sm:grid-cols-2 lg:grid-cols-4"
        >
          <div
            v-for="item in scoreBreakdown"
            :key="item.label"
            class="rounded-lg border bg-white p-4"
          >
            <p class="text-sm text-gray-500">{{ item.label }}</p>
            <p class="mt-1 text-2xl font-bold text-gray-900">
              {{ item.score }}
            </p>
          </div>
        </div>
        <p v-else class="mt-2 text-gray-500">
          No snapshots available yet.
        </p>
      </section>

      <section v-if="recommendations.length" class="mt-8">
        <h2 class="text-xl font-semibold">Recommendations</h2>
        <ul class="mt-4 space-y-3">
          <li
            v-for="(rec, index) in recommendations"
            :key="index"
            class="rounded-lg border bg-white p-4"
          >
            <span
              class="inline-block rounded bg-gray-100 px-2 py-0.5 text-xs font-medium text-gray-700"
            >
              {{ rec.label }}
            </span>
            <p class="mt-2 text-gray-700">{{ rec.action }}</p>
          </li>
        </ul>
      </section>

      <section class="mt-8">
        <h2 class="text-xl font-semibold">History</h2>
        <TrendChart
          v-if="snapshots.length"
          class="mt-4"
          :snapshots="snapshots"
        />
        <p v-else class="mt-2 text-gray-500">
          Historical data will appear once snapshots are collected.
        </p>
      </section>
    </article>
  </div>
</template>
