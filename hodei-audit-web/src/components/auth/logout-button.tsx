'use client';

import { signOut } from 'next-auth/react';
import { useRouter } from 'next/navigation';
import { useState } from 'react';
import { LogOut, Loader2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { useSession } from 'next-auth/react';

interface LogoutButtonProps {
  variant?: 'button' | 'menu-item';
  showIcon?: boolean;
  className?: string;
}

export function LogoutButton({
  variant = 'button',
  showIcon = true,
  className = ''
}: LogoutButtonProps) {
  const [isLoggingOut, setIsLoggingOut] = useState(false);
  const router = useRouter();

  const handleLogout = async () => {
    setIsLoggingOut(true);
    try {
      // Clear local storage (tenant selection, etc.)
      if (typeof window !== 'undefined') {
        localStorage.removeItem('selectedTenant');
      }

      // Sign out from NextAuth and redirect to login
      await signOut({
        redirect: true,
        callbackUrl: '/auth/login'
      });
    } catch (error) {
      console.error('Logout error:', error);
      setIsLoggingOut(false);
    }
  };

  if (variant === 'menu-item') {
    return (
      <DropdownMenuItem
        onClick={handleLogout}
        disabled={isLoggingOut}
        className={`cursor-pointer ${className}`}
      >
        {showIcon && <LogOut className="mr-2 h-4 w-4" />}
        {isLoggingOut ? (
          <>
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            Logging out...
          </>
        ) : (
          'Log out'
        )}
      </DropdownMenuItem>
    );
  }

  return (
    <Button
      variant="outline"
      onClick={handleLogout}
      disabled={isLoggingOut}
      className={className}
    >
      {isLoggingOut ? (
        <>
          <Loader2 className="mr-2 h-4 w-4 animate-spin" />
          Logging out...
        </>
      ) : (
        <>
          {showIcon && <LogOut className="mr-2 h-4 w-4" />}
          Log out
        </>
      )}
    </Button>
  );
}

/**
 * Full logout handler that can be used programmatically
 */
export async function handleLogout() {
  // Clear local storage
  if (typeof window !== 'undefined') {
    localStorage.removeItem('selectedTenant');
  }

  // Sign out and redirect
  await signOut({
    redirect: true,
    callbackUrl: '/auth/login'
  });
}
