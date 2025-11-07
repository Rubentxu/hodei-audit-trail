# Hodei Audit Web

CloudTrail-Inspired Audit Dashboard built with Next.js 14+, TypeScript, and TailwindCSS.

## 🚀 Getting Started

First, install dependencies:

```bash
npm install
```

Run the development server:

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## 📁 Project Structure

```
src/
├── app/                    # Next.js 14 App Router
│   ├── layout.tsx         # Root layout
│   ├── page.tsx           # Home page
│   └── globals.css        # Global styles
├── components/            # Reusable components
│   ├── ui/               # shadcn/ui components
│   └── layout/           # Layout components
└── lib/                  # Utilities and configurations
    ├── utils.ts          # Helper functions
    └── design-tokens/    # Design system tokens
```

## 🛠️ Tech Stack

- **Next.js 14+** - React framework with App Router
- **TypeScript 5+** - Type-safe development
- **TailwindCSS 3+** - Utility-first CSS
- **shadcn/ui** - High-quality UI components
- **next-themes** - Dark mode support
- **Lucide React** - Beautiful icons

## 📝 Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run start` - Start production server
- `npm run lint` - Run ESLint
- `npm run lint:fix` - Fix ESLint errors
- `npm run type-check` - TypeScript type checking
- `npm run format` - Format code with Prettier
- `npm run format:check` - Check code formatting

## 🎨 Design System

The project uses a custom design system with:

- **Colors**: Primary, semantic (success, warning, error, info), and neutral scales
- **Typography**: Inter font with defined sizes and weights
- **Spacing**: Consistent spacing scale
- **Border Radius**: Multiple radius options

## 📦 Dependencies

### Core
- Next.js 14
- React 18
- TypeScript 5

### UI
- TailwindCSS 3
- shadcn/ui components
- Lucide React icons
- next-themes

### State & Data
- @tanstack/react-query
- Zustand
- Zod validation

## 🧪 Testing

The project is set up for:
- Unit testing with Jest
- E2E testing with Playwright
- Type checking with TypeScript
- Linting with ESLint
- Formatting with Prettier

## 📄 License

ISC

## 👥 Contributing

Contributions are welcome! Please follow the established code style and conventions.
