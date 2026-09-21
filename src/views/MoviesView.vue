<script setup lang="ts">
import { movieCategories } from '../data/mockData';
import MediaRow from '../components/MediaRow.vue';
</script>

<template>
  <div class="pt-32 px-8 pb-32 min-h-screen">
    <div class="mb-12">
      <h2 class="text-4xl font-black text-white uppercase tracking-tight">Filmy</h2>
      <p class="text-neutral-400 mt-2">Objevte ty nejlepší celovečerní filmy z naší nabídky.</p>
    </div>
    
    <div class="space-y-12">
      <MediaRow 
        v-for="category in movieCategories" 
        :key="category.title" 
        :category="category" 
      />
    </div>
  </div>
</template>
