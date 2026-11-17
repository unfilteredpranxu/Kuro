import { existsSync } from 'node:fs';
import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests/e2e', timeout: 30000, fullyParallel: false,
  use: { baseURL: 'http://127.0.0.1:4174', browserName: 'chromium', launchOptions: { executablePath: process.env.CHROMIUM_PATH || (existsSync('/usr/bin/chromium') ? '/usr/bin/chromium' : undefined), args: ['--no-sandbox'] }, trace: 'retain-on-failure' },
  projects: [{ name: 'chromium', use: { viewport: { width: 1440, height: 900 } } }, { name: 'phone', testMatch: '**/workflows.spec.js', use: { viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true } }],
  webServer: { command: 'python3 -m http.server 4174 --bind 0.0.0.0 --directory target/dx/kuro-web/release/web/public', url: 'http://127.0.0.1:4174', reuseExistingServer: false },
  outputDir: 'test-results', reporter: [['list'], ['html', { open: 'never' }]],
});
