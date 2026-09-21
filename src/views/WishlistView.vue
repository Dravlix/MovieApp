<script setup lang="ts">
import { wishlistData } from '../data/mockData';
import MediaCard from '../components/MediaCard.vue';

const ambientImage = wishlistData[0]?.backdropUrl || wishlistData[0]?.posterUrl;
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
        <h2 class="text-4xl font-black text-white uppercase tracking-tight drop-shadow-md">Můj Wishlist</h2>
        <p class="text-neutral-400 mt-2 font-medium">Filmy a seriály, které si chcete pustit později.</p>
      </div>
      
      <div v-if="wishlistData.length > 0" class="flex flex-wrap gap-4">
        <MediaCard 
          v-for="item in wishlistData" 
          :key="item.id" 
          :item="item" 
        />
      </div>
      <div v-else class="text-neutral-500 font-medium">
        Zatím tu nic nemáte.
      </div>
    </div>
  </div>
</template>
