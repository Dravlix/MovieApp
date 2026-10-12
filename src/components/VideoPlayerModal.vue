<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { MediaItem, Chapter } from '../types';
import { useProfileSettings, type ProfileSettings } from '../composables/useProfileSettings';
import { useToast } from '../composables/useToast';

const props = defineProps<{
  isOpen: boolean;
  item: MediaItem | null;
}>();

const emit = defineEmits(['close']);

const toast = useToast();
const { settings: profileSettings, setSubtitleSize, getSubtitleFontSizePixels } = useProfileSettings();

// Video stav
const videoRef = ref<HTMLVideoElement | null>(null);
const isPlaying = ref(false);
const currentTime = ref(0);
const duration = ref(0);
const volume = ref(1);
const isMuted = ref(false);
const showControls = ref(true);
let hideControlsTimeout: any = null;

// Přehrávač a stopy
const activeAudioTrack = ref<string>('CZ');
const activeSubtitleTrack = ref<string>('off');
const showAudioMenu = ref(false);
const showSubtitleMenu = ref(false);
const showChaptersMenu = ref(false);

// MPV stav
const mpvAvailable = ref(false);
const isMpvPlaying = ref(false);
const showMpvInstallModal = ref(false);

// Ukázkové video pokud položka nemá vlastní videoUrl
const defaultVideoUrl = 'https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4';

// Dostupné audio stopy
const availableAudioTracks = computed(() => {
  if (props.item?.audioLanguages && props.item.audioLanguages.length > 0) {
    return props.item.audioLanguages;
  }
  return ['CZ', 'EN'];
});

// Dostupné titulkové stopy
const availableSubtitleTracks = computed(() => {
  const base = ['off'];
  const fromItem = props.item?.subtitleLanguages && props.item.subtitleLanguages.length > 0
    ? props.item.subtitleLanguages
    : ['CZ', 'EN', 'SK'];
  return [...base, ...fromItem];
});

// Kapitoly položky (vlastní nebo automaticky generované)
const chapters = computed<Chapter[]>(() => {
  if (props.item?.chapters && props.item.chapters.length > 0) {
    return props.item.chapters;
  }
  const dur = duration.value > 0 ? duration.value : 600;
  return [
    { id: 'ch1', title: '01. Úvod a expozice', startTime: 0 },
    { id: 'ch2', title: '02. Zápletka a incident', startTime: Math.floor(dur * 0.18) },
    { id: 'ch3', title: '03. Vyšetřování & konfrontace', startTime: Math.floor(dur * 0.42) },
    { id: 'ch4', title: '04. Zlomový moment', startTime: Math.floor(dur * 0.68) },
    { id: 'ch5', title: '05. Vyvrcholení a finále', startTime: Math.floor(dur * 0.88) },
  ];
});

// Aktuální kapitola podle currentTime
const currentChapter = computed(() => {
  if (!chapters.value.length) return null;
  for (let i = chapters.value.length - 1; i >= 0; i--) {
    if (currentTime.value >= chapters.value[i].startTime) {
      return chapters.value[i];
    }
  }
  return chapters.value[0];
});

// Ukázkové titulky pro interní přehrávač
const currentSubtitleText = computed(() => {
  if (activeSubtitleTrack.value === 'off') return '';
  const sec = Math.floor(currentTime.value) % 24;
  if (sec >= 2 && sec <= 6) {
    return activeSubtitleTrack.value === 'EN'
      ? 'Everything changes from this moment on.'
      : 'Od tohoto okamžiku se všechno změní.';
  }
  if (sec >= 10 && sec <= 15) {
    return activeSubtitleTrack.value === 'EN'
      ? 'Are you ready for what comes next?'
      : 'Jste připraveni na to, co přijde dál?';
  }
  if (sec >= 18 && sec <= 22) {
    return activeSubtitleTrack.value === 'EN'
      ? 'There is no turning back now.'
      : 'Odtud už není cesty zpět.';
  }
  return '';
});

const formatTime = (seconds: number) => {
  if (isNaN(seconds)) return '00:00';
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  if (hrs > 0) {
    return `${hrs}:${mins < 10 ? '0' : ''}${mins}:${secs < 10 ? '0' : ''}${secs}`;
  }
  return `${mins < 10 ? '0' : ''}${mins}:${secs < 10 ? '0' : ''}${secs}`;
};

const togglePlay = () => {
  if (isMpvPlaying.value) {
    sendMpvCommand(['cycle', 'pause']);
    return;
  }
  if (!videoRef.value) return;
  if (videoRef.value.paused) {
    videoRef.value.play();
    isPlaying.value = true;
  } else {
    videoRef.value.pause();
    isPlaying.value = false;
  }
  resetControlsTimeout();
};

const seek = (e: Event) => {
  if (!videoRef.value) return;
  const target = e.target as HTMLInputElement;
  const time = parseFloat(target.value);
  videoRef.value.currentTime = time;
  currentTime.value = time;
  if (isMpvPlaying.value) {
    sendMpvCommand(['seek', time, 'absolute']);
  }
};

