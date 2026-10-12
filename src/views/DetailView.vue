<script setup lang="ts">
import { inject, ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { MediaItem, Episode, Season } from '../types';
import { useToast } from '../composables/useToast';
import MediaCard from '../components/MediaCard.vue';

const props = defineProps<{
  item?: MediaItem;
}>();

const navigate = inject<Function>('navigate');
const playMedia = inject<Function>('playMedia');
const toast = useToast();

const inWishlist = ref(false);
const similarItems = ref<MediaItem[]>([]);
const selectedSeasonIndex = ref(0);

// Generování ukázkových sérií a epizod pro seriály
const seriesSeasons = computed<Season[]>(() => {
  if (props.item?.type !== 'series') return [];
  if (props.item.seasons && props.item.seasons.length > 0) return props.item.seasons;

  // Výchozí vygenerované epizody
  return [
    {
      seasonNumber: 1,
      title: '1. Série',
      episodes: [
        {
          id: 'ep1',
          episodeNumber: 1,
          title: 'Pilotní díl: Začátek cesty',
          duration: '54 min',
          overview: 'V prvním díle se seznamujeme s hlavními hrdiny a tajemným incidentem, který odstartuje celý řetězec nečekaných událostí.',
          stillUrl: props.item.backdropUrl || props.item.posterUrl,
        },
        {
          id: 'ep2',
          episodeNumber: 2,
          title: 'Bod zlomu',
          duration: '48 min',
          overview: 'Napětí stoupá, když se na scéně objevuje nečekaný spojenec a odhaluje střípky z minulosti.',
          stillUrl: props.item.backdropUrl || props.item.posterUrl,
        },
        {
          id: 'ep3',
          episodeNumber: 3,
          title: 'Stíny minulosti',
          duration: '51 min',
          overview: 'Tým musí čelit první velké překážce a riskovat vše pro záchranu klíčového svědka.',
          stillUrl: props.item.backdropUrl || props.item.posterUrl,
        },
        {
          id: 'ep4',
          episodeNumber: 4,
          title: 'Finále série: Bez návratu',
          duration: '59 min',
          overview: 'Všechny dějové linky vrcholí v dramatickém střetu, kde nic není takové, jak se na první pohled zdálo.',
          stillUrl: props.item.backdropUrl || props.item.posterUrl,
        },
      ]
    },
    {
      seasonNumber: 2,
      title: '2. Série',
      episodes: [
        {
          id: 'ep2_1',
          episodeNumber: 1,
          title: 'Nové probuzení',
          duration: '56 min',
          overview: 'Rok po událostech první řady se hrdinové ocitají v novém prostředí a čelí ještě větší hrozbě.',
          stillUrl: props.item.backdropUrl || props.item.posterUrl,
        },
        {
          id: 'ep2_2',
          episodeNumber: 2,
          title: 'Temné spojenectví',
          duration: '50 min',
          overview: 'Nezvyklé spojenectví přináší nové výzvy a morální dilemata pro všechny zúčastněné.',
          stillUrl: props.item.backdropUrl || props.item.posterUrl,
        }
      ]
    }
  ];
});

const currentEpisodes = computed<Episode[]>(() => {
  return seriesSeasons.value[selectedSeasonIndex.value]?.episodes || [];
});

onMounted(async () => {
  if (props.item) {
    try {
      inWishlist.value = await invoke<boolean>('check_wishlist', { id: props.item.id });
      
      // Načtení podobných filmů
      const all = await invoke<MediaItem[]>(props.item.type === 'series' ? 'get_series' : 'get_movies');
      similarItems.value = all.filter(m => m.id !== props.item?.id).slice(0, 6);
    } catch (e) {
      console.error(e);
    }
  }
});

const toggleWishlist = async () => {
  if (props.item) {
    try {
      inWishlist.value = await invoke<boolean>('toggle_wishlist', { id: props.item.id });
      if (inWishlist.value) {
        toast.success(`"${props.item.title}" byl přidán do Wishlistu.`);
      } else {
        toast.info(`"${props.item.title}" byl odebrán z Wishlistu.`);
      }
    } catch (e) {
      console.error(e);
    }
  }
};

const handlePlay = () => {
  if (props.item && playMedia) {
    playMedia(props.item);
  }
};

const playEpisode = (ep: Episode) => {
  if (props.item && playMedia) {
    playMedia({
      ...props.item,
      title: `${props.item.title} - S${seriesSeasons.value[selectedSeasonIndex.value]?.seasonNumber}E${ep.episodeNumber}: ${ep.title}`,
      description: ep.overview,
    });
  }
};
</script>

<template>
  <div v-if="item" class="relative w-full min-h-screen bg-neutral-900 pb-32">
    <!-- Tlačítko Zpět -->
    <button 
      @click="navigate && navigate('back')" 
      class="fixed top-24 left-8 md:left-12 z-40 flex items-center gap-2 px-4 py-2 bg-neutral-800/60 hover:bg-neutral-700/80 backdrop-blur-md rounded-full text-white text-sm font-medium transition-all shadow-lg border border-neutral-600/50 hover:scale-105 cursor-pointer"
    >
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-4 h-4">
        <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
      </svg>
      Zpět
    </button>

    <!-- Background Backdrop Image -->
    <div class="relative w-full h-[70vh] md:h-[80vh] z-0 overflow-hidden">
      <img :src="item.backdropUrl || item.posterUrl" alt="Backdrop" class="w-full h-full object-cover opacity-60" />
      <div class="absolute inset-0 bg-gradient-to-r from-neutral-900 via-neutral-900/80 to-transparent md:w-[70%]"></div>
      <div class="absolute inset-0 bg-gradient-to-t from-neutral-900 via-neutral-900/40 to-transparent"></div>
    </div>

    <!-- Content Wrapper -->
    <div class="relative z-10 px-8 md:px-20 max-w-7xl mx-auto -mt-64 md:-mt-80 flex flex-col gap-14">
      
      <!-- Horní řada: Hlavní info & Boční panel -->
      <div class="flex flex-col md:flex-row gap-12 justify-between">
        <!-- Levý sloupec (Nadpis, Meta, Popis, Tlačítka) -->
        <div class="flex-grow max-w-3xl flex flex-col gap-6">
          <div class="flex items-center gap-3">
            <span class="text-xs px-2.5 py-1 rounded-md font-bold uppercase tracking-wider bg-violet-600/30 text-violet-300 border border-violet-500/40">
              {{ item.type === 'series' ? 'TV Seriál' : 'Film' }}
            </span>
            <!-- Rating Badge -->
            <div class="flex items-center gap-1.5 bg-amber-500/20 border border-amber-500/40 px-2.5 py-0.5 rounded-md text-amber-300 text-xs font-black">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-3.5 h-3.5 text-amber-400">
                <path fill-rule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.006 5.404.434c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.434 2.082-5.005z" clip-rule="evenodd" />
              </svg>
              <span>{{ item.rating || 8.4 }} / 10</span>
            </div>
          </div>

          <h2 class="text-4xl md:text-6xl font-black text-white tracking-tight uppercase drop-shadow-md">
            {{ item.title }}
          </h2>
          
          <!-- Metadata (Rok, Délka, 4K, Audio) -->
          <div class="flex flex-wrap items-center gap-3 text-sm font-semibold text-neutral-300">
            <span>{{ item.year }}</span>
            <span v-if="item.duration">•</span>
            <span v-if="item.duration">{{ item.duration }}</span>
            <span>•</span>
            <span class="border border-neutral-600 px-1.5 py-0.5 rounded text-[10px] text-neutral-200 font-bold bg-neutral-800">4K UHD</span>
            <span class="border border-neutral-600 px-1.5 py-0.5 rounded text-[10px] text-neutral-200 font-bold bg-neutral-800">5.1 SURROUND</span>
            <span v-if="item.ageRating" class="border border-neutral-600 px-1.5 py-0.5 rounded text-[10px] text-neutral-200 font-bold bg-neutral-800">{{ item.ageRating }}</span>
          </div>
          
          <p class="text-neutral-200 text-base md:text-lg leading-relaxed font-normal">
            {{ item.description || 'Popis k tomuto titulu se momentálně připravuje. Brzy zde najdete více informací o příběhu, hercích a tvůrcích.' }}
          </p>
          
          <!-- Tlačítka -->
          <div class="flex flex-wrap items-center gap-4 mt-2">
            <button
              @click="handlePlay"
              class="flex items-center gap-2.5 px-8 py-3.5 bg-white text-black font-bold rounded-lg hover:bg-neutral-200 hover:scale-105 transition-all duration-300 shadow-[0_0_20px_rgba(255,255,255,0.2)] cursor-pointer"
            >
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-6 h-6">
                <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
              </svg>
              {{ item.type === 'series' ? 'Přehrát 1. epizodu' : 'Přehrát film' }}
            </button>

            <button
              @click="handlePlay"
              class="flex items-center gap-2 px-6 py-3.5 bg-neutral-800/80 hover:bg-neutral-700/80 backdrop-blur-md text-white font-bold rounded-lg border border-neutral-700 transition-all cursor-pointer"
            >
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5 text-violet-400">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5.25 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L8.029 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" />
              </svg>
              Trailer
            </button>
            
            <button
              @click="toggleWishlist"
              class="flex items-center gap-2 px-6 py-3.5 bg-neutral-800/80 backdrop-blur-md text-white font-bold rounded-lg hover:bg-neutral-700/80 transition-all border border-neutral-700 cursor-pointer"
            >
              <svg v-if="!inWishlist" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
              <svg v-else xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5 text-violet-400">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
              </svg>
              {{ inWishlist ? 'Ve wishlistu' : 'Do wishlistu' }}
            </button>
          </div>
        </div>

        <!-- Pravý sloupec (Hrají, Žánry, Vlastnosti) -->
        <div class="flex flex-col gap-4 text-sm w-full md:w-80 shrink-0 bg-neutral-800/40 p-6 rounded-2xl border border-neutral-700/40 backdrop-blur-md h-fit">
          <div v-if="item.cast && item.cast.length" class="leading-relaxed">
            <span class="text-neutral-400 font-semibold block mb-1">Hrají:</span> 
            <span class="text-neutral-200">{{ item.cast.join(', ') }}</span>
          </div>
          <div v-if="item.genres && item.genres.length" class="leading-relaxed">
            <span class="text-neutral-400 font-semibold block mb-1">Žánry:</span> 
            <span class="text-neutral-200">{{ item.genres.join(', ') }}</span>
          </div>
          <div v-if="item.moods && item.moods.length" class="leading-relaxed">
            <span class="text-neutral-400 font-semibold block mb-1">Nálada pořadu:</span> 
            <span class="text-neutral-200">{{ item.moods.join(', ') }}</span>
          </div>

          <!-- Dostupné jazyky a zvuk -->
          <div class="border-t border-neutral-700/50 pt-3">
            <span class="text-neutral-400 font-semibold block mb-1.5 flex items-center gap-1.5">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 text-violet-400">
                <path d="M10 2a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 0110 2zM10 15a.75.75 0 01.75.75v1.5a.75.75 0 01-1.5 0v-1.5A.75.75 0 0110 15zM10 7a3 3 0 100 6 3 3 0 000-6z" />
              </svg>
              Dostupný zvuk:
            </span> 
            <div class="flex flex-wrap gap-1.5">
              <span
                v-for="lang in (item.audioLanguages?.length ? item.audioLanguages : ['CZ (Dabing)', 'EN (Původní)'])"
                :key="lang"
                class="px-2 py-0.5 rounded text-xs font-bold bg-violet-950/60 text-violet-200 border border-violet-500/40"
              >
                {{ lang }}
              </span>
            </div>
          </div>

          <!-- Titulky -->
          <div>
            <span class="text-neutral-400 font-semibold block mb-1.5 flex items-center gap-1.5">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 text-violet-400">
                <path fill-rule="evenodd" d="M2 4.25A2.25 2.25 0 014.25 2h11.5A2.25 2.25 0 0118 4.25v8.5A2.25 2.25 0 0115.75 15h-3.19l-2.833 2.833a.75.75 0 01-1.06 0L5.833 15H4.25A2.25 2.25 0 012 12.75v-8.5zM4.5 5.5a.75.75 0 000 1.5h11a.75.75 0 000-1.5h-11zm0 3.5a.75.75 0 000 1.5h7a.75.75 0 000-1.5h-7z" clip-rule="evenodd" />
              </svg>
              Titulky:
            </span> 
            <div class="flex flex-wrap gap-1.5">
              <span
                v-for="sub in (item.subtitleLanguages?.length ? item.subtitleLanguages : ['Čeština', 'Angličtina', 'Slovenština'])"
                :key="sub"
                class="px-2 py-0.5 rounded text-xs font-medium bg-neutral-900/80 text-neutral-300 border border-neutral-700"
              >
                {{ sub }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Pokud jde o Seriál: Seznam epizod a sérií -->
      <section v-if="item.type === 'series'" class="flex flex-col gap-6 pt-4 border-t border-neutral-800">
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <h3 class="text-2xl font-bold text-white flex items-center gap-3">
            <span class="w-1.5 h-6 bg-violet-500 rounded-full"></span>
            Epizody
          </h3>

          <!-- Výběr série -->
          <div class="flex items-center gap-2">
            <button
              v-for="(season, sIdx) in seriesSeasons"
              :key="season.seasonNumber"
              @click="selectedSeasonIndex = sIdx"
              class="px-4 py-2 rounded-xl text-sm font-bold transition-all cursor-pointer"
              :class="selectedSeasonIndex === sIdx ? 'bg-violet-600 text-white shadow-lg shadow-violet-600/30' : 'bg-neutral-800 hover:bg-neutral-700 text-neutral-300'"
            >
              {{ season.title }}
            </button>
          </div>
        </div>

        <!-- Seznam epizod -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div
            v-for="ep in currentEpisodes"
            :key="ep.id"
            @click="playEpisode(ep)"
            class="group p-4 bg-[#161616] border border-neutral-800 hover:border-neutral-700 rounded-xl flex gap-4 cursor-pointer transition-all hover:scale-[1.01]"
          >
            <!-- Náhled epizody -->
            <div class="relative w-36 aspect-video bg-neutral-800 rounded-lg overflow-hidden shrink-0">
              <img :src="ep.stillUrl" :alt="ep.title" class="w-full h-full object-cover group-hover:scale-105 transition-transform" />
              <div class="absolute inset-0 bg-black/40 group-hover:bg-black/10 flex items-center justify-center transition-colors">
                <div class="w-8 h-8 rounded-full bg-violet-600 text-white flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity shadow-md">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4 ml-0.5">
                    <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
                  </svg>
                </div>
              </div>
            </div>

            <!-- Popis epizody -->
            <div class="flex-grow min-w-0">
              <div class="flex items-center justify-between text-xs text-neutral-400 mb-1">
                <span class="font-bold text-violet-400">Epizoda {{ ep.episodeNumber }}</span>
                <span>{{ ep.duration }}</span>
              </div>
              <h4 class="font-bold text-white text-sm group-hover:text-violet-300 transition-colors truncate">
                {{ ep.title }}
              </h4>
              <p class="text-xs text-neutral-400 mt-1 line-clamp-2 leading-relaxed">
                {{ ep.overview }}
              </p>
            </div>
          </div>
        </div>
      </section>

      <!-- Podobné tituly -->
      <section v-if="similarItems.length > 0" class="flex flex-col gap-6 pt-4 border-t border-neutral-800">
        <h3 class="text-2xl font-bold text-white flex items-center gap-3">
          <span class="w-1.5 h-6 bg-violet-500 rounded-full"></span>
          Mohlo by se vám líbit
        </h3>
        
        <div class="flex flex-wrap gap-x-6 gap-y-12 py-4">
          <MediaCard
            v-for="sim in similarItems"
            :key="sim.id"
            :item="sim"
          />
        </div>
      </section>

    </div>
  </div>
</template>
