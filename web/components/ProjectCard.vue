<script setup lang="ts">
import type { Project } from '~/composables/useProjects'

const props = defineProps<{
  project: Project
}>()

const githubUrl = computed(
  () => `https://github.com/${props.project.github_owner}/${props.project.github_name}`,
)
</script>

<template>
  <article class="rounded-lg border bg-white p-5 shadow-sm transition hover:shadow-md">
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0">
        <a
          :href="githubUrl"
          target="_blank"
          rel="noopener noreferrer"
          class="text-lg font-semibold text-blue-600 hover:underline"
        >
          {{ project.github_owner }} / {{ project.github_name }}
        </a>
        <p v-if="project.description" class="mt-1 truncate text-sm text-gray-600">
          {{ project.description }}
        </p>
      </div>
      <ScoreBadge :score="project.score" />
    </div>

    <div class="mt-4 flex flex-wrap items-center gap-3 text-sm text-gray-500">
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
  </article>
</template>
