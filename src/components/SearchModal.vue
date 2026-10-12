<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { MediaItem } from '../types';

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits(['close', 'select']);

const searchQuery = ref('');
const inputRef = ref<HTMLInputElement | null>(null);
const allItems = ref<MediaItem[]>([]);
const selectedIndex = ref(0);

const loadAllMedia = async () => {
  try {
    const [movies, series] = await Promise.all([
      invoke<MediaItem[]>('get_movies'),
      invoke<MediaItem[]>('get_series'),
    ]);
    // Sjednotíme do jednoho pole unikátních položek
    const map = new Map<string, MediaItem>();
    [...movies, ...series].forEach(item => map.set(item.id, item));
    allItems.value = Array.from(map.values());
  } catch (error) {
    console.error('Chyba při načítání medií pro vyhledávání:', error);
  }
};

onMounted(() => {
  loadAllMedia();
});

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    searchQuery.value = '';
    selectedIndex.value = 0;
    nextTick(() => {
      inputRef.value?.focus();
    });
  }
});

const filteredResults = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  if (!query) return [];

  return allItems.value.filter(item => {
    const inTitle = item.title.toLowerCase().includes(query);
    const inDesc = item.description?.toLowerCase().includes(query) ?? false;
    const inGenre = item.genre?.toLowerCase().includes(query) ?? false;
    const inGenres = item.genres?.some(g => g.toLowerCase().includes(query)) ?? false;
    const inCast = item.cast?.some(c => c.toLowerCase().includes(query)) ?? false;
    return inTitle || inDesc || inGenre || inGenres || inCast;
  });
});

const handleKeyDown = (e: KeyboardEvent) => {
  if (!props.isOpen) return;

  if (e.key === 'Escape') {
    e.preventDefault();
    e.stopPropagation();
    emit('close');
  } else if (e.key === 'ArrowDown') {
    e.preventDefault();
    if (filteredResults.value.length > 0) {
      selectedIndex.value = (selectedIndex.value + 1) % filteredResults.value.length;
    }
  } else if (e.key === 'ArrowUp') {
    e.preventDefault();
    if (filteredResults.value.length > 0) {
      selectedIndex.value = (selectedIndex.value - 1 + filteredResults.value.length) % filteredResults.value.length;
    }
  } else if (e.key === 'Enter') {
    e.preventDefault();
    if (filteredResults.value[selectedIndex.value]) {
      selectItem(filteredResults.value[selectedIndex.value]);
    }
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown, true);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown, true);
});

const selectItem = (item: MediaItem) => {
  emit('select', item);
  emit('close');
};
</script>

<template>
  <Transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="isOpen"
      @click="emit('close')"
      class="fixed inset-0 z-50 bg-black/75 backdrop-blur-md flex items-start justify-center pt-20 px-4"
    >
      <div
        @click.stop
        class="bg-[#181818] border border-neutral-700/80 rounded-2xl w-full max-w-2xl shadow-[0_25px_60px_rgba(0,0,0,0.8)] overflow-hidden flex flex-col max-h-[75vh] animate-in fade-in zoom-in-95 duration-150"
      >
        <!-- Vyhledávací pole -->
        <div class="relative flex items-center px-5 py-4 border-b border-neutral-800">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6 text-violet-400 shrink-0">
            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
          </svg>
          <input
            ref="inputRef"
            v-model="searchQuery"
            type="text"
            placeholder="Hledat filmy, seriály, žánry nebo herce..."
            class="w-full bg-transparent border-none text-white text-lg placeholder-neutral-500 ml-3 focus:outline-none focus:ring-0"
          />
          <span class="text-xs bg-neutral-800 text-neutral-400 border border-neutral-700 px-2 py-1 rounded font-mono shrink-0">
            ESC
          </span>
        </div>

        <!-- Výsledky -->
        <div class="overflow-y-auto p-3 flex flex-col gap-1.5 divide-y divide-neutral-800/40">
          <div v-if="searchQuery.trim() === ''" class="py-12 text-center text-neutral-500">
            <p class="text-sm">Začněte psát název filmu, žánr nebo jméno herce...</p>
            <div class="flex items-center justify-center gap-2 mt-3 text-xs text-neutral-600">
              <span class="bg-neutral-800/80 px-2 py-1 rounded">Spider-Man</span>
              <span class="bg-neutral-800/80 px-2 py-1 rounded">Sci-Fi</span>
              <span class="bg-neutral-800/80 px-2 py-1 rounded">Matrix</span>
            </div>
          </div>

          <div v-else-if="filteredResults.length === 0" class="py-12 text-center text-neutral-500">
            <p class="text-base text-neutral-400">Nebyly nalezeny žádné výsledky pro "{{ searchQuery }}"</p>
            <p class="text-xs text-neutral-600 mt-1">Zkontrolujte překlepy nebo zkuste jiný výraz.</p>
          </div>

          <div
            v-else
            v-for="(item, idx) in filteredResults"
            :key="item.id"
            @click="selectItem(item)"
            @mouseenter="selectedIndex = idx"
            class="flex items-center gap-4 p-2.5 rounded-xl cursor-pointer transition-colors pt-2"
            :class="selectedIndex === idx ? 'bg-violet-600/20 text-white' : 'hover:bg-neutral-800/60 text-neutral-300'"
          >
            <!-- Plakát náhled -->
            <img
              :src="item.posterUrl"
              :alt="item.title"
              class="w-12 h-16 object-cover rounded-lg shadow shrink-0 bg-neutral-800"
            />

            <!-- Info -->
            <div class="flex-grow min-w-0">
              <div class="flex items-center gap-2">
                <h4 class="font-bold text-white text-base truncate">
                  {{ item.title }}
                </h4>
                <span
                  class="text-[10px] px-1.5 py-0.5 rounded font-bold uppercase tracking-wider shrink-0"
                  :class="item.type === 'series' ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/30' : 'bg-violet-500/20 text-violet-300 border border-violet-500/30'"
                >
                  {{ item.type === 'series' ? 'Seriál' : 'Film' }}
                </span>
              </div>

              <div class="flex items-center gap-2 text-xs text-neutral-400 mt-0.5">
                <span>{{ item.year }}</span>
                <span>•</span>
                <span>{{ item.genre }}</span>
                <span v-if="item.duration">•</span>
                <span v-if="item.duration">{{ item.duration }}</span>
              </div>

              <p v-if="item.cast && item.cast.length" class="text-xs text-neutral-500 truncate mt-1">
                Hrají: {{ item.cast.join(', ') }}
              </p>
            </div>

            <!-- Šipka / Akce -->
            <div class="shrink-0 text-neutral-500 pr-2">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
              </svg>
            </div>
          </div>
        </div>

        <!-- Patička -->
        <div class="bg-neutral-900/60 border-t border-neutral-800 px-5 py-2.5 flex items-center justify-between text-xs text-neutral-500">
          <div class="flex items-center gap-4">
            <span class="flex items-center gap-1">
              <kbd class="bg-neutral-800 border border-neutral-700 px-1.5 py-0.5 rounded text-[10px]">↑</kbd>
              <kbd class="bg-neutral-800 border border-neutral-700 px-1.5 py-0.5 rounded text-[10px]">↓</kbd>
              pohyb
            </span>
            <span class="flex items-center gap-1">
              <kbd class="bg-neutral-800 border border-neutral-700 px-1.5 py-0.5 rounded text-[10px]">Enter</kbd>
              otevřít
            </span>
          </div>
          <span>{{ filteredResults.length }} nalezeno</span>
        </div>
      </div>
    </div>
  </Transition>
</template>
