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
}

export interface Category {
  title: string;
  items: MediaItem[];
}