const skip = (seconds: number) => {
  if (isMpvPlaying.value) {
    sendMpvCommand(['seek', seconds, 'relative']);
    return;
  }
  if (!videoRef.value) return;
  videoRef.value.currentTime = Math.max(0, Math.min(duration.value, videoRef.value.currentTime + seconds));
  resetControlsTimeout();
};

const seekToChapter = (chapter: Chapter) => {
  if (videoRef.value) {
    videoRef.value.currentTime = chapter.startTime;
    currentTime.value = chapter.startTime;
  }
  if (isMpvPlaying.value) {
    sendMpvCommand(['seek', chapter.startTime, 'absolute']);
  }
  toast.info(`Přesunuto na: ${chapter.title}`);
  showChaptersMenu.value = false;
  resetControlsTimeout();
};

const nextChapter = () => {
  if (!currentChapter.value) return;
  const idx = chapters.value.findIndex(c => c.id === currentChapter.value?.id);
  if (idx !== -1 && idx < chapters.value.length - 1) {
    seekToChapter(chapters.value[idx + 1]);
  }
};

const prevChapter = () => {
  if (!currentChapter.value) return;
  const idx = chapters.value.findIndex(c => c.id === currentChapter.value?.id);
  if (idx > 0) {
    seekToChapter(chapters.value[idx - 1]);
  } else if (idx === 0) {
    seekToChapter(chapters.value[0]);
  }
};

const selectAudioTrack = (lang: string) => {
  activeAudioTrack.value = lang;
  showAudioMenu.value = false;
  toast.success(`Zvuková stopa přepnuta: ${lang}`);
  if (isMpvPlaying.value) {
    sendMpvCommand(['set', 'alang', lang.toLowerCase()]);
  }
};

const selectSubtitleTrack = (sub: string) => {
  activeSubtitleTrack.value = sub;
  showSubtitleMenu.value = false;
  if (sub === 'off') {
    toast.info('Titulky vypnuty');
    if (isMpvPlaying.value) sendMpvCommand(['set', 'sid', 'no']);
  } else {
    toast.success(`Titulky zapnuty: ${sub}`);
    if (isMpvPlaying.value) sendMpvCommand(['set', 'slang', sub.toLowerCase()]);
  }
};

const setPlayerSubtitleSize = (size: ProfileSettings['subtitleSize']) => {
  setSubtitleSize(size);
  if (isMpvPlaying.value) {
    const px = getSubtitleFontSizePixels(size);
    sendMpvCommand(['set', 'sub-font-size', px]);
  }
};

const toggleMute = () => {
  if (!videoRef.value) return;
  isMuted.value = !isMuted.value;
  videoRef.value.muted = isMuted.value;
};

const handleVolumeChange = (e: Event) => {
  if (!videoRef.value) return;
  const target = e.target as HTMLInputElement;
  const vol = parseFloat(target.value);
  volume.value = vol;
  videoRef.value.volume = vol;
  isMuted.value = vol === 0;
};

const toggleFullScreen = () => {
  const container = document.getElementById('player-container');
  if (!container) return;
  if (!document.fullscreenElement) {
    container.requestFullscreen().catch(() => {});
  } else {
    document.exitFullscreen().catch(() => {});
  }
};

// MPV Integrace
const checkMpvStatus = async () => {
  try {
    const res = await invoke<{ is_installed: boolean; is_running: boolean }>('check_mpv_status');
    mpvAvailable.value = res.is_installed;
    isMpvPlaying.value = res.is_running;
  } catch (e) {
    mpvAvailable.value = false;
  }
};

const launchMpv = async () => {
  if (!mpvAvailable.value) {
    showMpvInstallModal.value = true;
    return;
  }

  // Pozastavíme vestavěné HTML5 video
  if (videoRef.value) {
    videoRef.value.pause();
    isPlaying.value = false;
  }

  const fileToPlay = props.item?.localFilePath || props.item?.videoUrl || defaultVideoUrl;
  const subSizePx = getSubtitleFontSizePixels();

  try {
    await invoke('mpv_play', {
      options: {
        filePath: fileToPlay,
        subtitleSize: subSizePx,
        audioTrack: activeAudioTrack.value,
        subtitleTrack: activeSubtitleTrack.value,
        startTime: currentTime.value,
      }
    });
    isMpvPlaying.value = true;
    toast.success('Přehrávání v MPV zahájeno s vaším profilem', 'MPV Spuštěno');
  } catch (err: any) {
    console.error('Chyba při spuštění MPV:', err);
    toast.error(String(err), 'Chyba MPV');
  }
};

const stopMpvAndResumeInternal = async () => {
  try {
    await invoke('mpv_stop');
  } catch (e) {
    console.warn(e);
  }
  isMpvPlaying.value = false;
  if (videoRef.value) {
    videoRef.value.play().then(() => {
      isPlaying.value = true;
    }).catch(() => {});
  }
  toast.info('Návrat do vestavěného přehrávače');
};

