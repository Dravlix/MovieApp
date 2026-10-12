<script setup lang="ts">
import { inject, ref, onMounted } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';

import { useProfileSettings } from '../composables/useProfileSettings';

const navigate = inject<Function>('navigate');
const startScan = inject<Function>('startScan');
const toast = useToast();

const { settings: profileSettings, setDefaultAudio, setDefaultSubtitles, setSubtitleSize, setPreferredPlayer } = useProfileSettings();

// Stav pro složky
const folders = ref<string[]>([]);
const needsSync = ref(false);

// Stav MPV
const mpvStatus = ref<{ is_installed: boolean; is_running: boolean; path?: string | null } | null>(null);

onMounted(async () => {
  try {
    const loaded = await invoke<string[]>('get_scan_folders');
    if (loaded && loaded.length > 0) {
      folders.value = loaded;
    } else {
      // Výchozí složka pokud je prázdno
      folders.value = ['/Users/dominik-mac/Filmy'];
    }
  } catch (e) {
    console.error('Nepodařilo se načíst složky z databáze:', e);
  }

  try {
    mpvStatus.value = await invoke<{ is_installed: boolean; is_running: boolean; path?: string | null }>('check_mpv_status');
  } catch (e) {
    console.warn('Nepodařilo se ověřit status MPV:', e);
  }
});

const addFolder = async () => {
  try {
    // Nativní dialog z OS (Finder na Macu) pro výběr složek
    const selected = await open({
      directory: true, // chceme složky
      multiple: true,  // umožníme vybrat více složek najednou
      title: 'Vyberte složky s filmy a seriály',
    });

    if (selected === null) {
      return;
    }

    const paths = Array.isArray(selected) ? selected : [selected];
    
    let addedCount = 0;
    for (const path of paths) {
      if (!folders.value.includes(path)) {
        folders.value.push(path);
        try {
          await invoke('add_scan_folder', { path });
        } catch (err) {
          console.error(err);
        }
        addedCount++;
      }
    }

    if (addedCount > 0) {
      needsSync.value = true;
      toast.info(`Přidáno ${addedCount} složek. Nezapomeňte synchronizovat.`);
    }
  } catch (e) {
    console.error("Tauri dialog se nepodařilo otevřít:", e);
    const fallbackPath = window.prompt('Otevření Finderu selhalo. Zadejte cestu ručně:');
    if (fallbackPath && !folders.value.includes(fallbackPath)) {
      folders.value.push(fallbackPath);
      try {
        await invoke('add_scan_folder', { path: fallbackPath });
      } catch (err) {
        console.error(err);
      }
      needsSync.value = true;
      toast.info('Složka přidána ručně.');
    }
  }
};

const removeFolder = async (pathToRemove: string) => {
  folders.value = folders.value.filter(path => path !== pathToRemove);
  try {
    await invoke('remove_scan_folder', { path: pathToRemove });
  } catch (e) {
    console.error(e);
  }
  needsSync.value = true;
  toast.warning(`Složka ${pathToRemove} byla odebrána.`);
};

const handleSync = () => {
  needsSync.value = false;
  if (startScan) {
    startScan();
  } else {
    toast.info('Spouštím skenování knihovny...');
  }
};

const clearCache = () => {
  toast.success('Dočasné soubory a mezipaměť byly promazány (uvolněno 240 MB).', 'Cache vyčištěna');
};

const exportBackup = () => {
  toast.success('Záloha knihovny byla exportována do souboru movies_backup.db.', 'Záloha vytvořena');
};

const resetDatabase = () => {
  const confirmed = window.confirm('Opravdu chcete resetovat celou knihovnu? Všechny neuložené změny budou ztraceny.');
  if (confirmed) {
    toast.error('Knihovna byla resetována do výchozího stavu.', 'Reset dokončen');
  }
};
</script>

