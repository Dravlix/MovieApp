<script setup lang="ts">
import { wishlistData } from '../data/mockData';
import MediaCard from '../components/MediaCard.vue';
</script>

<template>
  <div class="pt-32 px-8 pb-32 min-h-screen">
    <div class="mb-12">
      <h2 class="text-4xl font-black text-white uppercase tracking-tight">Můj Wishlist</h2>
      <p class="text-neutral-400 mt-2">Filmy a seriály, které si chcete pustit později.</p>
    </div>
    
    <div v-if="wishlistData.length > 0" class="flex flex-wrap gap-4">
      <MediaCard 
        v-for="item in wishlistData" 
        :key="item.id" 
        :item="item" 
      />
    </div>
    <div v-else class="text-neutral-500">
      Zatím tu nic nemáte.
    </div>
  </div>
</template>
