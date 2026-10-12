import puppeteer from 'puppeteer-core';
import fs from 'fs';
import path from 'path';

const outDir = '/tmp/movieapp_tests';
if (!fs.existsSync(outDir)) {
  fs.mkdirSync(outDir, { recursive: true });
}

async function run() {
  console.log('🚀 Launching Brave Browser in headless mode...');
  const browser = await puppeteer.launch({
    executablePath: '/Applications/Brave Browser.app/Contents/MacOS/Brave Browser',
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox', '--window-size=1440,900'],
    defaultViewport: { width: 1440, height: 900 },
  });

  const page = await browser.newPage();
  console.log('🌐 Opening http://localhost:1420 ...');
  await page.goto('http://localhost:1420', { waitUntil: 'networkidle0' });
  await new Promise(r => setTimeout(r, 1000));

  // 1. Home Page
  console.log('📸 1. Capturing Home page...');
  await page.screenshot({ path: path.join(outDir, '01_home.png') });

  // 2. Click "Filmy"
  console.log('👉 2. Clicking "Filmy"...');
  await page.evaluate(() => {
    const items = Array.from(document.querySelectorAll('li'));
    const moviesLi = items.find(el => el.textContent?.trim() === 'Filmy');
    moviesLi?.click();
  });
  await new Promise(r => setTimeout(r, 600));
  await page.screenshot({ path: path.join(outDir, '02_movies_catalog.png') });

  // 3. Filter by Sci-Fi
  console.log('👉 3. Filtering by Sci-Fi genre...');
  await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll('button'));
    const scifi = buttons.find(b => b.textContent?.trim() === 'Sci-Fi');
    scifi?.click();
  });
  await new Promise(r => setTimeout(r, 400));
  await page.screenshot({ path: path.join(outDir, '03_movies_filtered_scifi.png') });

  // 4. Click a movie to open DetailView
  console.log('👉 4. Opening Movie Detail...');
  await page.evaluate(() => {
    const card = document.querySelector('.group.relative.cursor-pointer');
    if (card) card.click();
  });
  await new Promise(r => setTimeout(r, 800));
  await page.screenshot({ path: path.join(outDir, '04_detail_movie.png') });

  // 5. Back to Home, then Series
  console.log('👉 5. Navigating to Seriály...');
  await page.evaluate(() => {
    const items = Array.from(document.querySelectorAll('li'));
    const seriesLi = items.find(el => el.textContent?.trim() === 'Seriály');
    seriesLi?.click();
  });
  await new Promise(r => setTimeout(r, 600));
  // Click first series
  await page.evaluate(() => {
    const card = document.querySelector('.group.relative.cursor-pointer');
    if (card) card.click();
  });
  await new Promise(r => setTimeout(r, 800));
  await page.screenshot({ path: path.join(outDir, '05_series_with_episodes.png') });

  // 6. Test Spotlight Search (Cmd+K)
  console.log('👉 6. Testing Spotlight Search (Cmd+K)...');
  await page.keyboard.down('Meta');
  await page.keyboard.press('KeyK');
  await page.keyboard.up('Meta');
  await new Promise(r => setTimeout(r, 500));
  await page.keyboard.type('Blade', { delay: 50 });
  await new Promise(r => setTimeout(r, 500));
  await page.screenshot({ path: path.join(outDir, '06_spotlight_search.png') });

  // 7. Hit Escape to close search
  console.log('👉 7. Closing search with Escape...');
  await page.keyboard.press('Escape');
  await new Promise(r => setTimeout(r, 400));

  // 8. Test Wishlist page
  console.log('👉 8. Navigating to Wishlist...');
  await page.evaluate(() => {
    const items = Array.from(document.querySelectorAll('li'));
    const wishLi = items.find(el => el.textContent?.trim() === 'Wishlist');
    wishLi?.click();
  });
  await new Promise(r => setTimeout(r, 600));
  await page.screenshot({ path: path.join(outDir, '07_wishlist.png') });

  // 9. Open Settings dropdown & Scan modal
  console.log('👉 9. Opening Scan Progress Modal...');
  await page.evaluate(() => {
    const cog = document.querySelector('.settings-dropdown button');
    if (cog) cog.click();
  });
  await new Promise(r => setTimeout(r, 400));
  await page.evaluate(() => {
    const buttons = Array.from(document.querySelectorAll('a'));
    const scanBtn = buttons.find(b => b.textContent?.includes('Vyhledat nové filmy'));
    scanBtn?.click();
  });
  await new Promise(r => setTimeout(r, 1200));
  await page.screenshot({ path: path.join(outDir, '08_scan_modal.png') });

  // Close scan modal
  await page.evaluate(() => {
    const closeBtn = document.querySelector('.scan-modal-close') || Array.from(document.querySelectorAll('button')).find(b => b.textContent?.includes('Zavřít') || b.textContent?.includes('Hotovo'));
    closeBtn?.click();
  });
  await new Promise(r => setTimeout(r, 500));

  // 10. Test Settings View - Přehrávání & Profil
  console.log('👉 10. Opening Settings View...');
  await page.evaluate(() => {
    const cog = document.querySelector('.settings-dropdown button');
    if (cog) cog.click();
  });
  await new Promise(r => setTimeout(r, 400));
  await page.evaluate(() => {
    const links = Array.from(document.querySelectorAll('a, button'));
    const settLink = links.find(l => l.textContent?.includes('Pokročilé nastavení'));
    settLink?.click();
  });
  await new Promise(r => setTimeout(r, 800));
  // Scroll down to Přehrávání section
  await page.evaluate(() => {
    window.scrollBy({ top: 500, behavior: 'instant' });
  });
  await new Promise(r => setTimeout(r, 400));
  await page.screenshot({ path: path.join(outDir, '09_settings_playback_profile.png') });

  // 11. Navigate to Home and open Video Player
  console.log('👉 11. Navigating to Home and launching Video Player...');
  await page.evaluate(() => {
    const items = Array.from(document.querySelectorAll('li'));
    const homeLi = items.find(el => el.textContent?.trim() === 'Domů');
    homeLi?.click();
  });
  await new Promise(r => setTimeout(r, 800));

  // Click "Přehrát" on hero banner
  await page.evaluate(() => {
    const playButtons = Array.from(document.querySelectorAll('button'));
    const heroPlay = playButtons.find(b => b.textContent?.includes('Přehrát'));
    if (heroPlay) heroPlay.click();
  });
  await new Promise(r => setTimeout(r, 1000));
  await page.screenshot({ path: path.join(outDir, '10_player_main.png') });

  // 12. Open Chapters Menu in Player
  console.log('👉 12. Opening Chapters menu in Video Player...');
  await page.evaluate(() => {
    const btns = Array.from(document.querySelectorAll('#player-container button'));
    const chapBtn = btns.find(b => b.textContent?.includes('Kapitoly'));
    chapBtn?.click();
  });
  await new Promise(r => setTimeout(r, 500));
  await page.screenshot({ path: path.join(outDir, '11_player_chapters_menu.png') });

  // 13. Open Subtitles Menu in Player
  console.log('👉 13. Opening Subtitles menu in Video Player...');
  await page.evaluate(() => {
    const btns = Array.from(document.querySelectorAll('#player-container button'));
    const subBtn = btns.find(b => b.textContent?.includes('CC'));
    subBtn?.click();
  });
  await new Promise(r => setTimeout(r, 500));
  await page.screenshot({ path: path.join(outDir, '12_player_subtitles_menu.png') });

  // 14. Open Audio Menu in Player
  console.log('👉 14. Opening Audio menu in Video Player...');
  await page.evaluate(() => {
    const btns = Array.from(document.querySelectorAll('#player-container button'));
    // Find audio track button (CZ or EN)
    const audioBtn = btns.find(b => b.getAttribute('title')?.includes('zvukovou stopu') || b.textContent?.includes('CZ') || b.textContent?.includes('EN'));
    audioBtn?.click();
  });
  await new Promise(r => setTimeout(r, 500));
  await page.screenshot({ path: path.join(outDir, '13_player_audio_menu.png') });

  // 15. Click MPV button in player
  console.log('👉 15. Testing MPV button in player...');
  await page.evaluate(() => {
    const btns = Array.from(document.querySelectorAll('#player-container button'));
    const mpvBtn = btns.find(b => b.textContent?.includes('MPV'));
    mpvBtn?.click();
  });
  await new Promise(r => setTimeout(r, 800));
  await page.screenshot({ path: path.join(outDir, '14_player_mpv_active.png') });

  await browser.close();
  console.log('🎉 Live interactive tests finished! All screenshots saved in', outDir);
}

run().catch(console.error);
