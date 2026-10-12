<script setup lang="ts">
import { ref, onMounted, computed, inject } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MediaCard from '../components/MediaCard.vue';
import type { MediaItem } from '../types';

const navigate = inject<Function>('navigate');
const wishlistData = ref<MediaItem[]>([]);
const selectedFilter = ref<'all' | 'movie' | 'series'>('all');

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

const filteredWishlist = computed(() => {
  if (selectedFilter.value === 'all') return wishlistData.value;
  return wishlistData.value.filter(item => item.type === selectedFilter.value);
});
</script>

<template>
  <div class="relative min-h-screen pb-32">
    <!-- Fixní Ambientní Glow Efekt -->
    <div class="fixed top-0 left-0 w-full h-screen z-0 overflow-hidden pointer-events-none">
      <img v-if="ambientImage" :src="ambientImage" class="w-full h-full object-cover opacity-20 blur-[150px] saturate-150 scale-125" />
      <div class="absolute inset-0 bg-gradient-to-b from-neutral-900/40 via-neutral-900/80 to-neutral-900/95"></div>
    </div>

    <div class="relative z-10 pt-32 px-8">
      <!-- Nadpis a filtry -->
      <div class="flex flex-col sm:flex-row sm:items-end justify-between gap-6 mb-8">
        <div>
          <h2 class="text-4xl font-black text-white uppercase tracking-tight drop-shadow-md flex items-center gap-3">
            Můj Wishlist
            <span v-if="wishlistData.length > 0" class="text-sm font-bold bg-violet-600/30 text-violet-300 border border-violet-500/40 px-2.5 py-1 rounded-full font-mono">
              {{ wishlistData.length }}
            </span>
          </h2>
          <p class="text-neutral-400 mt-2 font-medium">Filmy a seriály, které si chcete pustit později.</p>
        </div>

        <!-- Filtry typu -->
        <div v-if="wishlistData.length > 0" class="flex items-center gap-2 bg-neutral-800/60 border border-neutral-700/60 rounded-xl p-1 backdrop-blur-md">
          <button
            @click="selectedFilter = 'all'"
            class="px-3 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer"
            :class="selectedFilter === 'all' ? 'bg-violet-600 text-white shadow-sm' : 'text-neutral-400 hover:text-white'"
          >
            Vše
          </button>
          <button
            @click="selectedFilter = 'movie'"
            class="px-3 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer"
            :class="selectedFilter === 'movie' ? 'bg-violet-600 text-white shadow-sm' : 'text-neutral-400 hover:text-white'"
          >
            Filmy
          </button>
          <button
            @click="selectedFilter = 'series'"
            class="px-3 py-1 rounded-lg text-xs font-bold transition-all cursor-pointer"
            :class="selectedFilter === 'series' ? 'bg-violet-600 text-white shadow-sm' : 'text-neutral-400 hover:text-white'"
          >
            Seriály
          </button>
        </div>
      </div>
      
      <!-- Seznam karet -->
      <TransitionGroup 
        v-if="filteredWishlist.length > 0" 
        name="list" 
        tag="div" 
        class="flex flex-wrap gap-x-6 gap-y-12 py-6 pr-12 md:pr-32"
      >
        <MediaCard 
          v-for="item in filteredWishlist" 
          :key="item.id" 
          :item="item" 
          @wishlistUpdated="handleWishlistUpdate"
        />
      </TransitionGroup>

      <!-- Prázdný stav -->
      <div v-else class="py-24 flex flex-col items-center justify-center text-center max-w-md mx-auto">
        <div class="w-16 h-16 rounded-full bg-neutral-800/80 border border-neutral-700 flex items-center justify-center mb-4 text-violet-400">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.8" stroke="currentColor" class="w-8 h-8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M17.593 3.322c1.1.128 1.907 1.077 1.907 2.185V21L12 17.25 4.5 21V5.507c0-1.108.806-2.057 1.907-2.185a48.507 48.507 0 0111.186 0z" />
          </svg>
        </div>
        <h3 class="text-xl font-bold text-white mb-2">Váš Wishlist je prázdný</h3>
        <p class="text-neutral-400 text-sm leading-relaxed mb-6">
          Zatím jste si neuložili žádné tituly. Klikněte na ikonu plus (+) na libovolném filmu a přidejte si jej sem na později.
        </p>
        <button
          @click="navigate && navigate('movies')"
          class="px-6 py-2.5 bg-violet-600 hover:bg-violet-500 font-bold text-white text-sm rounded-xl transition-all shadow-lg shadow-violet-600/30 cursor-pointer"
        >
          Prozkoumat filmy
        </button>
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
