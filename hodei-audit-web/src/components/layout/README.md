# Sidebar Navigation - Hodei Audit

## 📦 Components Created

We've created a modern, responsive sidebar navigation system for the Hodei Audit application.

### 🎨 Components

1. **Sidebar** (`/src/components/layout/sidebar.tsx`)
   - Collapsible sidebar with smooth transitions
   - Logo and application branding
   - Navigation links with active state highlighting
   - Bottom section for settings and logout

2. **Header** (`/src/components/layout/header.tsx`)
   - Search bar
   - Notification bell
   - User avatar dropdown menu

3. **DashboardLayout** (`/src/components/layout/dashboard-layout.tsx`)
   - Layout wrapper combining sidebar and header
   - Main content area with scrolling

## 🚀 Features

### ✨ Sidebar Features
- **Collapsible Design**: Click the arrow to collapse/expand
- **Logo & Branding**: "Hodei Audit" with gradient logo
- **Navigation Links**:
  - Dashboard
  - Events
  - Analytics
  - Compliance
  - Settings
- **Active State**: Current page highlighted with gradient
- **Hover Effects**: Smooth transitions on hover
- **Logout Button**: Red logout button at bottom

### 🎨 Design
- Modern gradient logo (blue to purple)
- Clean white/dark backgrounds
- Smooth animations (300ms transitions)
- Lucide React icons
- Responsive design

## 📖 Usage

### Basic Usage

Wrap your page content with `DashboardLayout`:

```tsx
import { DashboardLayout } from "@/components/layout"

export default function MyPage() {
  return (
    <DashboardLayout>
      <div>
        <h1>My Page Content</h1>
        {/* Your content here */}
      </div>
    </DashboardLayout>
  )
}
```

### Using Individual Components

```tsx
import { Sidebar, Header } from "@/components/layout"

export function MyCustomLayout({ children }) {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="flex-1 flex flex-col">
        <Header />
        <main className="p-6">
          {children}
        </main>
      </div>
    </div>
  )
}
```

### Standalone Sidebar

```tsx
import { Sidebar } from "@/components/layout/sidebar"

export default function Page() {
  return (
    <div className="flex h-screen">
      <Sidebar />
      <main className="flex-1 p-6">
        <h1>Dashboard</h1>
      </main>
    </div>
  )
}
```

## 🎯 Navigation Items

The sidebar includes these navigation items by default:

| Icon | Title | Path |
|------|-------|------|
| LayoutDashboard | Dashboard | /dashboard |
| FileText | Events | /events |
| BarChart3 | Analytics | /analytics |
| ShieldCheck | Compliance | /compliance |
| Settings | Settings | /settings |

## 🎨 Styling

The sidebar uses:
- **Tailwind CSS** for styling
- **CSS variables** for theme colors
- **Gradients** for the logo and active states
- **Lucide React** for icons

### Color Scheme
- Primary: Blue to Purple gradient
- Background: White (light) / Dark gray (dark)
- Active: Blue-600 to Purple-600 gradient
- Text: Gray-700 (light) / Gray-300 (dark)

## 🔧 Customization

### Adding New Navigation Items

Edit `/src/components/layout/sidebar.tsx`:

```tsx
const navItems: NavItem[] = [
  // ...existing items
  {
    title: "Your Page",
    href: "/your-page",
    icon: YourIcon, // Import from lucide-react
  },
]
```

### Changing the Logo

Edit the logo section in `sidebar.tsx`:

```tsx
<div className="flex items-center justify-center w-10 h-10 rounded-lg bg-gradient-to-br from-blue-600 to-purple-600 text-white font-bold text-lg">
  H  // Change this character
</div>
```

### Modifying Colors

The sidebar uses Tailwind classes. Key classes:
- Active gradient: `bg-gradient-to-r from-blue-600 to-purple-600`
- Logo gradient: `from-blue-600 to-purple-600`
- Hover: `hover:bg-gray-100 dark:hover:bg-gray-800`

## 📱 Responsive Design

The sidebar is:
- ✅ Desktop optimized (w-64 expanded, w-16 collapsed)
- ✅ Mobile responsive
- ✅ Touch-friendly
- ✅ Keyboard accessible

## 🎭 Active State

The sidebar automatically highlights the current page using `usePathname`:

```tsx
const isActive = pathname === item.href || pathname.startsWith(item.href + "/")
```

This highlights both exact matches and child routes.

## 🔐 Authentication

The header includes a user menu with:
- User name and email
- Profile link
- Settings link
- Logout button

Currently uses:
```tsx
const { data: session } = useSession()
```

## 📂 File Structure

```
src/components/layout/
├── index.ts              # Export all components
├── sidebar.tsx           # Main sidebar component
├── header.tsx            # Header component
├── dashboard-layout.tsx  # Combined layout wrapper
└── README.md             # This file
```

## 🧪 Example Pages

See `src/app/dashboard/page.tsx` for a complete example of using the `DashboardLayout` component.

## 🎨 Screenshot Preview

The sidebar features:
- **Collapsed state**: Shows only icons (w-16)
- **Expanded state**: Shows icons and labels (w-64)
- **Smooth transitions**: 300ms animation
- **Active highlighting**: Gradient background
- **Hover effects**: Subtle color changes

## 🚀 Next Steps

To integrate this sidebar:
1. Use `DashboardLayout` in your page components
2. Update existing pages to use the new layout
3. Add more navigation items as needed
4. Customize colors and branding
5. Add user authentication integration

## 📝 Notes

- All components are marked with `"use client"` for Next.js App Router
- Uses `lucide-react` for icons (already installed)
- Compatible with dark mode (next-themes)
- No external dependencies beyond existing UI components