const sendMpvCommand = async (args: any[]) => {
  try {
    const cmdStr = JSON.stringify({ command: args });
    await invoke('mpv_command', { jsonCmd: cmdStr });
  } catch (err) {
    console.warn('MPV IPC command error:', err);
  }
};

const openInExternalSystemPlayer = async () => {
  showMpvInstallModal.value = false;
  const path = props.item?.localFilePath || props.item?.videoUrl;
  if (path) {
    try {
      await invoke('open_media_file', { path });
      toast.success('Otevřeno ve výchozím přehrávači systému (VLC / QuickTime)');
      closePlayer();
    } catch (e) {
      toast.error('Nepodařilo se otevřít soubor externě.');
    }
  } else {
    toast.warning('Tento titul nemá lokální soubor pro externí přehrávač.');
  }
};

const resetControlsTimeout = () => {
  showControls.value = true;
  clearTimeout(hideControlsTimeout);
  if (isPlaying.value && !showAudioMenu.value && !showSubtitleMenu.value && !showChaptersMenu.value) {
    hideControlsTimeout = setTimeout(() => {
      showControls.value = false;
    }, 3500);
  }
};

const handleMouseMove = () => {
  resetControlsTimeout();
};

const handleKeyDown = (e: KeyboardEvent) => {
  if (!props.isOpen) return;

  if (e.key === 'Escape') {
    e.preventDefault();
    e.stopPropagation();
    if (showAudioMenu.value || showSubtitleMenu.value || showChaptersMenu.value || showMpvInstallModal.value) {
      showAudioMenu.value = false;
      showSubtitleMenu.value = false;
      showChaptersMenu.value = false;
      showMpvInstallModal.value = false;
      return;
    }
    closePlayer();
  } else if (e.key === ' ') {
    e.preventDefault();
    togglePlay();
  } else if (e.key === 'ArrowRight') {
    e.preventDefault();
    skip(10);
  } else if (e.key === 'ArrowLeft') {
    e.preventDefault();
    skip(-10);
  } else if (e.key === 'n' || e.key === 'N') {
    e.preventDefault();
    nextChapter();
  } else if (e.key === 'p' || e.key === 'P') {
    e.preventDefault();
    prevChapter();
  }
};

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown, true);
  checkMpvStatus();
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown, true);
  clearTimeout(hideControlsTimeout);
});

// Inicializace stavu při otevření přehrávače
watch(() => props.isOpen, (newVal) => {
  if (newVal) {
    showControls.value = true;
    showAudioMenu.value = false;
    showSubtitleMenu.value = false;
    showChaptersMenu.value = false;
    isMpvPlaying.value = false;

    // Aplikace předvoleb z profilu
    if (availableAudioTracks.value.includes(profileSettings.value.defaultAudioLanguage)) {
      activeAudioTrack.value = profileSettings.value.defaultAudioLanguage;
    } else {
      activeAudioTrack.value = availableAudioTracks.value[0] || 'CZ';
    }

    if (availableSubtitleTracks.value.includes(profileSettings.value.defaultSubtitleLanguage)) {
      activeSubtitleTrack.value = profileSettings.value.defaultSubtitleLanguage;
    } else {
      activeSubtitleTrack.value = 'off';
    }

    checkMpvStatus();

    // Pokud uživatel preferuje MPV a MPV je nainstalováno, nabídnout či spustit MPV
    if (profileSettings.value.preferredPlayer === 'mpv' && mpvAvailable.value) {
      setTimeout(() => {
        launchMpv();
      }, 100);
      return;
    }

    setTimeout(() => {
      if (videoRef.value) {
        videoRef.value.play().then(() => {
          isPlaying.value = true;
          resetControlsTimeout();
        }).catch(() => {
          isPlaying.value = false;
        });
      }
    }, 200);
  } else {
    if (videoRef.value) {
      videoRef.value.pause();
      isPlaying.value = false;
    }
    if (isMpvPlaying.value) {
      invoke('mpv_stop').catch(() => {});
      isMpvPlaying.value = false;
    }
  }
});

const closePlayer = () => {
  if (videoRef.value) {
    videoRef.value.pause();
    isPlaying.value = false;
  }
  if (isMpvPlaying.value) {
    invoke('mpv_stop').catch(() => {});
    isMpvPlaying.value = false;
  }
  emit('close');
};
</script>

