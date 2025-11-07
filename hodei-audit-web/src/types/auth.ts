/// Authentication and authorization types

export type UserRole = 'admin' | 'analyst' | 'viewer' | 'auditor';

export interface User {
  id: string;
  email: string;
  name: string;
  role: UserRole;
  tenant_id: string;
  tenant_name: string;
  created_at: string;
  updated_at: string;
}

export interface Tenant {
  id: string;
  name: string;
  slug: string;
  created_at: string;
  updated_at: string;
  is_active: boolean;
}

export interface JWTPayload {
  sub: string;
  email: string;
  name: string;
  role: UserRole;
  tenant_id: string;
  tenant_name: string;
  iat: number;
  exp: number;
}

export interface SignInCredentials {
  email: string;
  password: string;
}

export interface SessionData {
  user: User;
  accessToken: string;
  expires: string;
}

export interface TokenData {
  accessToken: string;
  expires: number;
}
