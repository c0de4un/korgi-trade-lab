<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const title = computed(() => {
  const metaTitle = route.meta.title
  return typeof metaTitle === 'string' ? metaTitle : 'Раздел'
})
</script>

<template>
  <div class="card-panel">
    <h2 class="text-lg font-semibold text-ink">{{ title }}</h2>
    <p class="mt-2 text-sm text-ink-muted">
      Раздел в разработке. Маршрут:
      <code class="rounded bg-surface-muted px-1.5 py-0.5 text-ink">{{ route.path }}</code>
    </p>
  </div>
</template>
