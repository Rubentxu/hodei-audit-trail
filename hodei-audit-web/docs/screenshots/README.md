# Screenshots for User Guide

This directory contains screenshots of the Hodei Audit Trail application, generated automatically using Playwright.

## Page Screenshots

| Filename | Description | Size | Related Documentation |
|----------|-------------|------|----------------------|
| 01-home.png | Home Page - Landing | 33.61 KB | [User Guide](../USER-GUIDE.md#home) |
| 02-login.png | Login Page | 34.15 KB | [User Guide](../USER-GUIDE.md#login) |
| 03-tenant-select.png | Tenant Selection | 34.15 KB | [User Guide](../USER-GUIDE.md#tenant-select) |
| 04-events.png | Events Management | 34.15 KB | [User Guide](../USER-GUIDE.md#events) |
| 05-analytics.png | Analytics Dashboard | 72.75 KB | [User Guide](../USER-GUIDE.md#analytics) |
| 06-compliance.png | Compliance Dashboard | 34.15 KB | [User Guide](../USER-GUIDE.md#compliance) |

## Component Screenshots

| Filename | Description |
|----------|-------------|
| components/header-nav.png | Header Navigation |
| components/footer.png | Footer |

## Regenerating Screenshots

To regenerate these screenshots:

```bash
# Option 1: Using the script directly
node scripts/generate-screenshots.js

# Option 2: Using Playwright test
npx playwright test tests-e2e/screenshot.spec.ts --project=chromium
```

## Technical Details

- **Resolution**: 1920x1080
- **Browser**: Chromium (Headless)
- **Full Page**: Yes
- **Animations**: Disabled
- **Generated**: 2025-11-07T21:19:06.212Z

## Usage in Documentation

To reference a screenshot in Markdown:

```markdown
![Description](filename.png)
```

Or with a full path:

```markdown
![Description](../screenshots/filename.png)
```

---
Generated automatically by scripts/generate-screenshots.js
