<script setup lang="ts">
import { inject, ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { MediaItem } from '../types';

const props = defineProps<{
  item: MediaItem;
}>();

const emit = defineEmits(['wishlistUpdated']);

const navigate = inject<Function>('navigate');

const inWishlist = ref(false);
const isHovered = ref(false);
const isAnimatingOut = ref(false);
let hoverTimeout: any = null;

onMounted(async () => {
  try {
    inWishlist.value = await invoke<boolean>('check_wishlist', { id: props.item.id });
  } catch (e) {
    console.error(e);
  }
});

const toggleWishlist = async () => {
  try {
    inWishlist.value = await invoke<boolean>('toggle_wishlist', { id: props.item.id });
    emit('wishlistUpdated', props.item.id, inWishlist.value);
  } catch (e) {
    console.error(e);
  }
};

const handleMouseEnter = () => {
  clearTimeout(hoverTimeout);
  isHovered.value = true;
  isAnimatingOut.value = false;
};

const handleMouseLeave = () => {
  isHovered.value = false;
  isAnimatingOut.value = true;
  hoverTimeout = setTimeout(() => {
    isAnimatingOut.value = false;
  }, 300); // 300ms corresponds to transition duration
};
</script>

<template>
  <div 
    @click="navigate && navigate('detail', item)" 
    @mouseenter="handleMouseEnter"
    @mouseleave="handleMouseLeave"
    class="group relative flex-none w-40 md:w-48 aspect-[2/3] cursor-pointer snap-start transition-all"
    :class="[isHovered ? 'z-50' : (isAnimatingOut ? 'z-40' : 'z-10')]"
  >
    <!-- Expandující Hover Karta -->
    <div class="absolute top-0 left-0 h-full w-full bg-[#141414] rounded-xl transition-all duration-300 ease-out shadow-lg overflow-hidden flex ring-1 ring-transparent group-hover:w-[145%] group-hover:-translate-y-2 group-hover:scale-110 group-hover:shadow-[0_20px_50px_rgba(0,0,0,0.6)] group-hover:ring-neutral-700">
      
      <!-- Levá část: Plakát -->
      <div class="w-40 md:w-48 h-full flex-none relative">
        <img :src="item.posterUrl" :alt="item.title" class="w-full h-full object-cover" />
        <!-- Přechody a nadpis -->
        <div class="absolute inset-0 bg-gradient-to-r from-transparent via-transparent to-[#141414] opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
        <div class="absolute inset-0 bg-gradient-to-t from-[#141414] via-black/40 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end p-3 z-10">
          <h4 class="text-sm md:text-base font-black text-white leading-tight drop-shadow-[0_2px_4px_rgba(0,0,0,0.8)]">
            {{ item.title }}
          </h4>
        </div>
      </div>
      
      <!-- Pravá část: Obsah -->
      <div class="flex-grow opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-100 relative overflow-hidden">
        <div class="absolute inset-0 p-3 flex flex-col justify-center items-center gap-3">
          <div class="flex flex-col items-center gap-1 text-[10px] font-bold text-neutral-400">
            <span class="text-violet-400 uppercase text-center leading-tight">{{ item.genre }}</span>
            <span>{{ item.year }}</span>
            <span class="border border-neutral-600 px-1 py-0.5 rounded-sm text-[9px] text-white bg-neutral-800 mt-1">HD</span>
          </div>

          <div class="flex flex-col gap-2 mt-2">
            <!-- Přehrát -->
            <button class="w-8 h-8 flex items-center justify-center bg-white rounded-full hover:bg-neutral-300 hover:scale-110 transition-all shadow-xl text-black">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4 ml-0.5">
                <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
              </svg>
            </button>
            <!-- Přidat -->
            <button @click.stop="toggleWishlist" class="w-8 h-8 flex items-center justify-center bg-neutral-800/80 border border-neutral-500 text-white rounded-full hover:border-white hover:scale-110 transition-all backdrop-blur-sm">
              <svg v-if="!inWishlist" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4 text-violet-500">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
              </svg>
            </button>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>
