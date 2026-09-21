<script setup lang="ts">
import { inject } from 'vue';
import { Category } from '../types';
import MediaCard from '../components/MediaCard.vue';

const props = defineProps<{
  item?: Category; // Předáváme celou kategorii/kolekci v item
}>();

const navigate = inject<Function>('navigate');

// První položka poslouží jako hlavní motiv
const heroItem = props.item?.items[0];
</script>

<template>
  <div v-if="item && heroItem" class="relative min-h-screen pb-32">
    <!-- Tlačítko Zpět -->
    <button 
      @click="navigate && navigate('back')" 
      class="absolute top-24 left-8 md:left-12 z-20 flex items-center gap-2 px-4 py-2 bg-neutral-800/50 hover:bg-neutral-700/70 backdrop-blur-md rounded-full text-white text-sm font-medium transition-all shadow-lg border border-neutral-600/50 hover:scale-105"
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-4 h-4">
        <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
      </svg>
      Zpět
    </button>

    <!-- Hero Section stylizovaná podobně jako Home (pro kolekci) -->
    <header class="relative w-full h-[75vh] flex items-center">
      <div class="absolute inset-0 z-0">
        <img :src="heroItem.backdropUrl || heroItem.posterUrl" alt="Hero Backdrop" class="w-full h-full object-cover opacity-60" />
        <div class="absolute inset-0 bg-gradient-to-r from-neutral-900 via-neutral-900/70 to-transparent"></div>
        <div class="absolute inset-0 bg-gradient-to-t from-neutral-900 via-neutral-900/30 to-transparent"></div>
      </div>

      <div class="relative z-10 px-10 md:px-20 max-w-3xl flex flex-col gap-4 mt-20">
        <div class="flex items-center gap-3 text-sm font-bold text-violet-400">
          <span>{{ item.title }}</span>
        </div>
        
        <h2 class="text-5xl md:text-7xl font-black text-white tracking-tight uppercase shadow-sm">
          {{ heroItem.title }}
        </h2>
        
        <p class="text-neutral-300 text-lg line-clamp-3 mt-2 leading-relaxed">
          {{ heroItem.description || 'Pusťte se do dalšího dílu v této kolekci.' }}
        </p>
        
        <div class="flex items-center gap-4 mt-6">
          <button class="flex items-center gap-2 px-8 py-3 bg-white text-black font-bold rounded-md hover:bg-neutral-200 hover:scale-105 transition-all duration-300 shadow-[0_0_20px_rgba(255,255,255,0.2)]">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
              <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
            </svg>
            Přehrát 1. díl
          </button>
        </div>
      </div>
    </header>

    <!-- Seznam všech filmů v kolekci -->
    <main class="relative z-10 px-8 pt-8 md:px-20">
      <h3 class="text-2xl font-bold text-white mb-6">Vše v této kolekci</h3>
      <div class="flex flex-wrap gap-x-6 gap-y-12 py-8 pr-12 md:pr-32">
        <MediaCard 
          v-for="subItem in item.items" 
          :key="subItem.id" 
          :item="subItem" 
        />
      </div>
    </main>

  </div>
</template>
