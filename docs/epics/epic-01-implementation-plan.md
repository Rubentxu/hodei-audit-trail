# Epic 01: Project Foundation & Setup - Implementation Plan

## 📊 Overview

**Epic ID:** EPIC-01  
**Priority:** P0 (Critical)  
**Total Story Points:** 21  
**Estimated Duration:** 2 sprints (4 weeks)  
**Team Capacity:** 2-3 developers  
**Sprint Velocity Target:** 10-12 points per sprint

---

## 🎯 Goals

1. Establish a modern, type-safe development foundation
2. Implement consistent design system and UI component library
3. Configure development tools and CI/CD pipeline
4. Create reusable layout components
5. Ensure accessibility and responsive design from the start

---

## 📦 Dependencies

### Prerequisites
- Node.js 18+ installed
- npm/pnpm/yarn package manager
- Git repository initialized
- VS Code or compatible IDE

### Internal Dependencies
- Story 01.01 (Next.js + TypeScript) must complete before 01.02
- Story 01.02 (TailwindCSS) must complete before 01.03
- Story 01.03 (shadcn/ui) must complete before 01.05
- Story 01.04 (Dependencies) can run parallel with 01.02-01.03

---

## 🚀 Implementation Strategy

### Phase 1: Core Setup (Sprint 1 Week 1)
**Focus:** Establish the foundation
- Story 01.01: Initialize Next.js Project with TypeScript
- Story 01.02: Configure TailwindCSS
- Story 01.04: Set Up Project Dependencies (partial)

### Phase 2: UI System (Sprint 1 Week 2)
**Focus:** Build the design system
- Story 01.03: Install and Configure shadcn/ui
- Story 01.06: Implement Design System Tokens
- Story 01.04: Set Up Project Dependencies (complete)

### Phase 3: Layout & Tools (Sprint 2 Week 1)
**Focus:** Create reusable components
- Story 01.05: Create Base Layout Components
- Story 01.07: Set Up Development Tools

### Phase 4: Polish & Testing (Sprint 2 Week 2)
**Focus:** Refinement and validation
- End-to-end testing
- Performance optimization
- Documentation
- Security review

---

## 📋 Detailed Implementation Plan

### Story 01.01: Initialize Next.js Project with TypeScript
**Story Points:** 3  
**Priority:** P0 (Critical)  
**Estimated Duration:** 1-2 days

#### Technical Approach
```bash
# Create Next.js app with TypeScript
npx create-next-app@latest hodei-audit-web --typescript --tailwind --eslint --app --src-dir --import-alias "@/*" --use-npm
```

#### Implementation Steps
1. **Initialize Project**
   - Run create-next-app with TypeScript template
   - Configure App Router structure
   - Set up src directory architecture

2. **Configure TypeScript**
   - Enable strict mode in tsconfig.json
   - Configure path aliases (@/* for src/*)
   - Set up type checking options

3. **Set Up Project Structure**
   ```
   src/
   ├── app/                 # Next.js 14 App Router
   │   ├── (auth)/         # Auth route group
   │   ├── dashboard/      # Dashboard routes
   │   ├── events/         # Event history routes
   │   ├── analytics/      # Analytics routes
   │   ├── compliance/     # Compliance routes
   │   ├── layout.tsx      # Root layout
   │   ├── page.tsx        # Home page
   │   └── globals.css     # Global styles
   ├── components/         # Reusable components
   │   ├── ui/            # shadcn/ui components
   │   ├── forms/         # Form components
   │   ├── layout/        # Layout components
   │   └── charts/        # Chart components
   ├── lib/               # Utilities and configurations
   │   ├── utils.ts       # Helper functions
   │   ├── validations/   # Zod schemas
   │   ├── constants.ts   # App constants
   │   └── types/         # TypeScript types
   ├── hooks/             # Custom React hooks
   ├── stores/            # Zustand stores
   └── styles/            # Additional styles
   ```

4. **Configure ESLint and Prettier**
   - Set up .eslintrc.json with Next.js recommended config
   - Configure .prettierrc for consistent formatting
   - Set up .vscode/settings.json for VS Code

5. **Initialize Git Repository**
   - Create .gitignore (Node.js, Next.js, OS specific)
   - Initial commit with project structure

#### TDD Approach
**Unit Tests:**
```typescript
// tests/lib/utils.test.ts
import { describe, it, expect } from '@jest/core';
import { cn } from '@/lib/utils';

describe('utils', () => {
  it('should combine class names correctly', () => {
    expect(cn('class1', 'class2')).toBe('class1 class2');
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/homepage.test.ts
import { test, expect } from '@playwright/test';

test('homepage loads successfully', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/Hodei Audit/);
  await expect(page.getByRole('heading', { name: /Welcome/ })).toBeVisible();
});
```

#### Definition of Done
- [ ] Next.js 14+ with App Router is installed
- [ ] TypeScript compiles without errors
- [ ] ESLint passes without warnings
- [ ] Prettier formats code correctly
- [ ] Project structure matches specification
- [ ] All unit tests pass (≥ 90% coverage)
- [ ] E2E tests pass
- [ ] Git repository is initialized

---

### Story 01.02: Configure TailwindCSS
**Story Points:** 2  
**Priority:** P0 (Critical)  
**Estimated Duration:** 1 day

#### Technical Approach
TailwindCSS comes pre-configured with create-next-app, but we need to customize it for our design system.

#### Implementation Steps
1. **Verify TailwindCSS Installation**
   - Check tailwind.config.js exists
   - Verify postcss.config.js is set up
   - Ensure globals.css has Tailwind directives

2. **Configure Design Tokens**
   ```javascript
   // tailwind.config.js
   module.exports = {
     darkMode: ['class'],
     content: ['./src/**/*.{ts,tsx}'],
     theme: {
       extend: {
         colors: {
           border: 'hsl(var(--border))',
           input: 'hsl(var(--input))',
           ring: 'hsl(var(--ring))',
           background: 'hsl(var(--background))',
           foreground: 'hsl(var(--foreground))',
           primary: {
             DEFAULT: 'hsl(var(--primary))',
             foreground: 'hsl(var(--primary-foreground))',
           },
           // ... more colors
         },
       },
     },
   };
   ```

3. **Set Up CSS Variables**
   ```css
   /* src/app/globals.css */
   :root {
     --background: 0 0% 100%;
     --foreground: 222.2 84% 4.9%;
     --primary: 221.2 83.2% 53.3%;
     /* ... more variables */
   }

   .dark {
     --background: 222.2 84% 4.9%;
     --foreground: 210 40% 98%;
     /* ... dark mode variables */
   }
   ```

4. **Configure Responsive Breakpoints**
   - Mobile: 640px (sm)
   - Tablet: 768px (md)
   - Desktop: 1024px (lg)
   - Wide: 1280px (xl)
   - Ultra-wide: 1536px (2xl)

5. **Test Responsive Design**
   - Create test components to verify breakpoints
   - Test dark mode switching

#### TDD Approach
**Unit Tests:**
```typescript
// tests/lib/design-tokens.test.ts
import { designTokens } from '@/lib/design-tokens';

