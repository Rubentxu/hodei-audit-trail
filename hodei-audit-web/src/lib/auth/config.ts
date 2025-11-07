import { NextAuthOptions } from 'next-auth';
import CredentialsProvider from 'next-auth/providers/credentials';
import { JWTPayload, User } from '@/types/auth';

export const authConfig: NextAuthOptions = {
  providers: [
    CredentialsProvider({
      name: 'Credentials',
      credentials: {
        email: { label: 'Email', type: 'email' },
        password: { label: 'Password', type: 'password' },
      },
      async authorize(credentials) {
        if (!credentials?.email || !credentials?.password) {
          return null;
        }

        // In a real application, this would validate against a database
        // For now, we'll use mock users
        const mockUsers: User[] = [
          {
            id: '1',
            email: 'admin@hodei.com',
            name: 'Admin User',
            role: 'admin',
            tenant_id: 'tenant-1',
            tenant_name: 'Acme Corp',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
          },
          {
            id: '2',
            email: 'analyst@hodei.com',
            name: 'Analyst User',
            role: 'analyst',
            tenant_id: 'tenant-1',
            tenant_name: 'Acme Corp',
            created_at: new Date().toISOString(),
            updated_at: new Date().toISOString(),
          },
        ];

        // Mock authentication - replace with real authentication logic
        const user = mockUsers.find((u) => u.email === credentials.email);

        if (user) {
          return {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            tenant_id: user.tenant_id,
            tenant_name: user.tenant_name,
          } as any;
        }

        return null;
      },
    }),
  ],
  pages: {
    signIn: '/auth/signin',
    signOut: '/auth/signout',
    error: '/auth/error',
  },
  session: {
    strategy: 'jwt',
    maxAge: 15 * 60, // 15 minutes
  },
  jwt: {
    maxAge: 15 * 60, // 15 minutes
  },
  callbacks: {
    async jwt({ token, user }): Promise<JWTPayload> {
      if (user) {
        token.sub = user.id as string;
        token.email = user.email!;
        token.name = user.name!;
        token.role = (user as any).role;
        token.tenant_id = (user as any).tenant_id;
        token.tenant_name = (user as any).tenant_name;
        token.iat = Math.floor(Date.now() / 1000);
        token.exp = Math.floor(Date.now() / 1000) + 15 * 60; // 15 minutes
      }
      return token as JWTPayload;
    },
    async session({ session, token }) {
      (session.user as any) = {
        id: token.sub,
        email: token.email,
        name: token.name,
        role: token.role,
        tenant_id: token.tenant_id,
        tenant_name: token.tenant_name,
      };
      (session as any).accessToken = token;
      (session as any).expires = new Date(token.exp * 1000).toISOString();
      return session;
    },
  },
  secret: process.env.NEXTAUTH_SECRET || 'your-secret-key-change-in-production',
};
