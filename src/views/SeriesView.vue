<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MediaRow from '../components/MediaRow.vue';
import type { MediaItem, Category } from '../types';

const series = ref<MediaItem[]>([]);

const seriesCategories = computed<Category[]>(() => {
  if (series.value.length === 0) return [];
  return [{
    title: 'Všechny seriály',
    items: series.value
  }];
});

const ambientImage = computed(() => {
  return series.value[0]?.backdropUrl || series.value[0]?.posterUrl;
});

onMounted(async () => {
  series.value = await invoke<MediaItem[]>('get_series');
});
</script>

<template>
  <div class="relative min-h-screen pb-32">
    <!-- Fixní Ambientní Glow Efekt přes celou stránku -->
    <div class="fixed top-0 left-0 w-full h-screen z-0 overflow-hidden pointer-events-none">
      <img v-if="ambientImage" :src="ambientImage" class="w-full h-full object-cover opacity-20 blur-[150px] saturate-150 scale-125" />
      <div class="absolute inset-0 bg-gradient-to-b from-neutral-900/40 via-neutral-900/80 to-neutral-900/95"></div>
    </div>

    <div class="relative z-10 pt-32 px-8">
      <div class="mb-12">
        <h2 class="text-4xl font-black text-white uppercase tracking-tight drop-shadow-md">Seriály</h2>
        <p class="text-neutral-400 mt-2 font-medium">Nekonečná zábava na pokračování.</p>
      </div>
      
      <div class="space-y-12">
        <MediaRow 
          v-for="category in seriesCategories" 
          :key="category.title" 
          :category="category" 
        />
      </div>
    </div>
  </div>
</template>
