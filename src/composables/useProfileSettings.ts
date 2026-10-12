import { ref, watch } from 'vue';

export interface ProfileSettings {
  defaultAudioLanguage: string; // 'CZ', 'EN', 'ORIGINAL'
  defaultSubtitleLanguage: string; // 'off', 'CZ', 'EN', 'SK'
  subtitleSize: 'small' | 'medium' | 'large' | 'xlarge'; // 75%, 100%, 125%, 150%
  preferredPlayer: 'mpv' | 'internal';
}

const STORAGE_KEY = 'movieapp_profile_settings';

const defaultSettings: ProfileSettings = {
  defaultAudioLanguage: 'CZ',
  defaultSubtitleLanguage: 'off',
  subtitleSize: 'medium',
  preferredPlayer: 'mpv',
};

function loadSettings(): ProfileSettings {
  if (typeof window === 'undefined') return defaultSettings;
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) {
      return { ...defaultSettings, ...JSON.parse(raw) };
    }
  } catch (e) {
    console.error('Failed to load profile settings from localStorage:', e);
  }
  return defaultSettings;
}

const settings = ref<ProfileSettings>(loadSettings());

// Automatické ukládání při jakékoliv změně
watch(
  settings,
  (newVal) => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(newVal));
    } catch (e) {
      console.error('Failed to save profile settings:', e);
    }
  },
  { deep: true }
);

export function useProfileSettings() {
  const setSubtitleSize = (size: ProfileSettings['subtitleSize']) => {
    settings.value.subtitleSize = size;
  };

  const setDefaultAudio = (lang: string) => {
    settings.value.defaultAudioLanguage = lang;
  };

  const setDefaultSubtitles = (lang: string) => {
    settings.value.defaultSubtitleLanguage = lang;
  };

  const setPreferredPlayer = (player: 'mpv' | 'internal') => {
    settings.value.preferredPlayer = player;
  };

  // Helper pro výpočet pixelové velikosti pro MPV a HTML5
  const getSubtitleFontSizePixels = (size?: ProfileSettings['subtitleSize']) => {
    const s = size || settings.value.subtitleSize;
    switch (s) {
      case 'small':
        return 28;
      case 'medium':
        return 38;
      case 'large':
        return 48;
      case 'xlarge':
        return 58;
      default:
        return 38;
    }
  };

  return {
    settings,
    setSubtitleSize,
    setDefaultAudio,
    setDefaultSubtitles,
    setPreferredPlayer,
    getSubtitleFontSizePixels,
  };
}
