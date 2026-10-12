<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import MediaRow from '../components/MediaRow.vue';
import MediaCard from '../components/MediaCard.vue';
import type { MediaItem, Category } from '../types';

const movies = ref<MediaItem[]>([]);
const selectedGenre = ref('Vše');
const sortBy = ref<'newest' | 'oldest' | 'title_asc' | 'title_desc'>('newest');
const viewMode = ref<'grid' | 'rows'>('grid');

onMounted(async () => {
  movies.value = await invoke<MediaItem[]>('get_movies');
});

// Dynamický seznam všech existujících žánrů
const availableGenres = computed(() => {
  const set = new Set<string>();
  movies.value.forEach(m => {
    if (m.genre) set.add(m.genre);
    if (m.genres) m.genres.forEach(g => set.add(g));
  });
  return ['Vše', ...Array.from(set)];
});

// Filtrované a seřazené filmy
const processedMovies = computed(() => {
  let list = [...movies.value];

  // Filtrování žánru
  if (selectedGenre.value !== 'Vše') {
    list = list.filter(m =>
      m.genre?.toLowerCase() === selectedGenre.value.toLowerCase() ||
      m.genres?.some(g => g.toLowerCase() === selectedGenre.value.toLowerCase())
    );
  }

  // Řazení
  list.sort((a, b) => {
    if (sortBy.value === 'newest') return b.year - a.year;
    if (sortBy.value === 'oldest') return a.year - b.year;
    if (sortBy.value === 'title_asc') return a.title.localeCompare(b.title);
    if (sortBy.value === 'title_desc') return b.title.localeCompare(a.title);
    return 0;
  });

  return list;
});

const movieCategories = computed<Category[]>(() => {
  if (processedMovies.value.length === 0) return [];
  return [{
    title: selectedGenre.value === 'Vše' ? 'Všechny filmy' : `Filmy: ${selectedGenre.value}`,
    items: processedMovies.value
  }];
});

const ambientImage = computed(() => {
  return processedMovies.value[0]?.backdropUrl || processedMovies.value[0]?.posterUrl;
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
      <!-- Nadpis a ovládací panel -->
      <div class="flex flex-col md:flex-row md:items-end justify-between gap-6 mb-8">
        <div>
          <h2 class="text-4xl font-black text-white uppercase tracking-tight drop-shadow-md">Filmy</h2>
          <p class="text-neutral-400 mt-2 font-medium">Objevte ty nejlepší celovečerní filmy z naší nabídky.</p>
        </div>

        <!-- Ovládací prvky: Řazení a Mřížka/Řádky -->
        <div class="flex items-center gap-4">
          <!-- Řazení -->
          <div class="flex items-center gap-2 bg-neutral-800/60 border border-neutral-700/60 rounded-xl px-3 py-1.5 backdrop-blur-md">
            <span class="text-xs text-neutral-400 font-medium">Řadit:</span>
            <select
              v-model="sortBy"
              class="bg-transparent text-xs text-white font-bold outline-none cursor-pointer"
            >
              <option value="newest" class="bg-neutral-800 text-white">Nejnovější</option>
              <option value="oldest" class="bg-neutral-800 text-white">Nejstarší</option>
              <option value="title_asc" class="bg-neutral-800 text-white">Abecedně (A - Z)</option>
              <option value="title_desc" class="bg-neutral-800 text-white">Abecedně (Z - A)</option>
            </select>
          </div>

          <!-- Přepínač Grid / Rows -->
          <div class="flex items-center bg-neutral-800/60 border border-neutral-700/60 rounded-xl p-1 backdrop-blur-md">
            <button
              @click="viewMode = 'grid'"
              class="p-1.5 rounded-lg transition-colors"
              :class="viewMode === 'grid' ? 'bg-violet-600 text-white shadow-sm' : 'text-neutral-400 hover:text-white'"
              title="Mřížka"
            >
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
              </svg>
            </button>
            <button
              @click="viewMode = 'rows'"
              class="p-1.5 rounded-lg transition-colors"
              :class="viewMode === 'rows' ? 'bg-violet-600 text-white shadow-sm' : 'text-neutral-400 hover:text-white'"
              title="Řádky"
            >
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
              </svg>
            </button>
          </div>
        </div>
      </div>

      <!-- Žánrové filtry (Pills) -->
      <div class="flex items-center gap-2 overflow-x-auto pb-4 mb-8 no-scrollbar">
        <button
          v-for="genre in availableGenres"
          :key="genre"
          @click="selectedGenre = genre"
          class="px-4 py-1.5 rounded-full text-xs font-bold transition-all shrink-0 cursor-pointer"
          :class="selectedGenre === genre ? 'bg-violet-600 text-white shadow-md shadow-violet-600/30' : 'bg-neutral-800/70 hover:bg-neutral-700/80 text-neutral-300 hover:text-white border border-neutral-700/40'"
        >
          {{ genre }}
        </button>
        <span class="text-xs text-neutral-500 font-mono ml-auto hidden sm:inline shrink-0">
          {{ processedMovies.length }} filmů
        </span>
      </div>

      <!-- Prázdný stav -->
      <div v-if="processedMovies.length === 0" class="py-20 text-center text-neutral-500">
        <p class="text-lg text-neutral-400">V žánru "{{ selectedGenre }}" zatím nemáte žádné filmy.</p>
        <button @click="selectedGenre = 'Vše'" class="mt-4 px-4 py-2 bg-neutral-800 hover:bg-neutral-700 text-violet-400 rounded-lg text-sm font-semibold transition-colors">
          Zobrazit všechny žánry
        </button>
      </div>

      <!-- Mřížka (Grid View) -->
      <div
        v-else-if="viewMode === 'grid'"
        class="flex flex-wrap gap-x-6 gap-y-12 py-4 pr-12 md:pr-32"
      >
        <MediaCard
          v-for="movie in processedMovies"
          :key="movie.id"
          :item="movie"
        />
      </div>

      <!-- Řádky (Row View) -->
      <div v-else class="space-y-12">
        <MediaRow 
          v-for="category in movieCategories" 
          :key="category.title" 
          :category="category" 
        />
      </div>
    </div>
  </div>
</template>
