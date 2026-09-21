import { MediaItem, Category } from '../types';

export const featuredMovie: MediaItem = {
  id: 'm1',
  title: 'THE SHIMMER',
  description: 'Skupina vědců vstupuje do anomální zóny zvané Lesk, kde neplatí zákony přírody, biologie ani fyziky. Co najdou uvnitř, navždy změní lidstvo.',
  year: 2026,
  genre: 'Sci-Fi / Thriller',
  posterUrl: 'https://images.unsplash.com/photo-1618331835717-801e976710b2?q=80&w=800&auto=format&fit=crop',
  backdropUrl: 'https://images.unsplash.com/photo-1618331835717-801e976710b2?q=80&w=1920&auto=format&fit=crop',
  type: 'movie',
  duration: '1h 55m',
  ageRating: '18+',
  contentWarnings: 'násilí, hrubý jazyk',
  cast: ['Natalie Portman', 'Oscar Isaac', 'Jennifer Jason Leigh'],
  genres: ['Sci-Fi', 'Thriller', 'Mysteriózní'],
  moods: ['Temné', 'Napínavé', 'Mind-bending']
};

export const featuredSeries: MediaItem = {
  id: 's1',
  title: 'CYBER CITY',
  description: 'V neónem ozářeném městě budoucnosti se detektiv snaží rozplést síť korporátních konspirací.',
  year: 2025,
  genre: 'Sci-Fi / Akční',
  posterUrl: 'https://images.unsplash.com/photo-1518773553398-650c184e0bb3?q=80&w=800&auto=format&fit=crop',
  backdropUrl: 'https://images.unsplash.com/photo-1518773553398-650c184e0bb3?q=80&w=1920&auto=format&fit=crop',
  type: 'series',
  duration: '3 Série',
  ageRating: '15+',
  contentWarnings: 'násilí, drogy',
  cast: ['Keanu Reeves', 'Carrie-Anne Moss'],
  genres: ['Sci-Fi', 'Akční', 'Kyberpunk'],
  moods: ['Akční', 'Drsné']
};

export const moviesList: MediaItem[] = [
  { id: 'm2', title: 'Blade Runner 2049', description: '', year: 2017, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm3', title: 'Interstellar', description: '', year: 2014, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1444703686981-a3abbc4d4fe3?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm4', title: 'Drive', description: '', year: 2011, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm5', title: 'Joker', description: '', year: 2019, genre: 'Drama', posterUrl: 'https://images.unsplash.com/photo-1620336655055-088d06e36bf0?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm6', title: 'Mad Max', description: '', year: 2015, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1542362567-b07e54358753?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm7', title: 'Dune', description: '', year: 2021, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1547333590-478e7279b940?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm8', title: 'John Wick', description: '', year: 2014, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1566373767864-16a3a7891825?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm9', title: 'The Matrix', description: '', year: 1999, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm10', title: 'Inception', description: '', year: 2010, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'm11', title: 'Terminator 2', description: '', year: 1991, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1535295972055-1c762f4483e5?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
];

export const spidermanCollection: MediaItem[] = [
  { id: 'sp1', title: 'Spider-Man', description: 'Peter Parker získá pavoučí schopnosti.', year: 2002, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'sp2', title: 'Spider-Man 2', description: 'Peter Parker čelí Dr. Octopusovi.', year: 2004, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'sp3', title: 'Spider-Man 3', description: 'Peter bojuje s temnotou v sobě a s Venomem.', year: 2007, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'sp4', title: 'The Amazing Spider-Man', description: 'Reboot s Andrewem Garfieldem.', year: 2012, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'sp5', title: 'Spider-Man: Homecoming', description: 'Peter Parker v Marvel Cinematic Universe.', year: 2017, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'sp6', title: 'Spider-Man: Far From Home', description: 'Peter na školním výletě v Evropě.', year: 2019, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'sp7', title: 'Spider-Man: No Way Home', description: 'Multivesmír se otevírá.', year: 2021, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1635805737707-575885ab0820?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
];

export const matrixCollection: MediaItem[] = [
  { id: 'm9', title: 'The Matrix', description: 'Co je Matrix?', year: 1999, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'mx2', title: 'The Matrix Reloaded', description: 'Neo se učí ovládat své schopnosti.', year: 2003, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'mx3', title: 'The Matrix Revolutions', description: 'Závěrečná bitva o Zion.', year: 2003, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'mx4', title: 'The Matrix Resurrections', description: 'Návrat ke kořenům v nové formě.', year: 2021, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1526304640581-d334cdbbf45e?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
];

export const batmanCollection: MediaItem[] = [
  { id: 'bm1', title: 'Batman Begins', description: 'Zrození temného rytíře.', year: 2005, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'bm2', title: 'The Dark Knight', description: 'Joker rozpoutává chaos.', year: 2008, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
  { id: 'bm3', title: 'The Dark Knight Rises', description: 'Poslední vzdor.', year: 2012, genre: 'Action', posterUrl: 'https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'movie' },
];

// Add some more random series
export const seriesList: MediaItem[] = [
  { id: 's2', title: 'Stranger Things', description: '', year: 2016, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1614145121029-83a9f7b68bf4?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'series' },
  { id: 's3', title: 'The Last of Us', description: '', year: 2023, genre: 'Drama', posterUrl: 'https://images.unsplash.com/photo-1605806616949-1e87b487cb2a?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'series' },
  { id: 's4', title: 'Breaking Bad', description: '', year: 2008, genre: 'Crime', posterUrl: 'https://images.unsplash.com/photo-1587314645258-202d08a0d4c8?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'series' },
  { id: 's5', title: 'Dark', description: '', year: 2017, genre: 'Mystery', posterUrl: 'https://images.unsplash.com/photo-1509347528160-9a9e33742cdb?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'series' },
  { id: 's6', title: 'The Mandalorian', description: 'Bounty hunter v předaleké galaxii.', year: 2019, genre: 'Sci-Fi', posterUrl: 'https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'series' },
  { id: 's7', title: 'Arcane', description: 'Dvě sestry, dvě města, jedna válka.', year: 2021, genre: 'Animation', posterUrl: 'https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop', backdropUrl: '', type: 'series' },
];

export const wishlistData: MediaItem[] = [
  moviesList[0],
  seriesList[1],
  moviesList[4],
  spidermanCollection[0],
  spidermanCollection[6]
];

export const movieCategories: Category[] = [
  {
    title: 'Nedávno přidáno',
    items: moviesList.slice(0, 6)
  },
  {
    title: 'Akční nářez',
    items: moviesList.slice(6)
  },
  {
    title: 'Noví hrdinové',
    items: [spidermanCollection[6], batmanCollection[2], matrixCollection[3]]
  }
];

export const seriesCategories: Category[] = [
  {
    title: 'Populární seriály',
    items: seriesList
  }
];

export const homeCategories: Category[] = [
  movieCategories[0],
  seriesCategories[0],
  movieCategories[1]
];

export const allCollections: Category[] = [
  {
    title: 'Kolekce: Spider-Man (Popořadě)',
    items: spidermanCollection
  },
  {
    title: 'Kolekce: The Matrix',
    items: matrixCollection
  },
  {
    title: 'Kolekce: Temný rytíř (Batman)',
    items: batmanCollection
  }
];
