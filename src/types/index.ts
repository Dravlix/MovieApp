export interface Episode {
  id: string;
  episodeNumber: number;
  title: string;
  duration: string;
  overview: string;
  stillUrl?: string;
}

export interface Season {
  seasonNumber: number;
  title: string;
  episodes: Episode[];
}

export interface Chapter {
  id: string;
  title: string;
  startTime: number; // sekundy
}

export interface MediaItem {
  id: string;
  title: string;
  description: string;
  year: number;
  genre: string;
  posterUrl: string;
  backdropUrl: string;
  type?: 'movie' | 'series';
  duration?: string;
  ageRating?: string;
  contentWarnings?: string;
  cast?: string[];
  genres?: string[];
  moods?: string[];
  rating?: number; // např. 8.4
  trailerUrl?: string;
  videoUrl?: string;
  watchProgress?: number; // 0 - 100
  audioLanguages?: string[]; // např. ['CZ', 'EN']
  subtitleLanguages?: string[]; // např. ['CZ', 'EN', 'SK']
  localFilePath?: string;
  seasons?: Season[];
  chapters?: Chapter[];
}

export interface Category {
  title: string;
  items: MediaItem[];
}