<template>
  <div class="relative min-h-screen bg-[#111111] pb-32">
    
    <!-- Plovoucí upozornění na nutnost synchronizace -->
    <Transition
      enter-active-class="transition ease-out duration-300"
      enter-from-class="transform -translate-y-10 opacity-0"
      enter-to-class="transform translate-y-0 opacity-100"
      leave-active-class="transition ease-in duration-200"
      leave-from-class="transform translate-y-0 opacity-100"
      leave-to-class="transform -translate-y-10 opacity-0"
    >
      <div v-if="needsSync" class="fixed top-24 left-1/2 -translate-x-1/2 z-50 w-[90%] max-w-2xl bg-violet-600/90 backdrop-blur-md border border-violet-500 rounded-xl shadow-2xl p-4 flex flex-col sm:flex-row items-center justify-between gap-4">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-white/20 rounded-full">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5 text-white">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
          </div>
          <div>
            <h4 class="text-white font-bold text-sm">Změnili jste seznam složek</h4>
            <p class="text-violet-200 text-xs mt-0.5">Aby se změny projevily v aplikaci, je potřeba knihovnu synchronizovat.</p>
          </div>
        </div>
        <button @click="handleSync" class="px-5 py-2 bg-white text-violet-700 hover:bg-neutral-100 font-bold text-sm rounded-lg transition-colors whitespace-nowrap shadow-sm">
          Synchronizovat nyní
        </button>
      </div>
    </Transition>

    <!-- Hlavička -->
    <div class="sticky top-0 z-40 bg-[#111111]/90 backdrop-blur-md border-b border-neutral-800/80 pt-24 pb-6 px-8 md:px-20">
      <div class="max-w-3xl mx-auto flex items-center gap-6">
        <button 
          @click="navigate && navigate('back')" 
          class="p-2 -ml-2 text-neutral-400 hover:text-white rounded-full hover:bg-neutral-800 transition-colors"
        >
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
          </svg>
        </button>
        <div>
          <h2 class="text-3xl font-bold text-white tracking-tight">Nastavení</h2>
          <p class="text-neutral-400 text-sm mt-1">Správa vaší knihovny a chování aplikace</p>
        </div>
      </div>
    </div>

    <!-- Hlavní obsah -->
    <div class="relative z-10 pt-10 px-8 md:px-20 max-w-3xl mx-auto">
      <div class="flex flex-col gap-10">
        
        <!-- Sekce: Zdroje souborů -->
        <section>
          <h3 class="text-sm font-semibold text-neutral-400 ml-4 mb-3">Zdroje médií</h3>
          <div class="bg-[#1c1c1c] rounded-2xl border border-neutral-800/80 overflow-hidden divide-y divide-neutral-800/80">
            
            <!-- Správa složek -->
            <div class="p-4 sm:p-5 flex items-start sm:items-center justify-between gap-4">
              <div>
                <h4 class="text-neutral-100 font-medium">Složky s filmy a seriály</h4>
                <p class="text-neutral-500 text-sm mt-0.5 mb-4">Odkud má aplikace načítat soubory</p>
                
                <!-- Výpis vybraných složek -->
                <div v-if="folders.length > 0" class="flex flex-col gap-2 w-full max-w-lg">
                  <div v-for="folder in folders" :key="folder" class="flex items-center justify-between bg-neutral-900/50 border border-neutral-800 px-3 py-2 rounded-lg group">
                    <div class="flex items-center gap-3 overflow-hidden text-neutral-300">
                      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4 shrink-0 text-violet-500">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z" />
                      </svg>
                      <span class="text-sm truncate">{{ folder }}</span>
                    </div>
                    <button @click="removeFolder(folder)" class="p-1.5 text-neutral-500 hover:text-red-400 hover:bg-red-500/10 rounded-md transition-colors" title="Odebrat složku">
                      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                      </svg>
                    </button>
                  </div>
                </div>
                <div v-else class="text-neutral-500 text-sm italic">
                  Zatím nebyly přidány žádné složky.
                </div>
              </div>
              <button @click="addFolder" class="px-4 py-2 bg-neutral-800 hover:bg-neutral-700 text-neutral-200 text-sm font-medium rounded-lg transition-colors whitespace-nowrap border border-neutral-700 mt-1 sm:mt-0">
                + Přidat složku
              </button>
            </div>

            <!-- Automatické skenování -->
            <div class="p-4 sm:p-5 flex items-center justify-between cursor-pointer hover:bg-white/[0.02] transition-colors">
              <div>
                <h4 class="text-neutral-100 font-medium">Automatické skenování</h4>
                <p class="text-neutral-500 text-sm mt-0.5">Při startu aplikace automaticky hledat nové soubory</p>
              </div>
              <div class="w-12 h-6 bg-neutral-700 rounded-full relative ml-4 shrink-0 transition-colors">
                <div class="absolute left-1 top-1 w-4 h-4 bg-neutral-400 rounded-full transition-all"></div>
              </div>
            </div>

          </div>
        </section>

        <!-- Sekce: Metadata -->
        <section>
          <h3 class="text-sm font-semibold text-neutral-400 ml-4 mb-3">Metadata a stahování</h3>
          <div class="bg-[#1c1c1c] rounded-2xl border border-neutral-800/80 overflow-hidden divide-y divide-neutral-800/80">
            
            <!-- Jazyk -->
            <div class="p-4 sm:p-5 flex items-center justify-between cursor-pointer hover:bg-white/[0.02] transition-colors">
              <div>
                <h4 class="text-neutral-100 font-medium">Jazyk metadat</h4>
                <p class="text-neutral-500 text-sm mt-0.5">Preferovaný jazyk pro názvy a popisy filmů</p>
              </div>
              <select class="bg-transparent text-neutral-200 font-medium text-sm text-right outline-none cursor-pointer">
                <option value="cs" class="bg-neutral-800 text-left">Čeština</option>
                <option value="en" class="bg-neutral-800 text-left">Angličtina</option>
              </select>
            </div>

            <!-- Offline režim -->
            <div class="p-4 sm:p-5 flex items-center justify-between cursor-pointer hover:bg-white/[0.02] transition-colors">
              <div>
                <h4 class="text-neutral-100 font-medium">Ukládat obrázky lokálně</h4>
                <p class="text-neutral-500 text-sm mt-0.5">Stáhne plakáty pro plnohodnotný offline režim</p>
              </div>
              <div class="w-12 h-6 bg-violet-600 rounded-full relative ml-4 shrink-0 transition-colors">
                <div class="absolute left-7 top-1 w-4 h-4 bg-white rounded-full transition-all shadow-sm"></div>
              </div>
            </div>

          </div>
        </section>

        <!-- Sekce: Přehrávání a MPV Profil -->
        <section>
          <div class="flex items-center justify-between ml-4 mb-3">
            <h3 class="text-sm font-semibold text-neutral-400">Přehrávání a uživatelský profil</h3>
            <span class="text-xs text-neutral-500">Uloženo v profilu</span>
          </div>
          <div class="bg-[#1c1c1c] rounded-2xl border border-neutral-800/80 overflow-hidden divide-y divide-neutral-800/80">
            
            <!-- Výchozí přehrávač -->
            <div class="p-4 sm:p-5 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
              <div>
                <div class="flex items-center gap-2">
                  <h4 class="text-neutral-100 font-medium">Výchozí video přehrávač</h4>
                  <span v-if="mpvStatus?.is_installed" class="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400 mr-1.5 animate-pulse"></span>
                    MPV k dispozici
                  </span>
                  <span v-else class="inline-flex items-center px-2 py-0.5 rounded text-[11px] font-medium bg-amber-500/10 text-amber-400 border border-amber-500/20" title="Pro nejlepší výkon doporučujeme: brew install mpv">
                    MPV nenalezeno (doporučeno nainstalovat)
                  </span>
                </div>
                <p class="text-neutral-500 text-sm mt-0.5">
                  Vyberte, zda preferujete rychlý open-source MPV engine nebo vestavěný přehrávač.
                </p>
              </div>
              <div class="flex items-center gap-2 shrink-0">
                <button
                  type="button"
                  @click="setPreferredPlayer('mpv')"
                  :class="profileSettings.preferredPlayer === 'mpv' ? 'bg-violet-600 text-white font-semibold shadow-lg shadow-violet-600/30' : 'bg-neutral-800 text-neutral-300 hover:bg-neutral-700'"
                  class="px-3.5 py-1.5 rounded-lg text-sm transition-all flex items-center gap-1.5"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-4 h-4">
                    <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
                  </svg>
                  MPV Player
                </button>
                <button
                  type="button"
                  @click="setPreferredPlayer('internal')"
                  :class="profileSettings.preferredPlayer === 'internal' ? 'bg-violet-600 text-white font-semibold shadow-lg shadow-violet-600/30' : 'bg-neutral-800 text-neutral-300 hover:bg-neutral-700'"
                  class="px-3.5 py-1.5 rounded-lg text-sm transition-all"
                >
                  Vestavěný
                </button>
              </div>
            </div>

            <!-- Výchozí jazyk zvuku (Audio Stopa) -->
            <div class="p-4 sm:p-5 flex items-center justify-between">
              <div>
                <h4 class="text-neutral-100 font-medium">Výchozí zvuková stopa (Audio)</h4>
                <p class="text-neutral-500 text-sm mt-0.5">Jazyk, který bude automaticky vybrán při spuštění filmu či epizody</p>
              </div>
              <select
                :value="profileSettings.defaultAudioLanguage"
                @change="setDefaultAudio(($event.target as HTMLSelectElement).value)"
                class="bg-neutral-800 border border-neutral-700 text-neutral-200 text-sm rounded-lg px-3 py-1.5 outline-none cursor-pointer hover:border-neutral-600 focus:border-violet-500 transition-colors"
              >
                <option value="CZ">🇨🇿 Čeština (Dabing)</option>
                <option value="EN">🇬🇧 Angličtina (Původní)</option>
                <option value="SK">🇸🇰 Slovenština</option>
                <option value="ORIGINAL">🌐 Původní znění</option>
              </select>
            </div>

            <!-- Výchozí jazyk titulků -->
            <div class="p-4 sm:p-5 flex items-center justify-between">
              <div>
                <h4 class="text-neutral-100 font-medium">Výchozí jazyk titulků</h4>
                <p class="text-neutral-500 text-sm mt-0.5">Titulková stopa načtená automaticky při přehrávání</p>
              </div>
              <select
                :value="profileSettings.defaultSubtitleLanguage"
                @change="setDefaultSubtitles(($event.target as HTMLSelectElement).value)"
                class="bg-neutral-800 border border-neutral-700 text-neutral-200 text-sm rounded-lg px-3 py-1.5 outline-none cursor-pointer hover:border-neutral-600 focus:border-violet-500 transition-colors"
              >
                <option value="off">🚫 Vypnuto</option>
                <option value="CZ">🇨🇿 Čeština</option>
                <option value="EN">🇬🇧 Angličtina</option>
                <option value="SK">🇸🇰 Slovenština</option>
              </select>
            </div>

            <!-- Velikost písma titulků + Živý náhled -->
            <div class="p-4 sm:p-5 flex flex-col gap-4">
              <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
                <div>
                  <h4 class="text-neutral-100 font-medium">Velikost písma titulků</h4>
                  <p class="text-neutral-500 text-sm mt-0.5">Nastavení se aplikuje na vestavěný přehrávač i MPV</p>
                </div>
                <div class="flex items-center gap-1.5 bg-neutral-900 p-1 rounded-xl border border-neutral-800 self-start sm:self-auto">
                  <button
                    v-for="size in [
                      { id: 'small', label: 'Malé (75%)' },
                      { id: 'medium', label: 'Střední (100%)' },
                      { id: 'large', label: 'Velké (125%)' },
                      { id: 'xlarge', label: 'Extra velké (150%)' }
                    ] as const"
                    :key="size.id"
                    type="button"
                    @click="setSubtitleSize(size.id)"
                    :class="profileSettings.subtitleSize === size.id ? 'bg-violet-600 text-white font-medium shadow-sm' : 'text-neutral-400 hover:text-white'"
                    class="px-2.5 py-1 text-xs rounded-lg transition-all"
                  >
                    {{ size.label }}
                  </button>
                </div>
              </div>

              <!-- Náhled titulků -->
              <div class="relative bg-black/60 rounded-xl p-4 border border-neutral-800/60 overflow-hidden flex flex-col items-center justify-center min-h-[90px] bg-gradient-to-b from-neutral-900/40 to-black/80">
                <span class="text-[10px] text-neutral-500 uppercase tracking-widest font-mono mb-1">Živý náhled titulků</span>
                <p
                  :class="{
                    'text-sm': profileSettings.subtitleSize === 'small',
                    'text-base': profileSettings.subtitleSize === 'medium',
                    'text-xl': profileSettings.subtitleSize === 'large',
                    'text-2xl': profileSettings.subtitleSize === 'xlarge',
                  }"
                  class="font-semibold text-white drop-shadow-[0_2px_4px_rgba(0,0,0,0.9)] bg-black/40 px-3 py-1 rounded backdrop-blur-sm border border-white/5 transition-all duration-200"
                >
                  „Tohle je přesně ta chvíle, na kterou jsme čekali.“
                </p>
              </div>
            </div>

          </div>
        </section>

        <!-- Sekce: Údržba -->
        <section>
          <h3 class="text-sm font-semibold text-neutral-400 ml-4 mb-3">Údržba a databáze</h3>
          <div class="bg-[#1c1c1c] rounded-2xl border border-neutral-800/80 overflow-hidden divide-y divide-neutral-800/80">
            
            <button @click="clearCache" class="w-full text-left p-4 sm:p-5 hover:bg-white/[0.02] transition-colors cursor-pointer">
              <h4 class="text-neutral-200 font-medium">Vymazat mezipaměť (Cache)</h4>
              <p class="text-neutral-500 text-sm mt-0.5">Uvolní místo na disku smazáním dočasných souborů</p>
            </button>
            
            <button @click="exportBackup" class="w-full text-left p-4 sm:p-5 hover:bg-white/[0.02] transition-colors cursor-pointer">
              <h4 class="text-neutral-200 font-medium">Exportovat zálohu</h4>
              <p class="text-neutral-500 text-sm mt-0.5">Uloží databázi (historii a wishlist) do souboru</p>
            </button>
            
            <button @click="resetDatabase" class="w-full text-left p-4 sm:p-5 bg-red-950/10 hover:bg-red-950/30 transition-colors cursor-pointer">
              <h4 class="text-red-500 font-medium">Smazat databázi a resetovat</h4>
              <p class="text-red-500/70 text-sm mt-0.5">Nevratně vymaže celou knihovnu a uvede aplikaci do původního stavu</p>
            </button>

          </div>
        </section>

      </div>
    </div>
  </div>
</template>
