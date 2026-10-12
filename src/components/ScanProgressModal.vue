<script setup lang="ts">
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from '../composables/useToast';

interface ScanSummary {
  scannedFolders: number;
  filesFound: number;
  newAdded: number;
  newTitles: string[];
}

const props = defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits(['close', 'finished']);

const toast = useToast();

const progress = ref(0);
const currentStepText = ref('Příprava skenování...');
const isFinished = ref(false);
const discoveredMovies = ref<string[]>([]);
const stats = ref<{ scannedFolders: number; filesFound: number; newAdded: number }>({
  scannedFolders: 0,
  filesFound: 0,
  newAdded: 0,
});

const runRealScan = async () => {
  progress.value = 15;
  isFinished.value = false;
  discoveredMovies.value = [];
  currentStepText.value = 'Prohledávám nastavené lokální složky na disku...';

  try {
    const progressTimer = setInterval(() => {
      if (progress.value < 85) {
        progress.value += 10;
        if (progress.value === 45) {
          currentStepText.value = 'Detekuji video formáty (MKV, MP4, AVI)...';
        } else if (progress.value === 75) {
          currentStepText.value = 'Zjišťuji jazykové stopy a metadata...';
        }
      }
    }, 300);

    // Skutečné volání Rust backendu
    const result = await invoke<ScanSummary>('scan_library');
    clearInterval(progressTimer);

    progress.value = 100;
    isFinished.value = true;
    stats.value = {
      scannedFolders: result.scannedFolders,
      filesFound: result.filesFound,
      newAdded: result.newAdded,
    };
    discoveredMovies.value = result.newTitles;

    if (result.newAdded > 0) {
      currentStepText.value = `Dokončeno! Přidáno ${result.newAdded} nových titulů.`;
      toast.success(`Nalezeno a přidáno ${result.newAdded} nových médií.`, 'Knihovna aktualizována');
    } else {
      currentStepText.value = `Proskenováno ${result.filesFound} souborů. Vše je již aktuální.`;
      toast.info(`Nebyly nalezeny žádné nové soubory k přidání.`, 'Knihovna je aktuální');
    }

    emit('finished');
  } catch (error) {
    console.error('Chyba při skenování knihovny:', error);
    currentStepText.value = 'Došlo k chybě při čtení složek.';
    progress.value = 100;
    isFinished.value = true;
    toast.error('Nepodařilo se dokončit skenování složek.');
  }
};

watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    runRealScan();
  }
});

const close = () => {
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
      class="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex items-center justify-center p-4 select-none"
    >
      <div
        class="bg-[#181818] border border-neutral-700/80 rounded-2xl w-full max-w-lg shadow-[0_25px_60px_rgba(0,0,0,0.8)] overflow-hidden flex flex-col p-6 animate-in fade-in zoom-in-95 duration-150"
      >
        <!-- Hlavička -->
        <div class="flex items-center gap-3 mb-6">
          <div class="p-3 bg-violet-600/20 border border-violet-500/30 rounded-xl text-violet-400">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6 animate-spin" :class="{'animate-none': isFinished}">
              <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
            </svg>
          </div>
          <div>
            <h3 class="text-xl font-bold text-white tracking-tight">
              {{ isFinished ? 'Knihovna je aktuální' : 'Vyhledávání nových filmů' }}
            </h3>
            <p class="text-xs text-neutral-400 mt-0.5">
              Prohledávání lokálních složek a synchronizace metadat
            </p>
          </div>
        </div>

        <!-- Progress bar -->
        <div class="space-y-2 mb-6">
          <div class="flex items-center justify-between text-xs font-semibold">
            <span class="text-neutral-300">{{ currentStepText }}</span>
            <span class="text-violet-400 font-mono">{{ progress }}%</span>
          </div>
          <div class="w-full h-2.5 bg-neutral-800 rounded-full overflow-hidden border border-neutral-700/50">
            <div
              class="h-full bg-gradient-to-r from-violet-600 to-fuchsia-500 rounded-full transition-all duration-300 ease-out"
              :style="{ width: `${progress}%` }"
            ></div>
          </div>
        </div>

        <!-- Nalezené soubory -->
        <div v-if="discoveredMovies.length > 0" class="mb-6">
          <h4 class="text-xs font-bold text-neutral-400 uppercase tracking-wider mb-2">
            Nalezené novinky:
          </h4>
          <div class="space-y-1.5 max-h-36 overflow-y-auto pr-1">
            <div
              v-for="file in discoveredMovies"
              :key="file"
              class="flex items-center gap-2 p-2 bg-neutral-900/60 border border-neutral-800 rounded-lg text-xs text-neutral-300"
            >
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4 text-emerald-400 shrink-0">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
              </svg>
              <span class="font-mono truncate">{{ file }}</span>
            </div>
          </div>
        </div>

        <!-- Tlačítka -->
        <div class="flex items-center justify-end gap-3 pt-2">
          <button
            v-if="!isFinished"
            @click="close"
            class="px-4 py-2 text-sm text-neutral-400 hover:text-white transition-colors"
          >
            Běžet na pozadí
          </button>
          <button
            v-else
            @click="close"
            class="w-full px-5 py-2.5 bg-violet-600 hover:bg-violet-500 font-bold text-white text-sm rounded-xl transition-all shadow-lg shadow-violet-600/30"
          >
            Zavřít a zobrazit knihovnu
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>
