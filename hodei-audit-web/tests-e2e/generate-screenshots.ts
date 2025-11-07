// Screenshot generation script for User Guide
import { test, expect, chromium, Page } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

// Configure screenshot directory
const SCREENSHOT_DIR = path.join(process.cwd(), 'docs', 'screenshots');

// Page definitions for screenshot generation
const PAGES = [
  {
    name: '01-home-page',
    path: '/',
    description: 'Home Page - Landing',
    waitForSelector: 'h1:has-text("Welcome to Hodei Audit")',
    fullPage: true,
  },
  {
    name: '02-login-page',
    path: '/auth/login',
    description: 'Login Page',
    waitForSelector: 'form, h1, h2',
    fullPage: true,
  },
  {
    name: '03-dashboard',
    path: '/dashboard',
    description: 'Main Dashboard',
    waitForSelector: 'main, [data-testid="dashboard"], h1',
    fullPage: true,
  },
  {
    name: '04-events-list',
    path: '/events',
    description: 'Events Management Page',
    waitForSelector: 'main, table, [data-testid="events"], h1',
    fullPage: true,
  },
  {
    name: '05-analytics',
    path: '/analytics',
    description: 'Analytics Dashboard',
    waitForSelector: 'main, [data-testid="analytics"], h1, canvas, svg',
    fullPage: true,
  },
  {
    name: '06-compliance',
    path: '/compliance',
    description: 'Compliance Dashboard',
    waitForSelector: 'main, [data-testid="compliance"], h1',
    fullPage: true,
  },
  {
    name: '07-tenant-select',
    path: '/auth/tenant-select',
    description: 'Tenant Selection Page',
    waitForSelector: 'h1, h2, [data-testid="tenant-selector"], select',
    fullPage: true,
  },
  {
    name: '08-404-page',
    path: '/non-existent-page',
    description: '404 Error Page',
    waitForSelector: 'body',
    fullPage: true,
  },
];

// Component screenshots (UI elements)
const COMPONENTS = [
  {
    name: 'nav-header',
    description: 'Navigation Header',
    selector: 'nav, header, [data-testid="nav"]',
  },
  {
    name: 'sidebar',
    description: 'Sidebar Navigation',
    selector: 'aside, [data-testid="sidebar"]',
  },
  {
    name: 'footer',
    description: 'Footer',
    selector: 'footer, [data-testid="footer"]',
  },
];

/**
 * Take a screenshot of a page
 */
async function takePageScreenshot(
  page: Page,
  pageInfo: typeof PAGES[0]
): Promise<void> {
  try {
    console.log(`\n📸 Capturing: ${pageInfo.name}`);
    console.log(`   Path: ${pageInfo.path}`);
    console.log(`   Description: ${pageInfo.description}`);

    // Navigate to page
    await page.goto(pageInfo.path, {
      waitUntil: 'networkidle',
      timeout: 30000,
    });

    // Wait for page to load
    try {
      await page.waitForSelector(pageInfo.waitForSelector, {
        timeout: 10000,
      });
    } catch (error) {
      console.log(`   ⚠️  Selector not found, but continuing...`);
    }

    // Take screenshot
    const screenshotPath = path.join(SCREENSHOT_DIR, `${pageInfo.name}.png`);
    await page.screenshot({
      path: screenshotPath,
      fullPage: pageInfo.fullPage ?? false,
      animations: 'disabled',
    });

    console.log(`   ✅ Screenshot saved: ${screenshotPath}`);
  } catch (error) {
    console.error(`   ❌ Error capturing ${pageInfo.name}:`, error);
    throw error;
  }
}

/**
 * Take a screenshot of a component
 */
