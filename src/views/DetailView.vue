<script setup lang="ts">
import { inject } from 'vue';
import { MediaItem } from '../types';

defineProps<{
  item?: MediaItem;
}>();

const navigate = inject<Function>('navigate');
</script>

<template>
  <div v-if="item" class="relative w-full min-h-screen flex items-center bg-neutral-900">
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

    <!-- Background Image s přechody do ztracena -->
    <div class="absolute inset-0 z-0">
      <img :src="item.backdropUrl || item.posterUrl" alt="Hero Backdrop" class="w-full h-full object-cover opacity-70" />
      <div class="absolute inset-0 bg-gradient-to-r from-neutral-900 via-neutral-900/80 to-transparent md:w-[70%]"></div>
      <div class="absolute inset-0 bg-gradient-to-t from-neutral-900 via-neutral-900/20 to-transparent"></div>
    </div>

    <!-- Content -->
    <div class="relative z-10 px-8 md:px-20 w-full flex flex-col md:flex-row gap-12 mt-10 md:mt-24">
      
      <!-- Levý sloupec (Nadpis, Meta, Popis, Tlačítka) -->
      <div class="flex-grow max-w-3xl flex flex-col gap-6">
        <h2 class="text-5xl md:text-7xl font-black text-white tracking-tight uppercase shadow-sm">
          {{ item.title }}
        </h2>
        
        <!-- Metadata podle předlohy (Rok, Délka, HD, Titulky, Věk, Upozornění) -->
        <div class="flex flex-col gap-2">
           <div class="flex items-center gap-3 text-base font-semibold text-neutral-300">
             <span>{{ item.year }}</span>
             <span v-if="item.duration">{{ item.duration }}</span>
             <span class="border border-neutral-500 px-1.5 py-0.5 rounded-[3px] text-[10px] text-neutral-300 font-bold">HD</span>
             <!-- Ikonka pro titulky / audio popisy -->
             <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5 text-neutral-300">
               <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z" />
             </svg>
           </div>
           <div v-if="item.ageRating || item.contentWarnings" class="flex items-center gap-3 text-sm font-medium text-neutral-300">
             <span v-if="item.ageRating" class="border border-neutral-500 px-1.5 py-0.5 rounded-[3px] text-[12px] text-neutral-200">{{ item.ageRating }}</span>
             <span v-if="item.contentWarnings">{{ item.contentWarnings }}</span>
           </div>
        </div>
        
        <p class="text-white text-lg md:text-xl line-clamp-4 leading-relaxed max-w-2xl font-medium">
          {{ item.description || 'Popis k tomuto titulu se momentálně připravuje. Brzy zde najdete více informací o příběhu, hercích a tvůrcích.' }}
        </p>
        
        <div class="flex items-center gap-4 mt-2">
          <button class="flex items-center gap-2 px-8 py-4 bg-white text-black font-bold rounded-md hover:bg-neutral-200 hover:scale-105 transition-all duration-300 shadow-[0_0_20px_rgba(255,255,255,0.2)]">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7">
              <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
            </svg>
            Přehrát
          </button>
          
          <button class="flex items-center gap-2 px-8 py-4 bg-neutral-800/80 backdrop-blur-md text-white font-bold rounded-md hover:bg-neutral-700/80 hover:scale-105 transition-all duration-300 border border-neutral-600">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-7 h-7">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
            Do wishlistu
          </button>
        </div>
      </div>

      <!-- Pravý sloupec (Hrají, Žánry, Vlastnosti) podle předlohy -->
      <div class="flex flex-col gap-4 text-[15px] w-full md:w-80 md:mt-24 shrink-0">
         <div v-if="item.cast && item.cast.length" class="leading-snug">
            <span class="text-neutral-500">Hrají: </span> 
            <span class="text-neutral-200">{{ item.cast.join(', ') }}<i v-if="item.cast.length >= 3" class="italic">, a další</i></span>
         </div>
         <div v-if="item.genres && item.genres.length" class="leading-snug">
            <span class="text-neutral-500">Žánry: </span> 
            <span class="text-neutral-200">{{ item.genres.join(', ') }}</span>
         </div>
         <div v-if="item.moods && item.moods.length" class="leading-snug">
            <span class="text-neutral-500">Tento pořad je: </span> 
            <span class="text-neutral-200">{{ item.moods.join(', ') }}</span>
         </div>
      </div>

    </div>
  </div>
</template>
