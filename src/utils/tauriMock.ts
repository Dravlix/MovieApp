import type { MediaItem, Category } from '../types';

export const mockMovies: MediaItem[] = [
  {
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
    moods: ['Temné', 'Napínavé', 'Mind-bending'],
    rating: 8.6,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN', 'SK'],
  },
  {
    id: 'm2',
    title: 'Blade Runner 2049',
    description: 'Mladý Blade Runner odhaluje dlouho pohřbené tajemství, které může uvrhnout zbytek společnosti do chaosu.',
    year: 2017,
    genre: 'Sci-Fi',
    posterUrl: 'https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1534447677768-be436bb09401?q=80&w=1920&auto=format&fit=crop',
    type: 'movie',
    duration: '2h 44m',
    rating: 8.0,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
  {
    id: 'm3',
    title: 'Interstellar',
    description: 'Tým výzkumníků cestuje červí dírou v prostoru ve snaze zajistit přežití lidstva.',
    year: 2014,
    genre: 'Sci-Fi',
    posterUrl: 'https://images.unsplash.com/photo-1444703686981-a3abbc4d4fe3?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1444703686981-a3abbc4d4fe3?q=80&w=1920&auto=format&fit=crop',
    type: 'movie',
    duration: '2h 49m',
    rating: 8.7,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN', 'SK'],
  },
  {
    id: 'm4',
    title: 'Drive',
    description: 'Tajemný hollywoodský kaskadér a noční řidič pro zločince se zaplete do nebezpečné loupeže.',
    year: 2011,
    genre: 'Akční',
    posterUrl: 'https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1492144534655-ae79c964c9d7?q=80&w=1920&auto=format&fit=crop',
    type: 'movie',
    duration: '1h 40m',
    rating: 7.8,
    audioLanguages: ['EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
  {
    id: 'm5',
    title: 'Joker',
    description: 'Zkrachovalý komik Arthur Fleck se propadá do šílenství a mění se v ikonu kriminálního podsvětí.',
    year: 2019,
    genre: 'Drama',
    posterUrl: 'https://images.unsplash.com/photo-1620336655055-088d06e36bf0?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1620336655055-088d06e36bf0?q=80&w=1920&auto=format&fit=crop',
    type: 'movie',
    duration: '2h 02m',
    rating: 8.4,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ'],
  },
  {
    id: 'm6',
    title: 'Dune: Part Two',
    description: 'Paul Atreides se spojuje s Fremeny na válečné stezce pomsty proti spiklencům, kteří zničili jeho rodinu.',
    year: 2024,
    genre: 'Sci-Fi',
    posterUrl: 'https://images.unsplash.com/photo-1547333590-478e7279b940?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1547333590-478e7279b940?q=80&w=1920&auto=format&fit=crop',
    type: 'movie',
    duration: '2h 46m',
    rating: 8.6,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
  {
    id: 'm7',
    title: 'John Wick: Kapitola 4',
    description: 'John Wick nachází cestu, jak porazit Nejvyšší radu, ale předtím musí čelit novému nepříteli.',
    year: 2023,
    genre: 'Akční',
    posterUrl: 'https://images.unsplash.com/photo-1566373767864-16a3a7891825?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1566373767864-16a3a7891825?q=80&w=1920&auto=format&fit=crop',
    type: 'movie',
    duration: '2h 49m',
    rating: 7.7,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
];

export const mockSeries: MediaItem[] = [
  {
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
    moods: ['Akční', 'Drsné'],
    rating: 8.9,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
  {
    id: 's2',
    title: 'Stranger Things',
    description: 'Když mladý chlapec záhadně zmizí, jeho matka a přátelé musí čelit děsivým nadpřirozeným silám.',
    year: 2016,
    genre: 'Sci-Fi',
    posterUrl: 'https://images.unsplash.com/photo-1614145121029-83a9f7b68bf4?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1614145121029-83a9f7b68bf4?q=80&w=1920&auto=format&fit=crop',
    type: 'series',
    duration: '4 Série',
    rating: 8.7,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
  {
    id: 's3',
    title: 'The Last of Us',
    description: 'Po pádu globální civilizace je otrlý přeživší pověřen ochranou 14leté dívky, která může být poslední nadějí lidstva.',
    year: 2023,
    genre: 'Drama',
    posterUrl: 'https://images.unsplash.com/photo-1605806616949-1e87b487cb2a?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1605806616949-1e87b487cb2a?q=80&w=1920&auto=format&fit=crop',
    type: 'series',
    duration: '1 Série',
    rating: 8.8,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
  {
    id: 's4',
    title: 'Arcane',
    description: 'Dvě sestry bojují na opačných stranách konfliktu v technologicky vyspělém městě a jeho temném podzemí.',
    year: 2021,
    genre: 'Animovaný',
    posterUrl: 'https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=500&auto=format&fit=crop',
    backdropUrl: 'https://images.unsplash.com/photo-1618042164219-62c820f10723?q=80&w=1920&auto=format&fit=crop',
    type: 'series',
    duration: '2 Série',
    rating: 9.0,
    audioLanguages: ['CZ', 'EN'],
    subtitleLanguages: ['CZ', 'EN'],
  },
];

const mockWishlistIds = new Set(['m1', 's1', 'm3']);

export function setupBrowserTauriMock() {
  if (typeof window === 'undefined') return;

  const win = window as any;
  if (!win.__TAURI_INTERNALS__) {
    win.__TAURI_INTERNALS__ = {
      invoke: async (cmd: string, args: any = {}) => {
        switch (cmd) {
          case 'get_home_data':
            return [
              { title: 'Nedávno přidáno', items: mockMovies },
              { title: 'Populární seriály', items: mockSeries },
            ] as Category[];
          case 'get_movies':
            return mockMovies;
          case 'get_series':
            return mockSeries;
          case 'get_all_collections':
            return [
              { title: 'Kolekce: Sci-Fi Klasiky', items: mockMovies.filter(m => m.genre.includes('Sci-Fi')) },
            ];
          case 'get_collection':
            return mockMovies.slice(0, 4);
          case 'get_wishlist':
            return [...mockMovies, ...mockSeries].filter(item => mockWishlistIds.has(item.id));
          case 'check_wishlist':
            return mockWishlistIds.has(args.id);
          case 'toggle_wishlist': {
            if (mockWishlistIds.has(args.id)) {
              mockWishlistIds.delete(args.id);
              return false;
            } else {
              mockWishlistIds.add(args.id);
              return true;
            }
          }
          case 'get_scan_folders':
            return ['/Users/dominik-mac/Filmy', '/Users/dominik-mac/Downloads/Movies'];
          case 'add_scan_folder':
            return null;
          case 'remove_scan_folder':
            return null;
          case 'scan_library':
            return {
              scannedFolders: 2,
              filesFound: 6,
              newAdded: 3,
              newTitles: ['Oppenheimer (2023)', 'Dune: Part Two (2024)', 'Interstellar (2014)'],
            };
          case 'open_media_file':
            console.log('Otevírám soubor v přehrávači:', args.path);
            return null;
          case 'check_mpv_status':
            return {
              is_installed: true,
              is_running: false,
              path: '/usr/local/bin/mpv',
            };
          case 'mpv_play':
            console.log('MPV Play:', args);
            return null;
          case 'mpv_command':
            console.log('MPV Command:', args);
            return null;
          case 'mpv_stop':
            console.log('MPV Stop');
            return null;
          default:
            console.warn('Unhandled mock invoke command:', cmd, args);
            return null;
        }
      },
    };
  }
}
