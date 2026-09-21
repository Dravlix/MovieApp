<script setup lang="ts">
import { seriesCategories } from '../data/mockData';
import MediaRow from '../components/MediaRow.vue';
</script>

<template>
  <div class="pt-32 px-8 pb-32 min-h-screen">
    <div class="mb-12">
      <h2 class="text-4xl font-black text-white uppercase tracking-tight">Seriály</h2>
      <p class="text-neutral-400 mt-2">Nekonečná zábava na pokračování.</p>
    </div>
    
    <div class="space-y-12">
      <MediaRow 
        v-for="category in seriesCategories" 
        :key="category.title" 
        :category="category" 
      />
    </div>
  </div>
</template>
