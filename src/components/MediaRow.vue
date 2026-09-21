<script setup lang="ts">
import { Category } from '../types';
import MediaCard from './MediaCard.vue';

defineProps<{
  category: Category;
}>();
</script>

<template>
  <section class="flex flex-col gap-4">
    <h3 class="text-xl font-bold text-neutral-100 flex items-center gap-3">
      <span class="w-1 h-6 bg-violet-500 rounded-full"></span>
      {{ category.title }}
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
