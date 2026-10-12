<script setup lang="ts">
import { inject, ref, onMounted, computed, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MediaRow from '../components/MediaRow.vue';
import type { Category, MediaItem } from '../types';

const navigate = inject<Function>('navigate');
const playMedia = inject<Function>('playMedia');

const homeCategories = ref<Category[]>([]);
const isLoading = ref(true);
const featuredIndex = ref(0);
let autoRotateInterval: any = null;

const featuredList = computed<MediaItem[]>(() => {
  if (homeCategories.value.length === 0) return [];
  const allItems = homeCategories.value.flatMap(c => c.items);
  // Unikátní prvních 4 filmů
  const map = new Map<string, MediaItem>();
  allItems.forEach(i => map.set(i.id, i));
  return Array.from(map.values()).slice(0, 4);
});

const featuredMovie = computed<MediaItem | null>(() => {
  if (featuredList.value.length === 0) return null;
  return featuredList.value[featuredIndex.value] || featuredList.value[0];
});

// Rozkoukáno (Continue watching) s průběhem
const continueWatchingItems = computed(() => {
  if (featuredList.value.length < 2) return [];
  return [
    { ...featuredList.value[1], watchProgress: 68, remaining: '38 min zbývá' },
    { ...featuredList.value[2], watchProgress: 35, remaining: '1h 14m zbývá' },
    ...(featuredList.value[3] ? [{ ...featuredList.value[3], watchProgress: 85, remaining: '15 min zbývá' }] : [])
  ];
});

onMounted(async () => {
  try {
    isLoading.value = true;
    homeCategories.value = await invoke<Category[]>('get_home_data');
  } catch (error) {
    console.error('Failed to fetch home data:', error);
  } finally {
    isLoading.value = false;
  }

  // Automatické přepínání hero slideru každých 8 sekund
  autoRotateInterval = setInterval(() => {
    if (featuredList.value.length > 1) {
      featuredIndex.value = (featuredIndex.value + 1) % featuredList.value.length;
    }
  }, 8000);
});

onUnmounted(() => {
  clearInterval(autoRotateInterval);
});

const selectFeatured = (index: number) => {
  featuredIndex.value = index;
  clearInterval(autoRotateInterval);
};
</script>

<template>
  <div>
    <!-- Skeleton loader -->
    <div v-if="isLoading" class="w-full h-[85vh] bg-neutral-850 animate-pulse flex flex-col justify-end p-12">
      <div class="max-w-2xl space-y-4 mb-20">
        <div class="h-4 w-32 bg-neutral-800 rounded"></div>
        <div class="h-14 w-3/4 bg-neutral-800 rounded-lg"></div>
        <div class="h-20 w-full bg-neutral-800 rounded-lg"></div>
        <div class="flex gap-4">
          <div class="h-12 w-32 bg-neutral-800 rounded-md"></div>
          <div class="h-12 w-32 bg-neutral-800 rounded-md"></div>
        </div>
      </div>
    </div>

    <!-- Hero Section -->
    <header v-else-if="featuredMovie" class="relative w-full h-[85vh] flex items-center transition-all duration-700">
      <div class="absolute inset-0 z-0">
        <img
          :key="featuredMovie.id"
          :src="featuredMovie.backdropUrl || featuredMovie.posterUrl"
          alt="Hero Backdrop"
          class="w-full h-full object-cover opacity-80 animate-in fade-in duration-700"
        />
        <div class="absolute inset-0 bg-gradient-to-r from-neutral-900 via-neutral-900/60 to-transparent"></div>
        <div class="absolute inset-0 bg-gradient-to-t from-neutral-900 via-neutral-900/20 to-transparent"></div>
      </div>

      <div class="relative z-10 px-10 md:px-20 max-w-3xl flex flex-col gap-4 mt-20">
        <div class="flex items-center gap-3 text-xs font-bold tracking-widest text-violet-400 uppercase">
          <span>{{ featuredMovie.year }}</span>
          <span class="w-1.5 h-1.5 rounded-full bg-violet-500"></span>
          <span>{{ featuredMovie.genre }}</span>
          <span class="border border-violet-500/50 bg-violet-500/10 px-1.5 py-0.5 rounded text-[10px] text-violet-300">Doporučujeme</span>
        </div>
        
        <h2 class="text-5xl md:text-7xl font-black text-white tracking-tight uppercase shadow-sm">
          {{ featuredMovie.title }}
        </h2>
        
        <p class="text-neutral-300 text-lg line-clamp-3 mt-2 leading-relaxed">
          {{ featuredMovie.description }}
        </p>
        
        <div class="flex items-center gap-4 mt-6">
          <button
            @click="playMedia && playMedia(featuredMovie)"
            class="flex items-center gap-2 px-8 py-3 bg-white text-black font-bold rounded-md hover:bg-neutral-200 hover:scale-105 transition-all duration-300 shadow-[0_0_20px_rgba(255,255,255,0.2)] cursor-pointer"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
              <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
            </svg>
            Přehrát
          </button>
          
          <button
            @click="navigate && navigate('detail', featuredMovie)"
            class="flex items-center gap-2 px-8 py-3 bg-neutral-600/50 backdrop-blur-md text-white font-bold rounded-md hover:bg-neutral-500/50 hover:scale-105 transition-all duration-300 border border-neutral-500/30 cursor-pointer"
          >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z" />
            </svg>
            Více info
          </button>
        </div>
      </div>

      <!-- Tečky pro přepínání Hero slideru -->
      <div v-if="featuredList.length > 1" class="absolute bottom-28 right-12 z-20 flex items-center gap-2.5">
        <button
          v-for="(_, idx) in featuredList"
          :key="idx"
          @click="selectFeatured(idx)"
          class="h-2 rounded-full transition-all duration-300 cursor-pointer"
          :class="featuredIndex === idx ? 'w-8 bg-violet-500' : 'w-2 bg-neutral-600/80 hover:bg-neutral-400'"
          :title="`Přejít na film ${idx + 1}`"
        ></button>
      </div>
    </header>

    <main class="relative z-10 px-8 pb-32 -mt-24 space-y-12">
      <!-- Sekce: Pokračovat ve sledování -->
      <section v-if="continueWatchingItems.length > 0" class="flex flex-col gap-4">
        <h3 class="text-xl font-bold text-neutral-100 flex items-center gap-3">
          <span class="w-1 h-6 bg-fuchsia-500 rounded-full"></span>
          Pokračovat ve sledování
        </h3>
        
        <div class="flex gap-6 overflow-x-auto py-2 -mx-2 px-2 no-scrollbar">
          <div
            v-for="item in continueWatchingItems"
            :key="item.id"
            @click="navigate && navigate('detail', item)"
            class="group relative flex-none w-64 md:w-72 bg-[#161616] border border-neutral-800 rounded-xl overflow-hidden cursor-pointer hover:border-neutral-700 hover:scale-[1.02] transition-all shadow-lg"
          >
            <!-- Náhled -->
            <div class="relative aspect-video w-full overflow-hidden bg-neutral-900">
              <img :src="item.backdropUrl || item.posterUrl" :alt="item.title" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />
              <div class="absolute inset-0 bg-black/30 group-hover:bg-black/10 transition-colors flex items-center justify-center">
                <!-- Tlačítko Přehrát v centru -->
                <button
                  @click.stop="playMedia && playMedia(item)"
                  class="w-12 h-12 rounded-full bg-violet-600/90 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-all transform scale-90 group-hover:scale-100 shadow-xl"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6 ml-0.5">
                    <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
                  </svg>
                </button>
              </div>
              
              <!-- Progress bar -->
              <div class="absolute bottom-0 left-0 right-0 h-1.5 bg-neutral-800">
                <div class="h-full bg-violet-500 rounded-r" :style="{ width: `${item.watchProgress}%` }"></div>
              </div>
            </div>

            <!-- Popis pod náhledem -->
            <div class="p-3.5 flex items-center justify-between">
              <div>
                <h4 class="font-bold text-white text-sm truncate max-w-[180px]">{{ item.title }}</h4>
                <p class="text-[11px] text-neutral-400 mt-0.5">{{ item.remaining }}</p>
              </div>
              <span class="text-[10px] font-mono text-violet-400 bg-violet-500/10 px-2 py-0.5 rounded border border-violet-500/20">
                {{ item.watchProgress }}%
              </span>
            </div>
          </div>
        </div>
      </section>

      <!-- Standardní kategorie filmů a seriálů -->
      <MediaRow 
        v-for="category in homeCategories" 
        :key="category.title" 
        :category="category" 
      />
    </main>
  </div>
</template>
