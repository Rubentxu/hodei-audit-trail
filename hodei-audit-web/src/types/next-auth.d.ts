import NextAuth from "next-auth"
import { UserRole } from "./auth"

declare module "next-auth" {
  interface Session {
    user?: {
      id?: string
      name?: string | null
      email?: string | null
      image?: string | null
      role?: UserRole
      tenant_id?: string
      tenant_name?: string
    }
  }
}

declare module "next-auth/jwt" {
  interface JWT {
    sub?: string
    email?: string
    name?: string
    role?: UserRole
    tenant_id?: string
    tenant_name?: string
  }
}
