<script setup lang="ts">
import { ref, computed, provide, onMounted, onUnmounted } from 'vue'
import HomeView from './views/HomeView.vue'
import MoviesView from './views/MoviesView.vue'
import SeriesView from './views/SeriesView.vue'
import WishlistView from './views/WishlistView.vue'
import DetailView from './views/DetailView.vue'
import CollectionsView from './views/CollectionsView.vue'
import CollectionView from './views/CollectionView.vue'
import SettingsView from './views/SettingsView.vue'

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
  collections: CollectionsView,
  collection: CollectionView,
  settings: SettingsView
}

const currentView = computed(() => routes[currentRoute.value as keyof typeof routes] || HomeView)

// Stav pro nastavení
const isSettingsOpen = ref(false)
const toggleSettings = () => {
  isSettingsOpen.value = !isSettingsOpen.value
}

// Zavření dropdownu při kliknutí jinam
const closeSettings = (e: MouseEvent) => {
  const target = e.target as HTMLElement
  if (!target.closest('.settings-dropdown')) {
    isSettingsOpen.value = false
  }
}

onMounted(() => {
  window.addEventListener('click', closeSettings)
})

onUnmounted(() => {
  window.removeEventListener('click', closeSettings)
})
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
        
        <!-- Nastavení / Settings -->
        <div class="relative settings-dropdown">
          <button @click.stop="toggleSettings" class="text-neutral-300 hover:text-white transition-colors p-1 rounded-full focus:outline-none focus:ring-2 focus:ring-violet-500/50">
            <!-- Opravená ikonka ozubeného kola (Nastavení) -->
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-7 h-7">
              <path stroke-linecap="round" stroke-linejoin="round" d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z" />
              <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
          </button>
          
          <!-- Dropdown menu -->
          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 scale-95"
            enter-to-class="transform opacity-100 scale-100"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 scale-100"
            leave-to-class="transform opacity-0 scale-95"
          >
            <div v-if="isSettingsOpen" class="absolute right-0 mt-2 w-56 rounded-xl shadow-xl bg-neutral-800 border border-neutral-700/60 ring-1 ring-black ring-opacity-5 z-50 p-2 space-y-1">
              
              <!-- Tlačítko vyhledání s fialovým tématem -->
              <a href="#" @click.prevent="isSettingsOpen = false" class="block w-full px-4 py-2.5 text-sm font-bold text-white bg-violet-600 hover:bg-violet-500 rounded-lg transition-all duration-200 shadow-[0_0_15px_rgba(139,92,246,0.15)] hover:shadow-[0_0_20px_rgba(139,92,246,0.3)] hover:scale-[1.02]">
                <div class="flex items-center gap-2.5">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                  </svg>
                  Vyhledat nové filmy
                </div>
              </a>
              
              <!-- Pokročilé nastavení -->
              <a href="#" @click.prevent="navigate('settings'); isSettingsOpen = false" class="block w-full px-4 py-2 text-sm font-medium text-neutral-300 hover:bg-neutral-700 hover:text-white rounded-lg transition-colors">
                <div class="flex items-center gap-2.5">
                  <!-- Klasické ozubené kolečko -->
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M10.343 3.94c.09-.542.56-.94 1.11-.94h1.093c.55 0 1.02.398 1.11.94l.149.894c.07.424.384.764.78.93.398.164.855.142 1.205-.108l.737-.527a1.125 1.125 0 011.45.12l.773.774c.39.389.44 1.002.12 1.45l-.527.737c-.25.35-.272.806-.107 1.204.165.397.505.71.93.78l.893.15c.543.09.94.56.94 1.109v1.094c0 .55-.397 1.02-.94 1.11l-.893.149c-.425.07-.765.383-.93.78-.165.398-.143.854.107 1.204l.527.738c.32.447.269 1.06-.12 1.45l-.774.773a1.125 1.125 0 01-1.449.12l-.738-.527c-.35-.25-.806-.272-1.203-.107-.397.165-.71.505-.781.929l-.149.894c-.09.542-.56.94-1.11.94h-1.094c-.55 0-1.019-.398-1.11-.94l-.148-.894c-.071-.424-.384-.764-.781-.93-.398-.164-.854-.142-1.204.108l-.738.527c-.447.32-1.06.269-1.45-.12l-.773-.774a1.125 1.125 0 01-.12-1.45l.527-.737c.25-.35.273-.806.108-1.204-.165-.397-.505-.71-.93-.78l-.894-.15c-.542-.09-.94-.56-.94-1.109v-1.094c0-.55.398-1.02.94-1.11l.894-.149c.424-.07.765-.383.93-.78.165-.398.143-.854-.107-1.204l-.527-.738a1.125 1.125 0 01.12-1.45l.773-.773a1.125 1.125 0 011.45-.12l.737.527c.35.25.807.272 1.204.107.397-.165.71-.505.78-.929l.15-.894z" />
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  </svg>
                  Pokročilé nastavení
                </div>
              </a>
              
            </div>
          </Transition>
        </div>
      </div>
    </nav>

    <!-- Zobrazení aktuální stránky -->
    <component :is="currentView" :item="currentParams" />

  </div>
</template>