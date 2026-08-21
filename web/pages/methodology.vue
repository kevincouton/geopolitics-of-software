<script setup lang="ts">
useHead({
  title: 'Methodology — GeoSoft TrendBoard',
})
</script>

<template>
  <div>
    <h1 class="text-3xl font-bold">Methodology</h1>
    <p class="mt-2 text-gray-600">
      How GeoSoft TrendBoard calculates the Asia-readiness score.
    </p>

    <section class="mt-8 space-y-6">
      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">Overall formula</h2>
        <p class="mt-2 text-gray-600">
          The Asia-readiness score is a weighted sum of four sub-scores, each normalized to 0–100.
        </p>
        <pre class="mt-4 overflow-x-auto rounded-md bg-gray-100 p-4 text-sm text-gray-800">
asia_readiness_score =
  docs_score        * 0.30 +
  platform_score    * 0.25 +
  social_score      * 0.25 +
  community_score   * 0.20</pre>
      </div>

      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">Scoring weights</h2>
        <dl class="mt-4 grid gap-4 sm:grid-cols-2">
          <div>
            <dt class="font-medium text-gray-900">Documentation — 30%</dt>
            <dd class="mt-1 text-sm text-gray-600">
              English README quality, Chinese README presence, and architecture docs.
            </dd>
          </div>
          <div>
            <dt class="font-medium text-gray-900">Platform presence — 25%</dt>
            <dd class="mt-1 text-sm text-gray-600">
              Gitee mirror, GitHub topics, release notes, and discussions enabled.
            </dd>
          </div>
          <div>
            <dt class="font-medium text-gray-900">Social signals — 25%</dt>
            <dd class="mt-1 text-sm text-gray-600">
              Mentions and engagement on Juejin, Zhihu, V2EX, and Bilibili.
            </dd>
          </div>
          <div>
            <dt class="font-medium text-gray-900">Community health — 20%</dt>
            <dd class="mt-1 text-sm text-gray-600">
              Issue response time, contributor count, license, and security policy.
            </dd>
          </div>
        </dl>
      </div>

      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">Data sources</h2>
        <ul class="mt-4 list-disc space-y-2 pl-5 text-gray-600">
          <li>GitHub API for repository metadata, stars, forks, issues, and contributors.</li>
          <li>Gitee API for mirror presence and Chinese platform signals.</li>
          <li>Juejin, Zhihu, V2EX, and Bilibili for regional social mentions and engagement.</li>
        </ul>
      </div>

      <div class="rounded-lg border bg-white p-6">
        <h2 class="text-xl font-semibold">Update cadence</h2>
        <p class="mt-2 text-gray-600">
          Scores are recalculated daily from fresh snapshots. Weights may be adjusted over time as we learn which signals best predict adoption in China and Asia.
        </p>
      </div>
    </section>
  </div>
</template>
