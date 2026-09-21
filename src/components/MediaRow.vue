<script setup lang="ts">
import { inject } from 'vue';
import { Category } from '../types';
import MediaCard from './MediaCard.vue';

defineProps<{
  category: Category;
}>();

const navigate = inject<Function>('navigate');
</script>

<template>
  <section class="flex flex-col gap-4">
    <h3 
      @click="navigate && navigate('collection', category)"
      class="text-xl font-bold text-neutral-100 flex items-center gap-3 cursor-pointer hover:text-white group w-max transition-colors"
    >
      <span class="w-1 h-6 bg-violet-500 rounded-full group-hover:bg-violet-400 transition-colors"></span>
      {{ category.title }}
      <!-- Šipka ukazuje, že je to rozklikávací -->
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-5 h-5 text-neutral-500 group-hover:text-violet-400 group-hover:translate-x-1 transition-all">
        <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
      </svg>
    </h3>
    
    <div class="flex gap-4 overflow-x-auto py-8 px-4 pr-48 -mx-4 no-scrollbar scroll-smooth snap-x">
      <MediaCard 
        v-for="item in category.items" 
        :key="item.id" 
        :item="item" 
      />
    </div>
  </section>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}
.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