describe('design tokens', () => {
  it('should have correct color values', () => {
    expect(designTokens.colors.primary).toBeDefined();
    expect(designTokens.colors.secondary).toBeDefined();
  });

  it('should have correct spacing scale', () => {
    expect(designTokens.spacing.xs).toBe('0.25rem');
    expect(designTokens.spacing.sm).toBe('0.5rem');
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/tailwind.test.ts
import { test, expect } from '@playwright/test';

test('tailwind classes are applied correctly', async ({ page }) => {
  await page.goto('/');
  const element = page.getByTestId('test-element');
  await expect(element).toHaveClass(/bg-primary/);
  await expect(element).toHaveClass(/text-foreground/);
});
```

#### Definition of Done
- [ ] TailwindCSS 3+ is installed
- [ ] PostCSS is configured
- [ ] Custom design tokens are defined
- [ ] Global CSS file is configured
- [ ] Responsive breakpoints work
- [ ] Dark mode support is configured
- [ ] All tests pass

---

### Story 01.03: Install and Configure shadcn/ui
**Story Points:** 3  
**Priority:** P0 (Critical)  
**Estimated Duration:** 2 days

#### Technical Approach
shadcn/ui provides a collection of high-quality, accessible UI components built on Radix UI and TailwindCSS.

#### Implementation Steps
1. **Initialize shadcn/ui**
   ```bash
   npx shadcn-ui@latest init
   ```

2. **Install Base Components**
   ```bash
   npx shadcn-ui@latest add button
   npx shadcn-ui@latest add input
   npx shadcn-ui@latest add card
   npx shadcn-ui@latest add tabs
   npx shadcn-ui@latest add table
   npx shadcn-ui@latest add select
   npx shadcn-ui@latest add dropdown-menu
   npx shadcn-ui@latest add dialog
   npx shadcn-ui@latest add toast
   npx shadcn-ui@latest add badge
   npx shadcn-ui@latest add avatar
   npx shadcn-ui@latest add separator
   npx shadcn-ui@latest add scroll-area
   npx shadcn-ui@latest add skeleton
   npx shadcn-ui@latest add textarea
   npx shadcn-ui@latest add checkbox
   npx shadcn-ui@latest add radio-group
   npx shadcn-ui@latest add switch
   npx shadcn-ui@latest add tooltip
   npx shadcn-ui@latest add popover
   npx shadcn-ui@latest add calendar
   npx shadcn-ui@latest add command
   ```

3. **Configure Theme**
   - Update component styles to match design tokens
   - Ensure color consistency
   - Test all component variants

4. **Create Component Wrapper Pattern**
   ```typescript
   // src/components/ui/button.tsx
   import * as React from 'react';
   import { Slot } from '@radix-ui/react-slot';
   import { cva, type VariantProps } from 'class-variance-authority';
   import { cn } from '@/lib/utils';

   const buttonVariants = cva(
     'inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50',
     {
       variants: {
         variant: {
           default: 'bg-primary text-primary-foreground shadow hover:bg-primary/90',
           destructive: 'bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90',
           outline: 'border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground',
           secondary: 'bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80',
           ghost: 'hover:bg-accent hover:text-accent-foreground',
           link: 'text-primary underline-offset-4 hover:underline',
         },
         size: {
           default: 'h-9 px-4 py-2',
           sm: 'h-8 rounded-md px-3 text-xs',
           lg: 'h-10 rounded-md px-8',
           icon: 'h-9 w-9',
         },
       },
       defaultVariants: {
         variant: 'default',
         size: 'default',
       },
     }
   );

   export interface ButtonProps
     extends React.ButtonHTMLAttributes<HTMLButtonElement>,
       VariantProps<typeof buttonVariants> {
     asChild?: boolean;
   }

   const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
     ({ className, variant, size, asChild = false, ...props }, ref) => {
       const Comp = asChild ? Slot : 'button';
       return (
         <Comp
           className={cn(buttonVariants({ variant, size, className }))}
           ref={ref}
           {...props}
         />
       );
     }
   );
   Button.displayName = 'Button';

   export { Button, buttonVariants };
   ```

5. **Test Accessibility**
   - Verify ARIA attributes
   - Test keyboard navigation
   - Check color contrast
   - Test with screen readers

#### TDD Approach
**Unit Tests:**
```typescript
// tests/components/ui/button.test.tsx
import { render, screen } from '@testing-library/react';
import { Button, buttonVariants } from '@/components/ui/button';

describe('Button Component', () => {
  it('renders with default variant', () => {
    render(<Button>Click me</Button>);
    const button = screen.getByRole('button', { name: /click me/i });
    expect(button).toBeInTheDocument();
    expect(button).toHaveClass(buttonVariants({ variant: 'default' }));
  });

  it('renders with custom variant', () => {
    render(<Button variant="destructive">Delete</Button>);
    const button = screen.getByRole('button', { name: /delete/i });
    expect(button).toHaveClass(buttonVariants({ variant: 'destructive' }));
  });

  it('is accessible', () => {
    render(<Button aria-label="Close">×</Button>);
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('aria-label', 'Close');
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/components.test.ts
import { test, expect } from '@playwright/test';

test('button interactions work correctly', async ({ page }) => {
  await page.goto('/test-components');
  await page.click('button:has-text("Click me")');
  await expect(page.locator('[data-testid="clicked"]')).toBeVisible();
});
```

#### Definition of Done
- [ ] shadcn/ui is installed
- [ ] All base components are added
- [ ] Component configuration is set up
- [ ] Custom theme colors are configured
- [ ] All components are accessible (WCAG 2.1 AA)
- [ ] Component variants are defined
- [ ] All tests pass

---

### Story 01.04: Set Up Project Dependencies
**Story Points:** 3  
**Priority:** P0 (Critical)  
**Estimated Duration:** 1-2 days

#### Technical Approach
Install all required dependencies for the application to function properly.

#### Implementation Steps
1. **Install Core Dependencies**
   ```bash
   # State Management
   npm install @tanstack/react-query zustand

   # Validation
   npm install zod

   # Data Visualization
   npm install recharts

   # Date Manipulation
   npm install date-fns

   # Authentication (for future use)
   npm install next-auth

   # HTTP Client
   npm install axios

   # Icons
   npm install lucide-react

   # Utilities
   npm install clsx class-variance-authority tailwind-merge
   ```

2. **Install Type Definitions**
   ```bash
   npm install -D @types/node
   ```

3. **Verify package.json Configuration**
   ```json
   {
     "name": "hodei-audit-web",
     "version": "0.1.0",
     "private": true,
     "scripts": {
       "dev": "next dev",
       "build": "next build",
       "start": "next start",
       "lint": "next lint",
       "test": "jest",
       "test:watch": "jest --watch",
       "test:e2e": "playwright test"
     },
     "dependencies": {
       "next": "14.0.0",
       "react": "18.2.0",
       "react-dom": "18.2.0",
       "@tanstack/react-query": "^5.0.0",
       "zustand": "^4.4.0",
       "zod": "^3.22.0",
       "recharts": "^2.8.0",
       "date-fns": "^2.30.0",
       "next-auth": "^4.24.0",
       "axios": "^1.6.0",
       "lucide-react": "^0.292.0",
       "clsx": "^2.0.0",
       "class-variance-authority": "^0.7.0",
       "tailwind-merge": "^2.0.0"
     },
     "devDependencies": {
       "typescript": "^5.3.0",
       "@types/node": "^20.10.0",
       "@types/react": "^18.2.0",
       "@types/react-dom": "^18.2.0",
       "autoprefixer": "^10.4.0",
       "postcss": "^8.4.0",
       "tailwindcss": "^3.3.0",
       "eslint": "^8.56.0",
       "eslint-config-next": "14.0.0",
       "prettier": "^3.1.0",
       "@testing-library/react": "^13.4.0",
       "@testing-library/jest-dom": "^6.1.0",
       "jest": "^29.7.0",
       "jest-environment-jsdom": "^29.7.0",
       "@playwright/test": "^1.40.0"
     }
   }
   ```

4. **Update tsconfig.json for path mapping**
   ```json
   {
     "compilerOptions": {
       "target": "es5",
       "lib": ["dom", "dom.iterable", "esnext"],
       "allowJs": true,
       "skipLibCheck": true,
       "strict": true,
       "noEmit": true,
       "esModuleInterop": true,
       "module": "esnext",
       "moduleResolution": "bundler",
       "resolveJsonModule": true,
       "isolatedModules": true,
       "jsx": "preserve",
       "incremental": true,
       "plugins": [{ "name": "next" }],
       "baseUrl": ".",
       "paths": {
         "@/*": ["./src/*"]
       }
     },
     "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
     "exclude": ["node_modules"]
   }
   ```

5. **Create lib/utils.ts**
   ```typescript
   // src/lib/utils.ts
   import { type ClassValue, clsx } from 'clsx';
   import { twMerge } from 'tailwind-merge';

   export function cn(...inputs: ClassValue[]) {
     return twMerge(clsx(inputs));
   }
   ```

6. **Verify All Imports Work**
   - Test each dependency can be imported
   - Verify no TypeScript errors

#### TDD Approach
**Unit Tests:**
```typescript
// tests/lib/dependencies.test.ts
import * as ReactQuery from '@tanstack/react-query';
import { create } from 'zustand';
import { z } from 'zod';
import * as Recharts from 'recharts';
import { format } from 'date-fns';

describe('dependencies', () => {
  it('can import all dependencies', () => {
    expect(ReactQuery).toBeDefined();
    expect(create).toBeDefined();
    expect(z).toBeDefined();
    expect(Recharts).toBeDefined();
    expect(format).toBeDefined();
  });

  it('can use zod for validation', () => {
    const schema = z.object({ name: z.string() });
    const result = schema.parse({ name: 'test' });
    expect(result.name).toBe('test');
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/build.test.ts
import { test, expect } from '@playwright/test';

test('application builds successfully', async ({ page }) => {
  // If build succeeds, this test will pass
  // If there are dependency issues, build will fail
  expect(true).toBe(true);
});
```

#### Definition of Done
- [ ] All required dependencies are installed
- [ ] package.json is properly configured
- [ ] tsconfig.json has correct path mappings
- [ ] utils.ts is created
- [ ] All imports work correctly
- [ ] Application builds successfully
- [ ] All tests pass

---

### Story 01.05: Create Base Layout Components
**Story Points:** 5  
**Priority:** P0 (Critical)  
**Estimated Duration:** 3-4 days

#### Technical Approach
Create a consistent, accessible layout that works across all devices and screen sizes.

#### Implementation Steps

1. **Create Root Layout**
   ```typescript
   // src/app/layout.tsx
   import type { Metadata } from 'next';
   import { Inter } from 'next/font/google';
   import './globals.css';
   import { Providers } from '@/components/providers';
   import { Toaster } from '@/components/ui/toaster';

   const inter = Inter({ subsets: ['latin'] });

   export const metadata: Metadata = {
     title: 'Hodei Audit - CloudTrail-Inspired Audit Dashboard',
     description: 'Comprehensive audit trail dashboard with real-time analytics',
   };

   export default function RootLayout({
     children,
   }: {
     children: React.ReactNode;
   }) {
     return (
       <html lang="en" suppressHydrationWarning>
         <body className={inter.className}>
           <Providers>
             {children}
             <Toaster />
           </Providers>
         </body>
       </html>
     );
   }
   ```

2. **Create Providers Component**
   ```typescript
   // src/components/providers.tsx
   'use client';

   import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
   import { ThemeProvider } from 'next-themes';
   import { useState } from 'react';

   export function Providers({ children }: { children: React.ReactNode }) {
     const [queryClient] = useState(() => new QueryClient());

     return (
       <QueryClientProvider client={queryClient}>
         <ThemeProvider
           attribute="class"
           defaultTheme="system"
           enableSystem
           disableTransitionOnChange
         >
           {children}
         </ThemeProvider>
       </QueryClientProvider>
     );
   }
   ```

3. **Create Sidebar Navigation**
   ```typescript
   // src/components/layout/sidebar.tsx
   'use client';

   import Link from 'next/link';
   { usePathname } from 'next/navigation';
   import { cn } from '@/lib/utils';
   import { Button } from '@/components/ui/button';
   import {
     LayoutDashboard,
     History,
     BarChart3,
     ShieldCheck,
     Settings,
     Users,
   } from 'lucide-react';

   const navigation = [
     { name: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
     { name: 'Event History', href: '/events', icon: History },
     { name: 'Analytics', href: '/analytics', icon: BarChart3 },
     { name: 'Compliance', href: '/compliance', icon: ShieldCheck },
     { name: 'Users', href: '/users', icon: Users },
     { name: 'Settings', href: '/settings', icon: Settings },
   ];

   export function Sidebar() {
     const pathname = usePathname();

     return (
       <div className="flex h-full w-64 flex-col bg-card border-r">
         <div className="flex h-16 items-center border-b px-6">
           <h1 className="text-xl font-bold">Hodei Audit</h1>
         </div>
         <nav className="flex-1 space-y-1 p-4">
           {navigation.map((item) => (
             <Link key={item.name} href={item.href}>
               <Button
                 variant={pathname === item.href ? 'secondary' : 'ghost'}
                 className={cn(
                   'w-full justify-start',
                   pathname === item.href && 'bg-secondary'
                 )}
               >
                 <item.icon className="mr-2 h-4 w-4" />
                 {item.name}
               </Button>
             </Link>
           ))}
         </nav>
       </div>
     );
   }
   ```

4. **Create Header Component**
   ```typescript
   // src/components/layout/header.tsx
   'use client';

   import { useTheme } from 'next-themes';
   import { Button } from '@/components/ui/button';
   import {
     DropdownMenu,
     DropdownMenuContent,
     DropdownMenuItem,
     DropdownMenuTrigger,
   } from '@/components/ui/dropdown-menu';
   import { Sun, Moon, User, LogOut } from 'lucide-react';

   export function Header() {
     const { setTheme, theme } = useTheme();

     return (
       <header className="flex h-16 items-center justify-between border-b bg-card px-6">
         <div className="flex items-center space-x-4">
           {/* Tenant Selector - Will be implemented in Epic 2 */}
           <DropdownMenu>
             <DropdownMenuTrigger asChild>
               <Button variant="outline">Select Tenant</Button>
             </DropdownMenuTrigger>
             <DropdownMenuContent>
               <DropdownMenuItem>Tenant A</DropdownMenuItem>
               <DropdownMenuItem>Tenant B</DropdownMenuItem>
             </DropdownMenuContent>
           </DropdownMenu>
         </div>

         <div className="flex items-center space-x-2">
           <Button
             variant="ghost"
             size="icon"
             onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
           >
             <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
             <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
             <span className="sr-only">Toggle theme</span>
           </Button>

           <DropdownMenu>
             <DropdownMenuTrigger asChild>
               <Button variant="ghost" size="icon">
                 <User className="h-4 w-4" />
               </Button>
             </DropdownMenuTrigger>
             <DropdownMenuContent align="end">
               <DropdownMenuItem>Profile</DropdownMenuItem>
               <DropdownMenuItem>Settings</DropdownMenuItem>
               <DropdownMenuItem>
                 <LogOut className="mr-2 h-4 w-4" />
                 Logout
               </DropdownMenuItem>
             </DropdownMenuContent>
           </DropdownMenu>
         </div>
       </header>
     );
   }
   ```

5. **Create Main Layout Wrapper**
   ```typescript
   // src/components/layout/main-layout.tsx
   'use client';

   import { Sidebar } from './sidebar';
   import { Header } from './header';
   import { cn } from '@/lib/utils';

   export function MainLayout({ children }: { children: React.ReactNode }) {
     return (
       <div className="flex h-screen bg-background">
         <Sidebar />
         <div className="flex flex-1 flex-col overflow-hidden">
           <Header />
           <main className="flex-1 overflow-y-auto p-6">
             {children}
           </main>
         </div>
       </div>
     );
   }
   ```

6. **Create Mobile Navigation**
   ```typescript
   // src/components/layout/mobile-nav.tsx
   'use client';

   import { useState } from 'react';
   import { Button } from '@/components/ui/button';
   import { Sheet, SheetContent, SheetTrigger } from '@/components/ui/sheet';
   import { Menu } from 'lucide-react';
   import { Sidebar } from './sidebar';

   export function MobileNav() {
     const [open, setOpen] = useState(false);

     return (
       <Sheet open={open} onOpenChange={setOpen}>
         <SheetTrigger asChild>
           <Button variant="ghost" size="icon" className="md:hidden">
             <Menu className="h-5 w-5" />
           </Button>
         </SheetTrigger>
         <SheetContent side="left" className="p-0">
           <Sidebar />
         </SheetContent>
       </Sheet>
     );
   }
   ```

7. **Update Root Layout with Responsive Design**
   - Hide sidebar on mobile by default
   - Show mobile nav button
   - Ensure content is scrollable

8. **Add Skip Link for Accessibility**
   ```typescript
   // Add to layout.tsx
   <a
     href="#main-content"
     className="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 z-50 bg-primary px-4 py-2 text-primary-foreground"
   >
     Skip to main content
   </a>
   ```

9. **Create Responsive Styles**
   - Mobile: Sidebar hidden, hamburger menu
   - Tablet: Collapsible sidebar
   - Desktop: Persistent sidebar

#### TDD Approach
**Unit Tests:**
```typescript
// tests/components/layout/sidebar.test.tsx
import { render, screen } from '@testing-library/react';
import { Sidebar } from '@/components/layout/sidebar';
import { usePathname } from 'next/navigation';

jest.mock('next/navigation', () => ({
  usePathname: jest.fn(),
}));

describe('Sidebar', () => {
  it('renders navigation items', () => {
    (usePathname as jest.Mock).mockReturnValue('/dashboard');
    render(<Sidebar />);
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Event History')).toBeInTheDocument();
  });

  it('highlights active route', () => {
    (usePathname as jest.Mock).mockReturnValue('/dashboard');
    render(<Sidebar />);
    const dashboardButton = screen.getByText('Dashboard').closest('button');
    expect(dashboardButton).toHaveClass('bg-secondary');
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/layout.test.ts
import { test, expect } from '@playwright/test';

test('layout is responsive', async ({ page }) => {
  await page.goto('/dashboard');

  // Desktop view
  await page.setViewportSize({ width: 1920, height: 1080 });
  await expect(page.locator('[data-testid="sidebar"]')).toBeVisible();

  // Mobile view
  await page.setViewportSize({ width: 375, height: 667 });
  await expect(page.locator('[data-testid="mobile-nav-button"]')).toBeVisible();
});

test('navigation works correctly', async ({ page }) => {
  await page.goto('/dashboard');
  await page.click('text=Event History');
  await expect(page).toHaveURL('/events');
});
```

#### Definition of Done
- [ ] Root layout is created
- [ ] Sidebar navigation is implemented
- [ ] Header component is created
- [ ] Layout is responsive (mobile, tablet, desktop)
- [ ] Dark mode is supported
- [ ] Skip to main content link is added
- [ ] All tests pass
- [ ] Accessibility requirements met (WCAG 2.1 AA)

---

### Story 01.06: Implement Design System Tokens
**Story Points:** 3  
**Priority:** P1 (High)  
**Estimated Duration:** 2-3 days

#### Technical Approach
Create a comprehensive design system that ensures visual consistency across the application.

#### Implementation Steps

1. **Define Color Palette**
   ```typescript
   // src/lib/design-tokens/colors.ts
   export const colors = {
     primary: {
       50: 'hsl(221 100% 97%)',
       100: 'hsl(221 83% 93%)',
       200: 'hsl(221 83% 85%)',
       300: 'hsl(221 83% 77%)',
       400: 'hsl(221 83% 67%)',
       500: 'hsl(221 83% 53%)',
       600: 'hsl(221 83% 47%)',
       700: 'hsl(221 83% 40%)',
       800: 'hsl(221 83% 33%)',
       900: 'hsl(221 83% 27%)',
       950: 'hsl(221 83% 20%)',
     },
     semantic: {
       success: {
         50: 'hsl(142 76% 36%)',
         100: 'hsl(142 72% 30%)',
         500: 'hsl(142 72% 29%)',
         600: 'hsl(142 71% 25%)',
       },
       warning: {
         50: 'hsl(38 92% 50%)',
         100: 'hsl(38 92% 47%)',
         500: 'hsl(38 92% 45%)',
         600: 'hsl(38 92% 40%)',
       },
       error: {
         50: 'hsl(0 84% 60%)',
         100: 'hsl(0 84% 55%)',
         500: 'hsl(0 84% 52%)',
         600: 'hsl(0 84% 48%)',
       },
       info: {
         50: 'hsl(204 94% 94%)',
         100: 'hsl(204 94% 87%)',
         500: 'hsl(204 94% 70%)',
         600: 'hsl(204 94% 60%)',
       },
     },
     neutral: {
       50: 'hsl(210 40% 98%)',
       100: 'hsl(210 40% 96%)',
       200: 'hsl(210 40% 94%)',
       300: 'hsl(210 40% 90%)',
       400: 'hsl(210 40% 80%)',
       500: 'hsl(210 40% 65%)',
       600: 'hsl(210 40% 45%)',
       700: 'hsl(210 40% 35%)',
       800: 'hsl(210 40% 25%)',
       900: 'hsl(210 40% 15%)',
       950: 'hsl(210 40% 10%)',
     },
   };
   ```

2. **Define Typography Scale**
   ```typescript
   // src/lib/design-tokens/typography.ts
   export const typography = {
     fontFamily: {
       sans: ['Inter', 'system-ui', 'sans-serif'],
       mono: ['JetBrains Mono', 'Courier New', 'monospace'],
     },
     fontSize: {
       xs: ['0.75rem', { lineHeight: '1rem' }],
       sm: ['0.875rem', { lineHeight: '1.25rem' }],
       base: ['1rem', { lineHeight: '1.5rem' }],
       lg: ['1.125rem', { lineHeight: '1.75rem' }],
       xl: ['1.25rem', { lineHeight: '1.75rem' }],
       '2xl': ['1.5rem', { lineHeight: '2rem' }],
       '3xl': ['1.875rem', { lineHeight: '2.25rem' }],
       '4xl': ['2.25rem', { lineHeight: '2.5rem' }],
       '5xl': ['3rem', { lineHeight: '1' }],
     },
     fontWeight: {
       normal: '400',
       medium: '500',
       semibold: '600',
       bold: '700',
     },
   };
   ```

3. **Define Spacing Scale**
   ```typescript
   // src/lib/design-tokens/spacing.ts
   export const spacing = {
     xs: '0.25rem',   // 4px
     sm: '0.5rem',    // 8px
     md: '0.75rem',   // 12px
     lg: '1rem',      // 16px
     xl: '1.5rem',    // 24px
     '2xl': '2rem',   // 32px
     '3xl': '3rem',   // 48px
     '4xl': '4rem',   // 64px
   };
   ```

4. **Define Border Radius**
   ```typescript
   // src/lib/design-tokens/border-radius.ts
   export const borderRadius = {
     none: '0',
     sm: '0.125rem',   // 2px
     md: '0.375rem',   // 6px
     lg: '0.5rem',     // 8px
     xl: '0.75rem',    // 12px
     '2xl': '1rem',    // 16px
     full: '9999px',
   };
   ```

5. **Define Shadows**
   ```typescript
   // src/lib/design-tokens/shadows.ts
   export const shadows = {
     sm: '0 1px 2px 0 hsl(0 0% 0% / 0.05)',
     md: '0 4px 6px -1px hsl(0 0% 0% / 0.1), 0 2px 4px -2px hsl(0 0% 0% / 0.1)',
     lg: '0 10px 15px -3px hsl(0 0% 0% / 0.1), 0 4px 6px -4px hsl(0 0% 0% / 0.1)',
     xl: '0 20px 25px -5px hsl(0 0% 0% / 0.1), 0 8px 10px -6px hsl(0 0% 0% / 0.1)',
     '2xl': '0 25px 50px -12px hsl(0 0% 0% / 0.25)',
   };
   ```

6. **Export All Tokens**
   ```typescript
   // src/lib/design-tokens/index.ts
   import { colors } from './colors';
   import { typography } from './typography';
   import { spacing } from './spacing';
   import { borderRadius } from './border-radius';
   import { shadows } from './shadows';

   export const designTokens = {
     colors,
     typography,
     spacing,
     borderRadius,
     shadows,
   };

   export type DesignTokens = typeof designTokens;
   ```

7. **Update TailwindCSS Configuration**
   ```javascript
   // tailwind.config.js
   const { colors } = require('./src/lib/design-tokens/colors');

   module.exports = {
     theme: {
       extend: {
         colors: {
           border: 'hsl(var(--border))',
           input: 'hsl(var(--input))',
           ring: 'hsl(var(--ring))',
           background: 'hsl(var(--background))',
           foreground: 'hsl(var(--foreground))',
           primary: {
             DEFAULT: 'hsl(var(--primary))',
             foreground: 'hsl(var(--primary-foreground))',
             50: colors.primary[50],
             100: colors.primary[100],
             // ... etc
           },
         },
       },
     },
   };
   ```

8. **Create CSS Variables for Runtime Theming**
   ```css
   /* src/styles/theme.css */
   :root {
     --color-primary-50: 221 100% 97%;
     --color-primary-100: 221 83% 93%;
     /* ... */
   }

   .dark {
     /* Dark mode color adjustments */
   }
   ```

#### TDD Approach
**Unit Tests:**
```typescript
// tests/lib/design-tokens/colors.test.ts
import { colors } from '@/lib/design-tokens/colors';

describe('color tokens', () => {
  it('should have complete primary color scale', () => {
    expect(colors.primary).toHaveProperty('50');
    expect(colors.primary).toHaveProperty('100');
    expect(colors.primary).toHaveProperty('500');
    expect(colors.primary).toHaveProperty('900');
  });

  it('should have semantic colors', () => {
    expect(colors.semantic).toHaveProperty('success');
    expect(colors.semantic).toHaveProperty('warning');
    expect(colors.semantic).toHaveProperty('error');
    expect(colors.semantic).toHaveProperty('info');
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/theme.test.ts
import { test, expect } from '@playwright/test';

test('theme tokens are applied consistently', async ({ page }) => {
  await page.goto('/dashboard');

  // Check primary color is used
  const primaryElement = page.locator('[data-testid="primary-element"]');
  await expect(primaryElement).toHaveCSS('color', /hsl\(221/);

  // Test dark mode
  await page.click('[data-testid="theme-toggle"]');
  const darkElement = page.locator('[data-testid="background"]');
  await expect(darkElement).toHaveCSS('background-color', /hsl\(222/);
});
```

#### Definition of Done
- [ ] Color palette is defined
- [ ] Typography scale is defined
- [ ] Spacing scale is defined
- [ ] Border radius tokens are defined
- [ ] Shadow tokens are defined
- [ ] Tokens are exported as TypeScript types
- [ ] CSS variables are generated
- [ ] Theme configuration is created
- [ ] All tests pass

---

### Story 01.07: Set Up Development Tools
**Story Points:** 2  
**Priority:** P1 (High)  
**Estimated Duration:** 1-2 days

#### Technical Approach
Configure development tools to improve code quality, enforce standards, and enable efficient workflows.

#### Implementation Steps

1. **Install and Configure Husky**
   ```bash
   npm install --save-dev husky
   npx husky install
   npx husky add .husky/pre-commit "npm run lint && npm run type-check && npm test"
   npx husky add .husky/commit-msg "npx commitlint --edit $1"
   ```

2. **Configure Commitlint**
   ```bash
   npm install --save-dev @commitlint/config-conventional @commitlint/cli
   ```

   ```json
   // .commitlintrc.json
   {
     "extends": ["@commitlint/config-conventional"]
   }
   ```

3. **Configure Commitizen**
   ```bash
   npm install --save-dev commitizen
   npm install --save-dev cz-conventional-changelog
   ```

   ```json
   // package.json
   {
     "scripts": {
       "commit": "cz"
     },
     "config": {
       "commitizen": {
         "path": "./node_modules/cz-conventional-changelog"
       }
     }
   }
   ```

4. **Create GitHub Actions Workflow**
   ```yaml
   # .github/workflows/ci.yml
   name: CI

   on:
     push:
       branches: [ main, develop ]
     pull_request:
       branches: [ main ]

   jobs:
     test:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3

         - name: Setup Node.js
           uses: actions/setup-node@v3
           with:
             node-version: '18'
             cache: 'npm'

         - name: Install dependencies
           run: npm ci

         - name: Run linter
           run: npm run lint

         - name: Run type check
           run: npm run type-check

         - name: Run unit tests
           run: npm test

         - name: Build application
           run: npm run build

         - name: Install Playwright
           run: npx playwright install --with-deps

         - name: Run E2E tests
           run: npm run test:e2e
   ```

5. **Create VS Code Settings**
   ```json
   // .vscode/settings.json
   {
     "editor.formatOnSave": true,
     "editor.defaultFormatter": "esbenp.prettier-vscode",
     "editor.codeActionsOnSave": {
       "source.fixAll.eslint": true
     },
     "typescript.tsdk": "node_modules/typescript/lib",
     "typescript.enablePromptUseWorkspaceTsdk": true,
     "[typescript]": {
       "editor.defaultFormatter": "esbenp.prettier-vscode"
     },
     "[typescriptreact]": {
       "editor.defaultFormatter": "esbenp.prettier-vscode"
     },
     "[json]": {
       "editor.defaultFormatter": "esbenp.prettier-vscode"
     },
     "emmet.includeLanguages": {
       "typescript": "html"
     }
   }
   ```

6. **Create VS Code Extensions Recommendations**
   ```json
   // .vscode/extensions.json
   {
     "recommendations": [
       "bradlc.vscode-tailwindcss",
       "esbenp.prettier-vscode",
       "dbaeumer.vscode-eslint",
       "ms-vscode.vscode-typescript-next",
       "ms-playwright.playwright"
     ]
   }
   ```

7. **Create Jest Configuration**
   ```javascript
   // jest.config.js
   const nextJest = require('next/jest');

   const createJestConfig = nextJest({
     dir: './',
   });

   const customJestConfig = {
     moduleDirectories: ['node_modules', '<rootDir>/'],
     testEnvironment: 'jest-environment-jsdom',
     setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
     moduleNameMapping: {
       '^@/(.*)$': '<rootDir>/src/$1',
     },
     testMatch: [
       '**/__tests__/**/*.(test|spec).(ts|tsx)',
       '**/*.(test|spec).(ts|tsx)',
     ],
   };

   module.exports = createJestConfig(customJestConfig);
   ```

   ```javascript
   // jest.setup.js
   import '@testing-library/jest-dom';
   ```

8. **Create Playwright Configuration**
   ```javascript
   // playwright.config.ts
   import { defineConfig, devices } from '@playwright/test';

   export default defineConfig({
     testDir: './tests/e2e',
     fullyParallel: true,
     forbidOnly: !!process.env.CI,
     retries: process.env.CI ? 2 : 0,
     workers: process.env.CI ? 1 : undefined,
     reporter: 'html',
     use: {
       baseURL: 'http://localhost:3000',
       trace: 'on-first-retry',
     },
     projects: [
       {
         name: 'chromium',
         use: { ...devices['Desktop Chrome'] },
       },
       {
         name: 'firefox',
         use: { ...devices['Desktop Firefox'] },
       },
       {
         name: 'webkit',
         use: { ...devices['Desktop Safari'] },
       },
     ],
     webServer: {
       command: 'npm run dev',
       url: 'http://localhost:3000',
       reuseExistingServer: !process.env.CI,
     },
   });
   ```

9. **Update package.json Scripts**
   ```json
   {
     "scripts": {
       "dev": "next dev",
       "build": "next build",
       "start": "next start",
       "lint": "next lint",
       "lint:fix": "next lint --fix",
       "type-check": "tsc --noEmit",
       "test": "jest",
       "test:watch": "jest --watch",
       "test:coverage": "jest --coverage",
       "test:e2e": "playwright test",
       "test:e2e:ui": "playwright test --ui",
       "commit": "cz"
     }
   }
   ```

10. **Create Development Documentation**
    ```markdown
    # Development Guide

    ## Getting Started

    1. Install dependencies: `npm install`
    2. Run development server: `npm run dev`
    3. Open http://localhost:3000

    ## Available Scripts

    - `npm run dev` - Start development server
    - `npm run build` - Build for production
    - `npm run start` - Start production server
    - `npm run lint` - Run ESLint
    - `npm run type-check` - Run TypeScript type checking
    - `npm test` - Run unit tests
    - `npm run test:watch` - Run tests in watch mode
    - `npm run test:coverage` - Run tests with coverage
    - `npm run test:e2e` - Run E2E tests
    - `npm run commit` - Commit with conventional commits

    ## Code Style

    - Use TypeScript for all files
    - Follow ESLint and Prettier rules
    - Write tests for all new features
    - Use conventional commit messages

    ## Testing

    - Unit tests: Jest + React Testing Library
    - E2E tests: Playwright
    - Coverage target: ≥ 80%

    ## Git Workflow

    1. Create feature branch: `git checkout -b feature/new-feature`
    2. Make changes and commit: `npm run commit`
    3. Push and create PR
    4. Request review
    5. Merge after approval
    ```

#### TDD Approach
**Unit Tests:**
```typescript
// tests/dev-tools/husky.test.ts
import { execSync } from 'child_process';

describe('development tools', () => {
  it('husky pre-commit hook should exist', () => {
    const hookPath = '.husky/pre-commit';
    expect(hookPath).toExist();
  });

  it('should be able to run lint', () => {
    expect(() => execSync('npm run lint')).not.toThrow();
  });

  it('should be able to run type-check', () => {
    expect(() => execSync('npm run type-check')).not.toThrow();
  });
});
```

**E2E Tests:**
```typescript
// tests/e2e/ci.test.ts
import { test, expect } from '@playwright/test';

test('application passes all quality checks', async ({ page }) => {
  await page.goto('/');

  // Check page loads without console errors
  const logs = [];
  page.on('console', msg => logs.push(msg.text()));
  await page.reload();

  // Filter out non-error logs
  const errors = logs.filter(log => log.includes('error') || log.includes('Error'));
  expect(errors).toHaveLength(0);
});
```

#### Definition of Done
- [ ] Husky is installed and configured
- [ ] Pre-commit hooks are set up
- [ ] Commitizen is configured
- [ ] GitHub Actions workflow is created
- [ ] VS Code settings are provided
- [ ] Development documentation is created
- [ ] All tests pass
- [ ] CI/CD pipeline works

---

## 📊 Sprint Planning

### Sprint 1 (10-12 points)

**Week 1:**
- Day 1-2: Story 01.01 - Initialize Next.js Project with TypeScript
- Day 3-4: Story 01.02 - Configure TailwindCSS
- Day 5: Story 01.04 (partial) - Set Up Project Dependencies

**Week 2:**
- Day 1-3: Story 01.03 - Install and Configure shadcn/ui
- Day 4-5: Story 01.06 - Implement Design System Tokens

### Sprint 2 (9 points)

**Week 1:**
- Day 1-4: Story 01.05 - Create Base Layout Components
- Day 5: Story 01.07 (partial) - Set Up Development Tools

**Week 2:**
- Day 1-2: Story 01.07 (complete) - Set Up Development Tools
- Day 3-4: Testing, bug fixes, documentation
- Day 5: Epic review and planning for Epic 2

---

## 🎯 Definition of Done (Epic Level)

- [ ] All 7 user stories are completed
- [ ] All unit tests pass (≥ 80% coverage)
- [ ] All E2E tests pass
- [ ] Code review completed
- [ ] Security review completed
- [ ] Performance review completed
- [ ] Documentation updated
- [ ] No critical or high-priority bugs
- [ ] Accessibility requirements met (WCAG 2.1 AA)
- [ ] CI/CD pipeline is stable
- [ ] Application builds successfully
- [ ] Design system is consistent
- [ ] All components are accessible
- [ ] Development workflow is optimized

---

## 🚀 Success Metrics

1. **Code Quality**
   - TypeScript compilation: 0 errors
   - ESLint: 0 warnings
   - Test coverage: ≥ 80%
   - E2E tests: 100% pass rate

2. **Performance**
   - First Contentful Paint (FCP): < 1.5s
   - Largest Contentful Paint (LCP): < 2.5s
   - Time to Interactive (TTI): < 3.5s

3. **Accessibility**
   - WCAG 2.1 AA compliance
   - Zero critical accessibility issues
   - Keyboard navigation works everywhere

4. **Developer Experience**
   - Hot reload works in < 500ms
   - Pre-commit hooks complete in < 10s
   - CI/CD pipeline completes in < 5 minutes

---

## 📚 Testing Strategy

### Unit Testing (Jest + React Testing Library)
- **Components:** All UI components
- **Hooks:** All custom hooks
- **Utils:** All utility functions
- **Stores:** All Zustand stores

### E2E Testing (Playwright)
- **Critical Paths:**
  - Page navigation
  - Responsive layout
  - Component interactions
  - Theme switching
  - Accessibility features

### Integration Testing
- Component interactions
- State management
- API integration (when available)

### Coverage Targets
- **Unit Tests:** ≥ 85% line coverage
- **Branch Coverage:** ≥ 75%
- **Function Coverage:** ≥ 90%
- **Statement Coverage:** ≥ 85%

---

## 🔒 Security Considerations

1. **Dependencies**
   - All dependencies are up to date
   - No known vulnerabilities
   - Regular security audits

2. **Code Security**
   - No hardcoded secrets
   - XSS protection enabled
   - CSRF protection configured

3. **Accessibility**
   - WCAG 2.1 AA compliance
   - Keyboard navigation
   - Screen reader support
   - Color contrast requirements

---

## 📖 Documentation Deliverables

1. **Technical Documentation**
   - Architecture decisions (ADRs)
   - Component documentation
   - API documentation (when available)

2. **Developer Guide**
   - Getting started guide
   - Development workflow
   - Coding standards
   - Testing guidelines

3. **Design System**
   - Color palette
   - Typography guide
   - Component library
   - Spacing and layout guide

---

## 🔄 Risks & Mitigations

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Next.js 14+ compatibility issues | High | Low | Thorough testing, use stable versions |
| Design system complexity | Medium | Medium | Start simple, iterate based on needs |
| TypeScript strict mode errors | Medium | Low | Enable incrementally, fix systematically |
| Performance issues | High | Low | Test early, optimize continuously |
| Accessibility issues | High | Low | Test with tools, manual testing, user feedback |

---

## 🎓 Learning Resources

1. **Next.js 14+ Documentation**
   - App Router
   - Server Components
   - Routing

2. **TypeScript Documentation**
   - Strict mode
   - Advanced types
   - Type inference

3. **TailwindCSS Documentation**
   - Configuration
   - Custom design
   - Optimization

4. **shadcn/ui Documentation**
   - Component usage
   - Customization
   - Accessibility

5. **Testing Resources**
   - Jest + React Testing Library
   - Playwright E2E testing
   - Accessibility testing tools

---

## 📅 Timeline

| Week | Sprint | Milestone | Deliverables |
|------|--------|-----------|--------------|
| 1-2 | Sprint 1 | Foundation Complete | Next.js project, TailwindCSS, dependencies |
| 3-4 | Sprint 1 | UI System Complete | shadcn/ui, design tokens |
| 5-6 | Sprint 2 | Layout Complete | Base layout components |
| 7-8 | Sprint 2 | Tools Complete | Dev tools, CI/CD, documentation |

**Total Duration:** 8 weeks (2 sprints)

---

## 👥 Team Responsibilities

### Frontend Developer 1
- Story 01.01 - Next.js Setup
- Story 01.03 - shadcn/ui
- Story 01.05 - Layout Components

### Frontend Developer 2
- Story 01.02 - TailwindCSS
- Story 01.04 - Dependencies
- Story 01.06 - Design Tokens

### Tech Lead
- Story 01.07 - Development Tools
- Code reviews
- Architecture decisions
- Security reviews

### QA Engineer
- Test strategy implementation
- Automated test setup
- Manual testing
- Accessibility audits

---

## 🚀 Next Steps

After completing Epic 1:

1. **Start Epic 2** (Authentication & Multi-Tenancy)
2. **Begin Sprint Planning** for Epic 2
3. **Update Project Roadmap**
4. **Team Retrospective** for Epic 1
5. **Document Lessons Learned**

---

**Document Version:** 1.0  
**Created:** 2025-11-07  
**Status:** Ready for Implementation
