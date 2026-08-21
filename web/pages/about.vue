<script setup lang="ts">
useHead({
  title: 'About — GeoSoft TrendBoard',
})
</script>

<template>
  <div>
    <h1 class="text-3xl font-bold">About</h1>
    <p class="mt-2 text-gray-600">
      The geopolitics of software thesis behind GeoSoft TrendBoard.
    </p>

    <section class="mt-8 space-y-6">
      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">Open source is global; distribution is local</h2>
        <p class="mt-2 text-gray-600">
          Open-source projects are global by default, but distribution is local. A project can trend on GitHub and still miss the Chinese and Asian market because of language, platform, payment, and compliance gaps.
        </p>
      </div>

      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">Why it matters</h2>
        <p class="mt-2 text-gray-600">
          Asia represents a large and growing share of the world's developers. Yet most maintainers have no simple way to answer:
        </p>
        <ul class="mt-4 list-disc space-y-2 pl-5 text-gray-600">
          <li>Is my project discoverable in China?</li>
          <li>What are my competitors doing on Juejin, Zhihu, or Gitee?</li>
          <li>Which two or three positioning changes would have the biggest impact?</li>
        </ul>
      </div>

      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">What GeoSoft TrendBoard does</h2>
        <p class="mt-2 text-gray-600">
          GeoSoft TrendBoard tracks GitHub trending projects and scores their readiness for adoption across Chinese and Asian developer ecosystems. It surfaces actionable gaps — such as adding a Chinese README, mirroring to Gitee, or posting a demo on Bilibili — so maintainers can prioritize the moves that matter.
        </p>
      </div>
    </section>
  </div>
</template>
