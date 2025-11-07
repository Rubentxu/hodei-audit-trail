"use client";

import { useTheme } from "next-themes";
import { useSession } from "next-auth/react";
import { useRouter } from "next/navigation";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Sun, Moon, User, LogOut, Settings } from "lucide-react";
import { TenantSelector } from "@/components/tenant/tenant-selector";
import { LogoutButton } from "@/components/auth/logout-button";
import { PermissionGuard } from "@/components/auth/permission-guard";
import { Permission } from "@/lib/auth/permissions";

export function Header() {
  const { setTheme, theme } = useTheme();
  const { data: session } = useSession();
  const router = useRouter();

  const handleProfile = () => {
    router.push("/profile");
  };

  const handleSettings = () => {
    router.push("/settings");
  };

  const userName = session?.user?.name || "User";
  const userEmail = session?.user?.email || "";
  const userRole = session?.user?.role || "";

  return (
    <header className="flex h-16 items-center justify-between border-b bg-card px-6">
      <div className="flex items-center space-x-4">
        {session && <TenantSelector />}
      </div>

      <div className="flex items-center space-x-2">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
          data-testid="theme-toggle"
        >
          <Sun className="h-4 w-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-4 w-4 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>

        {session && (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" className="flex items-center space-x-2">
                <User className="h-4 w-4" />
                <span className="text-sm font-medium">{userName}</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end" className="w-56">
              <DropdownMenuLabel>
                <div className="flex flex-col space-y-1">
                  <p className="text-sm font-medium leading-none">{userName}</p>
                  <p className="text-xs leading-none text-muted-foreground">
                    {userEmail}
                  </p>
                  {userRole && (
                    <p className="text-xs leading-none text-muted-foreground">
                      Role: {userRole}
                    </p>
                  )}
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem onClick={handleProfile}>
                <User className="mr-2 h-4 w-4" />
                Profile
              </DropdownMenuItem>
              <PermissionGuard permission={Permission.MANAGE_SETTINGS}>
                <DropdownMenuItem onClick={handleSettings}>
                  <Settings className="mr-2 h-4 w-4" />
                  Settings
                </DropdownMenuItem>
              </PermissionGuard>
              <DropdownMenuSeparator />
              <LogoutButton variant="menu-item" showIcon={true} />
            </DropdownMenuContent>
          </DropdownMenu>
        )}
      </div>
    </header>
  );
}