async function takeComponentScreenshot(
  page: Page,
  componentInfo: typeof COMPONENTS[0],
  pageUrl: string
): Promise<void> {
  try {
    console.log(`\n📱 Capturing component: ${componentInfo.name}`);

    // Navigate to page first
    await page.goto(pageUrl, {
      waitUntil: 'networkidle',
      timeout: 30000,
    });

    // Try to find and screenshot the component
    const element = page.locator(componentInfo.selector).first();
    const count = await element.count();

    if (count > 0) {
      const screenshotPath = path.join(
        SCREENSHOT_DIR,
        `components`,
        `${componentInfo.name}.png`
      );
      await element.screenshot({
        path: screenshotPath,
        animations: 'disabled',
      });
      console.log(`   ✅ Component screenshot saved: ${screenshotPath}`);
    } else {
      console.log(`   ⚠️  Component not found on this page`);
    }
  } catch (error) {
    console.error(`   ❌ Error capturing component ${componentInfo.name}:`, error);
  }
}

/**
 * Generate all screenshots
 */
async function generateScreenshots() {
  // Create screenshot directory
  const componentDir = path.join(SCREENSHOT_DIR, 'components');
  fs.mkdirSync(SCREENSHOT_DIR, { recursive: true });
  fs.mkdirSync(componentDir, { recursive: true });

  console.log('\n🎬 Starting Screenshot Generation');
  console.log(`📁 Output directory: ${SCREENSHOT_DIR}`);
  console.log('='.repeat(60));

  // Launch browser
  const browser = await chromium.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
  });

  const context = await browser.newContext({
    viewport: { width: 1920, height: 1080 },
    deviceScaleFactor: 1,
  });

  const page = await context.newPage();

  // Capture page screenshots
  for (const pageInfo of PAGES) {
    await takePageScreenshot(page, pageInfo);
    await page.waitForTimeout(1000); // Wait between pages
  }

  // Capture component screenshots on home page
  console.log('\n' + '='.repeat(60));
  console.log('📱 Capturing Components');
  console.log('='.repeat(60));

  for (const component of COMPONENTS) {
    await takeComponentScreenshot(page, component, '/');
  }

  // Close browser
  await browser.close();

  // Create index file
  createIndexFile();

  console.log('\n' + '='.repeat(60));
  console.log('✅ Screenshot Generation Complete!');
  console.log('='.repeat(60));
  console.log(`📁 All screenshots saved to: ${SCREENSHOT_DIR}`);
  console.log(`📄 Index file created: ${path.join(SCREENSHOT_DIR, 'README.md')}`);
}

/**
 * Create an index README file
 */
function createIndexFile(): void {
  const readmePath = path.join(SCREENSHOT_DIR, 'README.md');
  const readmeContent = `# Screenshots for User Guide

This directory contains screenshots of the Hodei Audit Trail application.

## Pages

| Screenshot | Description | Section in User Guide |
|------------|-------------|----------------------|
| 01-home-page.png | Home page with welcome message | Getting Started |
| 02-login-page.png | Login form | Authentication |
| 03-dashboard.png | Main dashboard | Dashboard |
| 04-events-list.png | Events management page | Event Management |
| 05-analytics.png | Analytics dashboard | Analytics |
| 06-compliance.png | Compliance dashboard | Compliance |
| 07-tenant-select.png | Tenant selection page | Authentication |
| 08-404-page.png | 404 error page | Troubleshooting |

## Components

| Screenshot | Description |
|------------|-------------|
| components/nav-header.png | Top navigation bar |
| components/sidebar.png | Left sidebar navigation |
| components/footer.png | Footer |

## Generating Screenshots

To regenerate these screenshots:

\`\`\`bash
cd hodei-audit-web
npx playwright test generate-screenshots.ts --project=chromium
\`\`\`

Or directly:

\`\`\`bash
npx ts-node tests-e2e/generate-screenshots.ts
\`\`\`

## Notes

- Screenshots are taken in 1920x1080 resolution
- Full page screenshots are taken where appropriate
- Animations are disabled during capture
- Each page is given time to fully load before capture

---
Generated on: ${new Date().toISOString()}
`;

  fs.writeFileSync(readmePath, readmeContent);
}

// Run if called directly
if (require.main === module) {
  generateScreenshots()
    .then(() => {
      console.log('\n🎉 Done!');
      process.exit(0);
    })
    .catch((error) => {
      console.error('\n❌ Failed to generate screenshots:', error);
      process.exit(1);
    });
}

export { generateScreenshots };
