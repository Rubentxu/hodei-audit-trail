'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { cn } from '@/lib/utils'
import { Button } from '@/components/ui/button'
import {
  LayoutDashboard,
  History,
  BarChart3,
  ShieldCheck,
  Settings,
  Users,
} from 'lucide-react'

const navigation = [
  { name: 'Dashboard', href: '/dashboard', icon: LayoutDashboard },
  { name: 'Event History', href: '/events', icon: History },
  { name: 'Analytics', href: '/analytics', icon: BarChart3 },
  { name: 'Compliance', href: '/compliance', icon: ShieldCheck },
  { name: 'Users', href: '/users', icon: Users },
  { name: 'Settings', href: '/settings', icon: Settings },
]

export function Sidebar() {
  const pathname = usePathname()

  return (
    <div className="flex h-full w-64 flex-col bg-card border-r" data-testid="sidebar">
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
  )
}
