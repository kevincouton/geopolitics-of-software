<template>
  <div class="min-h-screen bg-gray-50 text-gray-900">
    <header class="border-b bg-white">
      <div class="mx-auto max-w-6xl px-4 py-4">
        <NuxtLink to="/" class="text-xl font-bold">GeoSoft TrendBoard</NuxtLink>
      </div>
    </header>
    <main class="mx-auto max-w-6xl px-4 py-8">
      <slot />
    </main>
  </div>
</template>
