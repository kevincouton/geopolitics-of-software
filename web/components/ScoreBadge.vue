<script setup lang="ts">
const props = defineProps<{
  score: number
}>()

const variant = computed(() => {
  if (props.score >= 80) return 'bg-emerald-100 text-emerald-800'
  if (props.score >= 50) return 'bg-amber-100 text-amber-800'
  return 'bg-rose-100 text-rose-800'
})
</script>

<template>
  <span
    class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium"
    :class="variant"
  >
    Asia Readiness {{ score }}
  </span>
</template>