<template>
  <Transition
    enter-active-class="transition duration-300 ease-out"
    enter-from-class="opacity-0 scale-95"
    enter-to-class="opacity-100 scale-100"
    leave-active-class="transition duration-200 ease-in"
    leave-from-class="opacity-100 scale-100"
    leave-to-class="opacity-0 scale-95"
  >
    <div
      v-if="isOpen && item"
      id="player-container"
      @mousemove="handleMouseMove"
      class="fixed inset-0 z-[80] bg-black flex items-center justify-center overflow-hidden select-none font-sans"
    >
      <!-- HTML5 Video Element (pokud neběží MPV) -->
      <video
        ref="videoRef"
        @click="togglePlay"
        @timeupdate="currentTime = videoRef?.currentTime || 0"
        @loadedmetadata="duration = videoRef?.duration || 0"
        @ended="isPlaying = false; showControls = true"
        :src="item.videoUrl || defaultVideoUrl"
        :poster="item.backdropUrl || item.posterUrl"
        class="w-full h-full object-contain cursor-pointer"
        playsinline
      ></video>

      <!-- MPV Aktivní Overlay (když video běží přes MPV proces) -->
      <div
        v-if="isMpvPlaying"
        class="absolute inset-0 bg-neutral-950/90 backdrop-blur-md flex flex-col items-center justify-center p-6 text-center z-10"
      >
        <div class="w-20 h-20 rounded-2xl bg-violet-600/20 border border-violet-500/40 flex items-center justify-center mb-6 shadow-2xl shadow-violet-600/30">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-10 h-10 text-violet-400">
            <path d="M4.5 4.5a3 3 0 00-3 3v9a3 3 0 003 3h8.25a3 3 0 003-3v-9a3 3 0 00-3-3H4.5zM19.94 18.75l-3.19-2.392v-8.716l3.19-2.392a1.5 1.5 0 012.31 1.228v11.044a1.5 1.5 0 01-2.31 1.228z" />
          </svg>
        </div>
        <h3 class="text-2xl font-black text-white tracking-tight">MPV Přehrávač je aktivní</h3>
        <p class="text-neutral-400 text-sm max-w-md mt-2">
          Přehrává se: <span class="text-white font-medium">{{ item.title }}</span> v externím okně MPV s plnou hardwarovou akcelerací a všemi kodeky.
        </p>

        <!-- MPV Remote Controls -->
        <div class="flex items-center gap-4 mt-8 bg-neutral-900/80 p-3 rounded-2xl border border-neutral-800">
          <button
            @click="sendMpvCommand(['seek', -10, 'relative'])"
            class="p-3 rounded-xl bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white transition-colors"
            title="Zpět 10s"
          >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
              <path stroke-linecap="round" stroke-linejoin="round" d="M15 15l-6-6m0 0l6-6m-6 6h12" />
            </svg>
          </button>

          <button
            @click="sendMpvCommand(['cycle', 'pause'])"
            class="px-6 py-3 rounded-xl bg-violet-600 hover:bg-violet-500 text-white font-bold transition-all shadow-lg shadow-violet-600/30 flex items-center gap-2"
          >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
              <path fill-rule="evenodd" d="M6.75 5.25a.75.75 0 01.75.75v12a.75.75 0 01-1.75 0v-12a.75.75 0 01.75-.75zm9 0a.75.75 0 01.75.75v12a.75.75 0 01-1.75 0v-12a.75.75 0 01.75-.75z" clip-rule="evenodd" />
            </svg>
            Pauza / Přehrát
          </button>

          <button
            @click="sendMpvCommand(['seek', 10, 'relative'])"
            class="p-3 rounded-xl bg-neutral-800 hover:bg-neutral-700 text-neutral-300 hover:text-white transition-colors"
            title="Vpřed 10s"
          >
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9 15l6-6m0 0l-6-6m6 6H3" />
            </svg>
          </button>
        </div>

        <div class="flex items-center gap-3 mt-6">
          <button
            @click="stopMpvAndResumeInternal"
            class="px-4 py-2 rounded-xl bg-neutral-800 hover:bg-neutral-700 text-neutral-300 text-xs font-semibold transition-colors border border-neutral-700"
          >
            Zavřít MPV a přehrávat zde
          </button>
          <button
            @click="closePlayer"
            class="px-4 py-2 rounded-xl bg-red-600/20 hover:bg-red-600/30 text-red-400 text-xs font-semibold transition-colors border border-red-500/30"
          >
            Ukončit přehrávání
          </button>
        </div>
      </div>

      <!-- Živé zobrazení titulků na obrazovce (pro vestavěný přehrávač) -->
      <div
        v-if="!isMpvPlaying && currentSubtitleText"
        class="absolute bottom-28 left-0 right-0 z-20 flex justify-center pointer-events-none px-6"
      >
        <span
          :class="{
            'text-base py-1 px-3': profileSettings.subtitleSize === 'small',
            'text-xl py-1.5 px-4': profileSettings.subtitleSize === 'medium',
            'text-2xl py-2 px-6': profileSettings.subtitleSize === 'large',
            'text-3xl py-2.5 px-8': profileSettings.subtitleSize === 'xlarge',
          }"
          class="font-semibold text-white drop-shadow-[0_2px_4px_rgba(0,0,0,1)] bg-black/60 rounded-xl backdrop-blur-sm border border-white/10 text-center max-w-3xl leading-snug tracking-wide"
        >
          {{ currentSubtitleText }}
        </span>
      </div>

      <!-- Ovládací prvky Overlay -->
      <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-300 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-show="showControls"
          class="absolute inset-0 bg-gradient-to-t from-black/90 via-black/20 to-black/80 flex flex-col justify-between p-6 md:p-10 pointer-events-none z-30"
        >
          <!-- Horní lišta: Titulek, Info a Zavřít -->
          <div class="flex items-center justify-between pointer-events-auto">
            <div class="flex items-center gap-4">
              <button
                @click="closePlayer"
                class="p-2.5 rounded-full bg-neutral-900/70 hover:bg-neutral-800 text-neutral-300 hover:text-white transition-all backdrop-blur-md border border-neutral-700/50 hover:scale-105"
                title="Zavřít přehrávač (ESC)"
              >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                </svg>
              </button>
              <div>
                <h3 class="text-xl md:text-2xl font-black text-white tracking-tight drop-shadow-md">
                  {{ item.title }}
                </h3>
                <div class="flex items-center gap-2 text-xs font-medium text-neutral-400 mt-0.5">
                  <span>{{ item.year }}</span>
                  <span>•</span>
                  <span>{{ item.genre }}</span>
                  <span v-if="currentChapter" class="text-violet-400 font-semibold">• {{ currentChapter.title }}</span>
                  <span class="border border-neutral-600 px-1 rounded text-[10px] text-neutral-300">1080p HD</span>
                </div>
              </div>
            </div>

            <!-- Pravá strana horní lišty: MPV Tlačítko & Zavřít -->
            <div class="flex items-center gap-3">
              <button
                @click="launchMpv"
                :class="isMpvPlaying ? 'bg-violet-600 text-white border-violet-400 shadow-violet-600/40' : 'bg-neutral-900/80 text-neutral-200 hover:text-white hover:bg-neutral-800 border-neutral-700/60'"
                class="px-3.5 py-2 rounded-xl text-xs font-bold transition-all backdrop-blur-md border flex items-center gap-2 shadow-lg"
                title="Přepnout do přehrávače MPV"
              >
                <span class="w-2 h-2 rounded-full" :class="isMpvPlaying ? 'bg-emerald-400 animate-pulse' : 'bg-violet-400'"></span>
                {{ isMpvPlaying ? 'Běží v MPV' : 'Spustit v MPV' }}
              </button>

              <button
                @click="closePlayer"
                class="p-2 rounded-full bg-neutral-900/70 hover:bg-red-500/80 text-neutral-400 hover:text-white transition-all backdrop-blur-md border border-neutral-700/50"
              >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Střední tlačítko Přehrát/Pauza (když je pozastaveno a neběží MPV) -->
          <div v-if="!isPlaying && !isMpvPlaying" class="flex items-center justify-center pointer-events-auto">
            <button
              @click="togglePlay"
              class="w-20 h-20 rounded-full bg-violet-600/90 hover:bg-violet-500 text-white flex items-center justify-center transition-all transform hover:scale-110 shadow-2xl shadow-violet-600/50"
            >
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-10 h-10 ml-1">
                <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>

          <!-- Dolní panel: Timeline, Zvuk, Kapitoly, Titulky, Fullscreen -->
          <div class="flex flex-col gap-3 pointer-events-auto max-w-5xl mx-auto w-full relative">
            
            <!-- POPUP MENUS (Audio, Titulky, Kapitoly) -->
            <!-- Menu pro Zvukové stopy -->
            <div
              v-if="showAudioMenu"
              class="absolute bottom-16 left-48 z-50 w-56 bg-neutral-900/95 backdrop-blur-xl border border-neutral-700/80 rounded-2xl shadow-2xl p-3 flex flex-col gap-1.5 animate-in fade-in zoom-in-95 duration-150"
            >
              <div class="text-[11px] font-bold text-neutral-400 uppercase tracking-wider px-2 py-1 border-b border-neutral-800">
                Zvuková stopa (Audio)
              </div>
              <button
                v-for="track in availableAudioTracks"
                :key="track"
                @click="selectAudioTrack(track)"
                :class="activeAudioTrack === track ? 'bg-violet-600 text-white font-semibold' : 'text-neutral-300 hover:bg-neutral-800'"
                class="w-full text-left px-3 py-2 rounded-xl text-xs flex items-center justify-between transition-colors"
              >
                <span>{{ track === 'CZ' ? '🇨🇿 Čeština' : track === 'EN' ? '🇬🇧 Angličtina' : track === 'SK' ? '🇸🇰 Slovenština' : track }}</span>
                <span v-if="activeAudioTrack === track" class="text-white text-xs">✓</span>
              </button>
            </div>

            <!-- Menu pro Titulky a Velikost -->
            <div
              v-if="showSubtitleMenu"
              class="absolute bottom-16 left-64 z-50 w-72 bg-neutral-900/95 backdrop-blur-xl border border-neutral-700/80 rounded-2xl shadow-2xl p-4 flex flex-col gap-3 animate-in fade-in zoom-in-95 duration-150"
            >
              <div>
                <div class="text-[11px] font-bold text-neutral-400 uppercase tracking-wider mb-2">
                  Titulky
                </div>
                <div class="flex flex-col gap-1">
                  <button
                    v-for="sub in availableSubtitleTracks"
                    :key="sub"
                    @click="selectSubtitleTrack(sub)"
                    :class="activeSubtitleTrack === sub ? 'bg-violet-600 text-white font-semibold' : 'text-neutral-300 hover:bg-neutral-800'"
                    class="w-full text-left px-3 py-1.5 rounded-xl text-xs flex items-center justify-between transition-colors"
                  >
                    <span>
                      {{ sub === 'off' ? '🚫 Vypnuto' : sub === 'CZ' ? '🇨🇿 Čeština' : sub === 'EN' ? '🇬🇧 Angličtina' : sub === 'SK' ? '🇸🇰 Slovenština' : sub }}
                    </span>
                    <span v-if="activeSubtitleTrack === sub" class="text-white text-xs">✓</span>
                  </button>
                </div>
              </div>

              <!-- Nastavení velikosti písma -->
              <div class="pt-3 border-t border-neutral-800">
                <div class="text-[11px] font-bold text-neutral-400 uppercase tracking-wider mb-2 flex items-center justify-between">
                  <span>Velikost písma</span>
                  <span class="text-[10px] text-violet-400 font-normal">Uloženo v profilu</span>
                </div>
                <div class="grid grid-cols-2 gap-1.5">
                  <button
                    v-for="s in [
                      { id: 'small', label: 'Malé (75%)' },
                      { id: 'medium', label: 'Normální (100%)' },
                      { id: 'large', label: 'Velké (125%)' },
                      { id: 'xlarge', label: 'Extra (150%)' }
                    ] as const"
                    :key="s.id"
                    @click="setPlayerSubtitleSize(s.id)"
                    :class="profileSettings.subtitleSize === s.id ? 'bg-violet-600 text-white font-semibold' : 'bg-neutral-800 text-neutral-300 hover:bg-neutral-700'"
                    class="px-2.5 py-1.5 text-xs rounded-xl transition-all text-center"
                  >
                    {{ s.label }}
                  </button>
                </div>
              </div>
            </div>

            <!-- Menu pro Kapitoly -->
            <div
              v-if="showChaptersMenu"
              class="absolute bottom-16 right-20 z-50 w-72 bg-neutral-900/95 backdrop-blur-xl border border-neutral-700/80 rounded-2xl shadow-2xl p-3 flex flex-col gap-1.5 max-h-72 overflow-y-auto animate-in fade-in zoom-in-95 duration-150"
            >
              <div class="text-[11px] font-bold text-neutral-400 uppercase tracking-wider px-2 py-1 border-b border-neutral-800 flex items-center justify-between">
                <span>Kapitoly</span>
                <span class="text-neutral-500 font-mono text-[10px]">{{ chapters.length }}</span>
              </div>
              <button
                v-for="c in chapters"
                :key="c.id"
                @click="seekToChapter(c)"
                :class="currentChapter?.id === c.id ? 'bg-violet-600/30 text-violet-300 border border-violet-500/40 font-semibold' : 'text-neutral-300 hover:bg-neutral-800/80'"
                class="w-full text-left px-3 py-2 rounded-xl text-xs flex items-center justify-between transition-colors"
              >
                <span class="truncate pr-2">{{ c.title }}</span>
                <span class="text-neutral-400 font-mono text-[10px] shrink-0">{{ formatTime(c.startTime) }}</span>
              </button>
            </div>

            <!-- Timeline / Seekbar s Chapter Markery -->
            <div class="flex items-center gap-3">
              <span class="text-xs font-mono text-neutral-300 w-12 text-right">
                {{ formatTime(currentTime) }}
              </span>
              <div class="relative flex-grow flex items-center group">
                <!-- Chapter Markery na ose -->
                <div class="absolute inset-x-0 h-1.5 pointer-events-none z-10 flex">
                  <div
                    v-for="c in chapters"
                    :key="c.id"
                    v-show="c.startTime > 0 && duration > 0"
                    class="absolute top-0 bottom-0 w-0.5 bg-white/50 rounded-full"
                    :style="{ left: `${(c.startTime / duration) * 100}%` }"
                    :title="`${c.title} (${formatTime(c.startTime)})`"
                  ></div>
                </div>

                <input
                  type="range"
                  min="0"
                  :max="duration || 100"
                  step="0.1"
                  :value="currentTime"
                  @input="seek"
                  class="w-full h-1.5 bg-neutral-700/80 rounded-lg appearance-none cursor-pointer accent-violet-500 group-hover:h-2.5 transition-all relative z-20"
                />
              </div>
              <span class="text-xs font-mono text-neutral-400 w-12">
                {{ formatTime(duration) }}
              </span>
            </div>

            <!-- Ovládací tlačítka dole -->
            <div class="flex items-center justify-between">
              <!-- Levá část: Přehrát, Přeskočit, Hlasitost, Kapitoly skok -->
              <div class="flex items-center gap-3">
                <!-- Play / Pause -->
                <button @click="togglePlay" class="text-white hover:text-violet-400 transition-colors p-1" title="Přehrát / Pozastavit (Mezerník)">
                  <svg v-if="!isPlaying && !isMpvPlaying" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7">
                    <path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" />
                  </svg>
                  <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7">
                    <path fill-rule="evenodd" d="M6.75 5.25a.75.75 0 01.75.75v12a.75.75 0 01-1.75 0v-12a.75.75 0 01.75-.75zm9 0a.75.75 0 01.75.75v12a.75.75 0 01-1.75 0v-12a.75.75 0 01.75-.75z" clip-rule="evenodd" />
                  </svg>
                </button>

                <!-- Předchozí kapitola -->
                <button @click="prevChapter" class="text-neutral-400 hover:text-white transition-colors" title="Předchozí kapitola (P)">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 16.811c0 .864-.933 1.405-1.683.977l-7.108-4.062a1.125 1.125 0 010-1.953l7.108-4.062A1.125 1.125 0 0121 8.688v8.123zM11.25 16.811c0 .864-.933 1.405-1.683.977l-7.108-4.062a1.125 1.125 0 010-1.953L9.567 7.71a1.125 1.125 0 011.683.977v8.123z" />
                  </svg>
                </button>

                <!-- -10s -->
                <button @click="skip(-10)" class="text-neutral-400 hover:text-white transition-colors" title="Zpět 10s">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 15l-6-6m0 0l6-6m-6 6h12" />
                  </svg>
                </button>

                <!-- +10s -->
                <button @click="skip(10)" class="text-neutral-400 hover:text-white transition-colors" title="Vpřed 10s">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 15l6-6m0 0l-6-6m6 6H3" />
                  </svg>
                </button>

                <!-- Další kapitola -->
                <button @click="nextChapter" class="text-neutral-400 hover:text-white transition-colors" title="Další kapitola (N)">
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 8.688c0-.864.933-1.405 1.683-.977l7.108 4.062a1.125 1.125 0 010 1.953l-7.108 4.062A1.125 1.125 0 013 16.812V8.688zM12.75 8.688c0-.864.933-1.405 1.683-.977l7.108 4.062a1.125 1.125 0 010 1.953l-7.108 4.062a1.125 1.125 0 01-1.683-.977V8.688z" />
                  </svg>
                </button>

                <!-- Hlasitost -->
                <div class="flex items-center gap-2 ml-2">
                  <button @click="toggleMute" class="text-neutral-300 hover:text-white transition-colors">
                    <svg v-if="isMuted || volume === 0" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
                      <path d="M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.5A2.25 2.25 0 002.25 9.75v4.5a2.25 2.25 0 002.25 2.25h1.94l4.5 4.5c.944.945 2.56.276 2.56-1.06V4.06zM17.28 9.28a.75.75 0 00-1.06 1.06L18.44 12l-2.22 2.22a.75.75 0 101.06 1.06L19.5 13.06l2.22 2.22a.75.75 0 101.06-1.06L20.56 12l2.22-2.22a.75.75 0 00-1.06-1.06L19.5 10.94l-2.22-2.22z" />
                    </svg>
                    <svg v-else xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-5 h-5">
                      <path d="M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.5A2.25 2.25 0 002.25 9.75v4.5a2.25 2.25 0 002.25 2.25h1.94l4.5 4.5c.944.945 2.56.276 2.56-1.06V4.06zM18.54 5.46a.75.75 0 011.06 0 11.25 11.25 0 010 15.91.75.75 0 01-1.06-1.06 9.75 9.75 0 000-13.79.75.75 0 010-1.06z" />
                      <path d="M16.41 7.59a.75.75 0 011.06 0 8.25 8.25 0 010 11.66.75.75 0 11-1.06-1.06 6.75 6.75 0 000-9.54.75.75 0 010-1.06z" />
                    </svg>
                  </button>
                  <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.05"
                    :value="isMuted ? 0 : volume"
                    @input="handleVolumeChange"
                    class="w-16 h-1 bg-neutral-700 rounded appearance-none cursor-pointer accent-violet-500"
                  />
                </div>
              </div>

              <!-- Pravá část: Audio, Titulky, Kapitoly, Fullscreen -->
              <div class="flex items-center gap-2">
                <!-- Audio stopa tlačítko -->
                <button
                  @click="showAudioMenu = !showAudioMenu; showSubtitleMenu = false; showChaptersMenu = false"
                  :class="showAudioMenu ? 'bg-violet-600 text-white' : 'bg-neutral-900/60 text-neutral-300 hover:text-white hover:bg-neutral-800'"
                  class="px-2.5 py-1.5 rounded-xl text-xs font-semibold border border-neutral-700/60 transition-all flex items-center gap-1.5"
                  title="Vybrat zvukovou stopu"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.114 5.636a9 9 0 010 12.728M16.463 8.288a5.25 5.25 0 010 7.424M6.75 8.25l4.72-4.72a.75.75 0 011.28.53v15.88a.75.75 0 01-1.28.53l-4.72-4.72H4.51c-.88 0-1.704-.507-1.938-1.354A9.01 9.01 0 012.25 12c0-.83.112-1.633.322-2.396C2.806 8.757 3.63 8.25 4.51 8.25H6.75z" />
                  </svg>
                  <span>{{ activeAudioTrack }}</span>
                </button>

                <!-- Titulky tlačítko -->
                <button
                  @click="showSubtitleMenu = !showSubtitleMenu; showAudioMenu = false; showChaptersMenu = false"
                  :class="showSubtitleMenu ? 'bg-violet-600 text-white' : activeSubtitleTrack !== 'off' ? 'bg-violet-600/30 text-violet-300 border-violet-500/50' : 'bg-neutral-900/60 text-neutral-300 hover:text-white hover:bg-neutral-800'"
                  class="px-2.5 py-1.5 rounded-xl text-xs font-semibold border border-neutral-700/60 transition-all flex items-center gap-1.5"
                  title="Nastavení titulků a velikosti"
                >
                  <span class="font-mono text-[10px] font-black border border-current px-1 rounded">CC</span>
                  <span>{{ activeSubtitleTrack === 'off' ? 'Vyp' : activeSubtitleTrack }}</span>
                </button>

                <!-- Kapitoly tlačítko -->
                <button
                  @click="showChaptersMenu = !showChaptersMenu; showAudioMenu = false; showSubtitleMenu = false"
                  :class="showChaptersMenu ? 'bg-violet-600 text-white' : 'bg-neutral-900/60 text-neutral-300 hover:text-white hover:bg-neutral-800'"
                  class="px-2.5 py-1.5 rounded-xl text-xs font-semibold border border-neutral-700/60 transition-all flex items-center gap-1.5"
                  title="Seznam kapitol"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-4 h-4">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" />
                  </svg>
                  <span>Kapitoly</span>
                </button>

                <!-- Celá obrazovka -->
                <button
                  @click="toggleFullScreen"
                  class="p-2 text-neutral-300 hover:text-white hover:bg-neutral-800/80 rounded-xl transition-colors ml-1"
                  title="Celá obrazovka"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-5 h-5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15" />
                  </svg>
                </button>
              </div>
            </div>

          </div>
        </div>
      </Transition>

      <!-- Modal upozornění: MPV není nainstalováno -->
      <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0 scale-95"
        enter-to-class="opacity-100 scale-100"
        leave-active-class="transition duration-150 ease-in"
        leave-from-class="opacity-100 scale-100"
        leave-to-class="opacity-0 scale-95"
      >
        <div
          v-if="showMpvInstallModal"
          class="fixed inset-0 z-[100] flex items-center justify-center bg-black/80 backdrop-blur-md p-4"
          @click.self="showMpvInstallModal = false"
        >
          <div class="bg-neutral-900 border border-neutral-700/80 rounded-2xl max-w-md w-full p-6 shadow-2xl">
            <div class="flex items-center gap-3 text-amber-400 mb-3">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
              </svg>
              <h4 class="text-lg font-bold text-white">MPV není nainstalováno</h4>
            </div>
            <p class="text-neutral-300 text-sm mb-4">
              Přehrávač <code class="text-violet-400 bg-neutral-800 px-1 py-0.5 rounded font-mono">mpv</code> nebyl nalezen v systému.
            </p>
            <div class="bg-neutral-950 p-3 rounded-xl border border-neutral-800 font-mono text-xs text-neutral-300 mb-4 select-all">
              # Instalace na macOS:<br>
              <span class="text-emerald-400 font-bold">brew install mpv</span>
            </div>
            <p class="text-neutral-400 text-xs mb-6">
              Můžete video přehrát ve vestavěném přehrávači, nebo jej otevřít v systémovém přehrávači (VLC).
            </p>
            <div class="flex items-center justify-end gap-3">
              <button
                @click="showMpvInstallModal = false"
                class="px-4 py-2 rounded-xl bg-neutral-800 hover:bg-neutral-700 text-neutral-300 text-xs font-semibold transition-colors"
              >
                Přehrát zde (Vestavěný)
              </button>
              <button
                @click="openInExternalSystemPlayer"
                class="px-4 py-2 rounded-xl bg-violet-600 hover:bg-violet-500 text-white text-xs font-bold transition-all shadow-md shadow-violet-600/30"
              >
                Otevřít ve VLC / Systému
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </Transition>
</template>
