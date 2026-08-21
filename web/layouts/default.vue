<template>
  <div class="min-h-screen bg-gray-50 text-gray-900">
    <header class="border-b bg-white">
      <div class="mx-auto flex max-w-6xl items-center justify-between px-4 py-4">
        <NuxtLink to="/" class="text-xl font-bold">GeoSoft TrendBoard</NuxtLink>
        <nav class="flex gap-4 text-sm font-medium text-gray-600">
          <NuxtLink to="/" class="hover:text-gray-900">Trending</NuxtLink>
          <NuxtLink to="/dashboard" class="hover:text-gray-900">Dashboard</NuxtLink>
        </nav>
      </div>
    </header>
    <main class="mx-auto max-w-6xl px-4 py-8">
      <slot />
    </main>
  </div>
</template>
