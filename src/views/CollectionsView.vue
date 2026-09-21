<script setup lang="ts">
import { allCollections } from '../data/mockData';
import MediaRow from '../components/MediaRow.vue';
</script>

<template>
  <div class="pt-32 px-8 pb-32 min-h-screen">
    <div class="mb-12">
      <h2 class="text-4xl font-black text-white uppercase tracking-tight">Kolekce</h2>
      <p class="text-neutral-400 mt-2">Užijte si celé série a propojené světy pěkně popořadě.</p>
    </div>
    
    <div class="space-y-12">
      <MediaRow 
        v-for="collection in allCollections" 
        :key="collection.title" 
        :category="collection" 
      />
    </div>
  </div>
</template>
