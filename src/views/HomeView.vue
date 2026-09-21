<script setup lang="ts">
import { inject, ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MediaRow from '../components/MediaRow.vue';
import type { Category, MediaItem } from '../types';

const navigate = inject<Function>('navigate');

const homeCategories = ref<Category[]>([]);
const featuredMovie = ref<MediaItem | null>(null);

onMounted(async () => {
  try {
    homeCategories.value = await invoke<Category[]>('get_home_data');
    if (homeCategories.value.length > 0 && homeCategories.value[0].items.length > 0) {
      featuredMovie.value = homeCategories.value[0].items[0];
    }
  } catch (error) {
    console.error('Failed to fetch home data:', error);
  }
});
</script>

<template>
  <div>
    <!-- Hero Section -->
    <header v-if="featuredMovie" class="relative w-full h-[85vh] flex items-center">
      <div class="absolute inset-0 z-0">
        <img :src="featuredMovie.backdropUrl" alt="Hero Backdrop" class="w-full h-full object-cover opacity-80" />
        <div class="absolute inset-0 bg-gradient-to-r from-neutral-900 via-neutral-900/60 to-transparent"></div>
        <div class="absolute inset-0 bg-gradient-to-t from-neutral-900 via-neutral-900/20 to-transparent"></div>
      </div>

      <div class="relative z-10 px-10 md:px-20 max-w-3xl flex flex-col gap-4 mt-20">
        <div class="flex items-center gap-3 text-xs font-bold tracking-widest text-violet-400 uppercase">
          <span>{{ featuredMovie.year }}</span>
          <span class="w-1.5 h-1.5 rounded-full bg-violet-500"></span>
          <span>{{ featuredMovie.genre }}</span>
        </div>
        
        <h2 class="text-5xl md:text-7xl font-black text-white tracking-tight uppercase shadow-sm">
          {{ featuredMovie.title }}
        </h2>
        
        <p class="text-neutral-300 text-lg line-clamp-3 mt-2 leading-relaxed">
          {{ featuredMovie.description }}
        </p>
        
        <div class="flex items-center gap-4 mt-6">
          <button class="flex items-center gap-2 px-8 py-3 bg-white text-black font-bold rounded-md hover:bg-neutral-200 hover:scale-105 transition-all duration-300 shadow-[0_0_20px_rgba(255,255,255,0.2)]">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
              <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
            </svg>
            Přehrát
          </button>
          
          <button @click="navigate && navigate('detail', featuredMovie)" class="flex items-center gap-2 px-8 py-3 bg-neutral-600/50 backdrop-blur-md text-white font-bold rounded-md hover:bg-neutral-500/50 hover:scale-105 transition-all duration-300 border border-neutral-500/30">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
            </svg>
            Více info
          </button>
        </div>
      </div>
    </header>

    <main class="relative z-10 px-8 pb-32 -mt-24 space-y-12">
      <MediaRow 
        v-for="category in homeCategories" 
        :key="category.title" 
        :category="category" 
      />
    </main>
  </div>
</template>
