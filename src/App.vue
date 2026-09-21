<script setup lang="ts">
import { ref, computed, provide } from 'vue'
import HomeView from './views/HomeView.vue'
import MoviesView from './views/MoviesView.vue'
import SeriesView from './views/SeriesView.vue'
import WishlistView from './views/WishlistView.vue'
import DetailView from './views/DetailView.vue'
import CollectionsView from './views/CollectionsView.vue'

// Jednoduchý stavový router s podporou parametrů a historie
const currentRoute = ref('home')
const currentParams = ref<any>(null)
const routeHistory = ref<string[]>(['home'])

const navigate = (route: string, params: any = null) => {
  if (route === 'back') {
    if (routeHistory.value.length > 1) {
      routeHistory.value.pop() // odstraníme aktuální
      currentRoute.value = routeHistory.value[routeHistory.value.length - 1]
      currentParams.value = null // reset parametrů pro návrat do seznamu
    } else {
      currentRoute.value = 'home'
    }
  } else {
    currentRoute.value = route
    currentParams.value = params
    if (routeHistory.value[routeHistory.value.length - 1] !== route) {
      routeHistory.value.push(route)
    }
  }
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

// Zpřístupníme navigační funkci pro všechny potomky (kartičky atd.)
provide('navigate', navigate)

const routes = {
  home: HomeView,
  movies: MoviesView,
  series: SeriesView,
  wishlist: WishlistView,
  detail: DetailView,
  collections: CollectionsView
}

const currentView = computed(() => routes[currentRoute.value as keyof typeof routes] || HomeView)
</script>

<template>
  <div class="min-h-screen bg-neutral-900 text-neutral-100 font-sans overflow-x-hidden selection:bg-violet-500/30">
    
    <!-- Navbar -->
    <nav class="fixed top-0 w-full z-50 px-8 py-4 flex items-center justify-between bg-neutral-900/70 backdrop-blur-md border-b border-neutral-800/50 transition-all duration-300">
      <div class="flex items-center gap-10">
        <h1 
          @click="navigate('home')"
          class="text-3xl font-black text-violet-500 tracking-tighter uppercase drop-shadow-[0_0_15px_rgba(139,92,246,0.3)] cursor-pointer"
        >
          YourMovies
        </h1>
        <ul class="hidden md:flex gap-6 text-sm font-medium text-neutral-300">
          <li @click="navigate('home')" :class="{'text-white font-bold': currentRoute === 'home'}" class="hover:text-white cursor-pointer transition-colors">Domů</li>
          <li @click="navigate('movies')" :class="{'text-white font-bold': currentRoute === 'movies'}" class="hover:text-white cursor-pointer transition-colors">Filmy</li>
          <li @click="navigate('series')" :class="{'text-white font-bold': currentRoute === 'series'}" class="hover:text-white cursor-pointer transition-colors">Seriály</li>
          <li @click="navigate('collections')" :class="{'text-white font-bold': currentRoute === 'collections'}" class="hover:text-white cursor-pointer transition-colors">Kolekce</li>
          <li @click="navigate('wishlist')" :class="{'text-violet-300 font-bold': currentRoute === 'wishlist'}" class="cursor-pointer transition-colors text-violet-400 hover:text-violet-300">Wishlist</li>
        </ul>
      </div>
      <div class="flex items-center gap-6">
        <!-- Search Icon Placeholder -->
        <button class="text-neutral-300 hover:text-white transition-colors">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
          </svg>
        </button>
        <!-- Profile Avatar Placeholder -->
        <div class="w-9 h-9 rounded-md bg-gradient-to-tr from-violet-500 to-fuchsia-600 cursor-pointer shadow-lg"></div>
      </div>
    </nav>

    <!-- Zobrazení aktuální stránky -->
    <component :is="currentView" :item="currentParams" />

  </div>
</template>