<script setup lang="ts">
import type { Snapshot } from '~/composables/useProjects'

const props = defineProps<{
  snapshots: Snapshot[]
}>()

const rows = computed(() =>
  props.snapshots.map((snap) => ({
    date: snap.snapshot_date,
    score: snap.asia_readiness_score,
    stars: snap.stars,
    forks: snap.forks,
  })),
)
</script>

<template>
  <div class="rounded-lg border bg-white p-4">
    <table class="w-full text-left text-sm">
      <thead>
        <tr class="border-b text-gray-500">
          <th class="py-2 pr-4">Date</th>
          <th class="py-2 pr-4">Score</th>
          <th class="py-2 pr-4">Stars</th>
          <th class="py-2">Forks</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="row in rows"
          :key="row.date"
          class="border-b last:border-0"
        >
          <td class="py-2 pr-4">{{ row.date }}</td>
          <td class="py-2 pr-4 font-medium">{{ row.score }}</td>
          <td class="py-2 pr-4">{{ row.stars.toLocaleString() }}</td>
          <td class="py-2">{{ row.forks.toLocaleString() }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
