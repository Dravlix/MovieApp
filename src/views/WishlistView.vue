<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MediaCard from '../components/MediaCard.vue';
import type { MediaItem } from '../types';

const wishlistData = ref<MediaItem[]>([]);

const ambientImage = computed(() => {
  return wishlistData.value[0]?.backdropUrl || wishlistData.value[0]?.posterUrl;
});

onMounted(async () => {
  wishlistData.value = await invoke<MediaItem[]>('get_wishlist');
});

const handleWishlistUpdate = (id: string, inWishlist: boolean) => {
  if (!inWishlist) {
    wishlistData.value = wishlistData.value.filter(item => item.id !== id);
  }
};
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
      
      <TransitionGroup 
        v-if="wishlistData.length > 0" 
        name="list" 
        tag="div" 
        class="flex flex-wrap gap-x-6 gap-y-12 py-8 pr-12 md:pr-32"
      >
        <MediaCard 
          v-for="item in wishlistData" 
          :key="item.id" 
          :item="item" 
          @wishlistUpdated="handleWishlistUpdate"
        />
      </TransitionGroup>
      <div v-else class="text-neutral-500 font-medium">
        Zatím tu nic nemáte.
      </div>
    </div>
  </div>
</template>

<style scoped>
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: scale(0.9);
}
.list-leave-active {
  position: absolute;
}
</style>
