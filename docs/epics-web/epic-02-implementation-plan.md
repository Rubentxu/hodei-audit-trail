# Epic 02: Authentication & Multi-Tenancy - Implementation Plan

## 📋 Overview
**Epic ID:** EPIC-02  
**Total Story Points:** 42  
**Estimated Duration:** 2 sprints (4 weeks)  
**Team Size:** 2-3 developers  
**Priority:** P0 (Critical)  
**Status:** Ready for Development

---

## 🎯 Goals & Business Value

Enable secure, multi-tenant access to the Hodei Audit web application with:
- **NextAuth.js** for authentication
- **JWT tokens** for session management
- **Role-based access control** (RBAC)
- **Tenant isolation** for security
- **Seamless user experience**

---

## 📦 Dependencies

### Prerequisites (Before Starting)
- ✅ Epic 01 (Project Foundation) must be completed
- ✅ Next.js 14+ project with TypeScript
- ✅ shadcn/ui component library installed
- ✅ TailwindCSS configured
- ✅ Development environment ready

### Parallel Work
- 🔄 Epic 07 (API Integration) - Can start after Story 02.02

---

## 🚀 Implementation Strategy

### Phase 1: Core Authentication (Sprint 1)
**Stories:** 02.01, 02.02, 02.06, 02.07  
**Story Points:** 15  
**Focus:** Basic authentication infrastructure

### Phase 2: Multi-Tenancy (Sprint 1-2)
**Stories:** 02.03, 02.09  
**Story Points:** 8  
**Focus:** Tenant selection and isolation

### Phase 3: Authorization & Security (Sprint 2)
**Stories:** 02.04, 02.05, 02.08  
**Story Points:** 13  
**Focus:** RBAC and protected routes

### Phase 4: Advanced Features (Sprint 2)
**Stories:** 02.10  
**Story Points:** 8  
**Focus:** Admin features (optional for MVP)

---

## 📅 Detailed Sprint Planning

### Sprint 1 (2 weeks) - 21 Story Points

#### Week 1
**Day 1-2: Story 02.01 - NextAuth.js Configuration (5 pts)**
- Install NextAuth.js
- Configure JWT strategy
- Set up auth API routes
- Create session handling
- Implement sign-in/sign-out pages
- **Testing:** Unit tests for auth config, E2E tests for login flow

**Day 3-4: Story 02.02 - JWT Token Structure (3 pts)**
- Define JWT token interface
- Create signing/verification functions
- Implement token validation middleware
- **Testing:** Unit tests for token handling, E2E tests for token validation

**Day 5: Story 02.06 - Create Login Page (5 pts)**
- Build login form component
- Add form validation
- Implement error handling
- Style with TailwindCSS
- **Testing:** Unit tests for form, E2E tests for login flow

#### Week 2
**Day 1-2: Story 02.07 - Logout Functionality (2 pts)**
- Implement logout button
- Clear session and tokens
- Redirect to login
- **Testing:** E2E tests for logout

**Day 3-4: Story 02.03 - Tenant Selector (3 pts)**
- Create tenant selector component
- Add dropdown UI
- Store selected tenant
- **Testing:** Unit tests for component, E2E tests for selection

**Day 5: Story 02.09 - Session Management (5 pts)**
- Implement session persistence
- Add auto-refresh
- Handle session timeout
- Multi-tab synchronization
- **Testing:** E2E tests for session behavior

**Day 5: Buffer for completion and code review**

### Sprint 2 (2 weeks) - 21 Story Points

#### Week 1
**Day 1-2: Story 02.04 - Protected Route Middleware (3 pts)**
- Create Next.js middleware
- Validate JWT tokens
- Implement redirects
- **Testing:** Unit tests for middleware, E2E tests for protected routes

**Day 3-4: Story 02.05 - RBAC Implementation (5 pts)**
- Define roles (admin, analyst, viewer, auditor)
- Create permission checking
- Implement UI hiding based on role
- Add route protection by role
- **Testing:** Unit tests for RBAC, E2E tests for role-based access

**Day 5: Story 02.08 - User Profile Page (3 pts)**
- Create profile page
- Display user info
- Add profile editing
- **Testing:** Unit tests for profile, E2E tests for profile management

#### Week 2
**Day 1-3: Story 02.10 - Tenant Management (Admin Only) (8 pts)**
- Create tenant management page
- Implement CRUD operations
- Add user-tenant assignments
- **Testing:** Unit tests for tenant mgmt, E2E tests for admin features

**Day 4-5: Buffer for:**
- Integration testing
- Security review
- Performance testing
- Bug fixes
- Documentation

---

## 🛠️ Technical Implementation Details

### Story 02.01: NextAuth.js Configuration

**Implementation Steps:**
1. Install NextAuth.js
```bash
npm install next-auth
```

2. Configure NextAuth in `lib/auth.ts`:
```typescript
import NextAuth from "next-auth"
import { authOptions } from "@/lib/auth"

const handler = NextAuth(authOptions)
```

3. Set up JWT strategy in `lib/auth-config.ts`:
```typescript
export const authOptions = {
  session: {
    strategy: "jwt",
  },
  secret: process.env.NEXTAUTH_SECRET,
  // ... other config
}
```

4. Create API route in `app/api/auth/[...nextauth]/route.ts`

5. Add session provider in layout

**File Structure:**
```
lib/
├── auth.ts
├── auth-config.ts
└── types/
    └── auth.ts
app/
├── api/
│   └── auth/
│       └── [...nextauth]/
│           └── route.ts
└── (auth)/
    ├── signin/
    │   └── page.tsx
    └── signup/
        └── page.tsx
```

**API Endpoints Created:**
- `POST /api/auth/signin` - Sign in
- `POST /api/auth/signout` - Sign out
- `GET /api/auth/session` - Get session

---

### Story 02.02: JWT Token Structure

**JWT Token Claims:**
```typescript
interface JWTPayload {
  sub: string;        // user ID
  email: string;      // user email
  tenant_id: string;  // current tenant
  roles: string[];    // user roles
  iat: number;        // issued at
  exp: number;        // expiration
}
```

**Implementation:**
1. Create token utilities in `lib/jwt.ts`:
   - `signToken()` - Create JWT
   - `verifyToken()` - Validate JWT
   - `decodeToken()` - Extract claims

2. Add middleware for token validation

3. Create TypeScript types

**Dependencies:**
- `jsonwebtoken` library
- Environment variables for secret

---

### Story 02.03: Tenant Selector Component

**Component Design:**
```typescript
interface TenantSelectorProps {
  tenants: Tenant[]
  selectedTenant: string
  onTenantChange: (tenantId: string) => void
}
```

**Implementation:**
1. Create `components/tenant/TenantSelector.tsx`
2. Use shadcn/ui Select component
3. Store selection in Zustand store
4. Persist in localStorage
5. Add to header component

**State Management:**
```typescript
// lib/store/tenant-store.ts
interface TenantState {
  currentTenant: string | null
  setTenant: (tenantId: string) => void
  tenants: Tenant[]
}
```

---

### Story 02.04: Protected Route Middleware

**Middleware Implementation:**
```typescript
// middleware.ts
import { NextResponse } from 'next/server'
import { getToken } from 'next-auth/jwt'

export async function middleware(request: NextRequest) {
  const token = await getToken({ req: request, secret: process.env.NEXTAUTH_SECRET })
  
  if (!token) {
    return NextResponse.redirect(new URL('/auth/signin', request.url))
  }
  
  return NextResponse.next()
}

export const config = {
  matcher: ['/dashboard/:path*', '/events/:path*', '/analytics/:path*']
}
```

**Protected Routes:**
- `/dashboard/*` - Requires authentication
- `/events/*` - Requires authentication
- `/analytics/*` - Requires authentication
- `/compliance/*` - Requires admin/auditor role

---

### Story 02.05: Role-Based Access Control (RBAC)

**Roles Definition:**
```typescript
enum UserRole {
  ADMIN = 'admin',        // Full access
  ANALYST = 'analyst',    // View and analyze
  VIEWER = 'viewer',      // Read-only
  AUDITOR = 'auditor'     // Compliance reports
}
```

**Implementation:**
1. Create permission utility `lib/rbac.ts`:
   - `hasRole()` - Check user role
   - `canAccess()` - Check resource permission
   - `getPermissions()` - Get user permissions

2. Add role checking in components:
```typescript
const { data: session } = useSession()
const canManageUsers = usePermission('users:manage')
```

3. Create Higher-Order Component (HOC):
```typescript
function withAuth<P>(Component: React.ComponentType<P>, requiredRole?: UserRole) {
  return function AuthenticatedComponent(props: P) {
    // Role checking logic
  }
}
```

**Permission Matrix:**
```
Resource          | Admin | Analyst | Viewer | Auditor
------------------|-------|---------|--------|--------
Dashboard         |   ✓   |    ✓    |   ✓    |   ✓
Event History     |   ✓   |    ✓    |   ✓    |   ✓
Analytics         |   ✓   |    ✓    |   ✗    |   ✗
Compliance        |   ✓   |    ✗    |   ✗    |   ✓
User Management   |   ✓   |    ✗    |   ✗    |   ✗
Tenant Management |   ✓   |    ✗    |   ✗    |   ✗
```

---

### Story 02.06: Login Page

**Page Structure:**
```
app/(auth)/
└── signin/
    ├── page.tsx              # Main login page
    ├── components/
    │   ├── LoginForm.tsx     # Form component
    │   └── LoginCard.tsx     # Card wrapper
    └── hooks/
        └── useLogin.ts       # Login hook
```

**Form Features:**
- Email and password fields
- Form validation with Zod
- Error display
- Loading state
- "Remember me" option
- "Forgot password" link
- Social login (future)

**Validation Schema (Zod):**
```typescript
const loginSchema = z.object({
  email: z.string().email('Invalid email'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
  remember: z.boolean().optional()
})
```

**Styling:**
- Use shadcn/ui components
- TailwindCSS for styling
- Responsive design
- Dark mode support
- Loading skeletons

---

### Story 02.07: Logout Functionality

**Implementation:**
1. Add logout button to user menu in header
2. Implement logout action:
```typescript
const handleLogout = async () => {
  await signOut({ callbackUrl: '/auth/signin' })
  // Clear local storage
  localStorage.removeItem('selectedTenant')
}
```

3. Add confirmation dialog
4. Clear all state (Zustand, React Query)
5. Redirect to sign-in page

**User Menu Component:**
```typescript
// components/layout/UserMenu.tsx
<DropdownMenu>
  <DropdownMenuTrigger>User Avatar</DropdownMenuTrigger>
  <DropdownMenuContent>
    <DropdownMenuItem>Profile</DropdownMenuItem>
    <DropdownMenuItem>Settings</DropdownMenuItem>
    <DropdownMenuSeparator />
    <DropdownMenuItem onClick={handleLogout}>
      Logout
    </DropdownMenuItem>
  </DropdownMenuContent>
</DropdownMenu>
```

---

### Story 02.08: User Profile Page

**Page Structure:**
```
app/
└── profile/
    ├── page.tsx              # Profile page
    ├── components/
    │   ├── ProfileForm.tsx   # Edit form
    │   ├── ProfileInfo.tsx   # Display info
    │   └── ChangePassword.tsx # Password change
    └── hooks/
        └── useProfile.ts     # Profile hook
```

**Features:**
- Display user information
- Edit profile (name, email)
- Change password
- View tenant information
- View role and permissions
- View last login

**Update Profile API:**
```typescript
// app/api/profile/route.ts
export async function PUT(request: Request) {
  const session = await getServerSession(authOptions)
  // Update user profile
}
```

---

### Story 02.09: Session Management

**Features to Implement:**

1. **Session Persistence:**
   - Store in httpOnly cookie
   - Configure session expiry (15 min)
   - Implement sliding expiration

2. **Auto-Refresh:**
   - Refresh token before expiry
   - Background refresh
   - Notify user before expiry

3. **Multi-Tab Synchronization:**
   - Broadcast session changes
   - Sync tenant selection
   - Sync user preferences

**Implementation:**
```typescript
// hooks/useSession.ts
export function useSession() {
  const [session, setSession] = useState<Session | null>(null)
  
  useEffect(() => {
    // Setup session listener
    const interval = setInterval(() => {
      // Refresh session
    }, 5 * 60 * 1000) // 5 minutes
    
    return () => clearInterval(interval)
  }, [])
}
```

**Session Timeout Warning:**
```typescript
// components/SessionTimeoutWarning.tsx
export function SessionTimeoutWarning() {
  const [showWarning, setShowWarning] = useState(false)
  
  useEffect(() => {
    const warningTime = 14 * 60 * 1000 // 14 min
    const timeout = setTimeout(() => {
      setShowWarning(true)
    }, warningTime)
    
    return () => clearTimeout(timeout)
  }, [])
}
```

---

### Story 02.10: Tenant Management (Admin Only)

**Page Structure:**
```
app/
└── admin/
    └── tenants/
        ├── page.tsx              # Tenant list
        ├── [id]/
        │   └── page.tsx          # Tenant details
        ├── new/
        │   └── page.tsx          # Create tenant
        └── components/
            ├── TenantForm.tsx
            ├── TenantList.tsx
            └── UserAssignment.tsx
```

**Features:**
- List all tenants
- Create new tenant
- Edit tenant details
- Deactivate tenant
- Assign users to tenant
- View tenant statistics

**API Endpoints:**
- `GET /api/admin/tenants` - List tenants
- `POST /api/admin/tenants` - Create tenant
- `PUT /api/admin/tenants/:id` - Update tenant
- `DELETE /api/admin/tenants/:id` - Delete tenant
- `GET /api/admin/tenants/:id/users` - Get tenant users
- `POST /api/admin/tenants/:id/users` - Assign user

**Data Table:**
```typescript
// Using TanStack Table
const columns: ColumnDef<Tenant>[] = [
  { accessorKey: 'name', header: 'Name' },
  { accessorKey: 'domain', header: 'Domain' },
  { accessorKey: 'status', header: 'Status' },
  { accessorKey: 'createdAt', header: 'Created' },
  {
    id: 'actions',
    cell: ({ row }) => <TenantActions tenant={row.original} />
  }
]
```

---

## 🧪 Testing Strategy

### Unit Tests
**Coverage Required:** ≥ 85%

**Test Files:**
- `__tests__/lib/auth.test.ts`
- `__tests__/lib/jwt.test.ts`
- `__tests__/lib/rbac.test.ts`
- `__tests__/components/tenant/TenantSelector.test.tsx`
- `__tests__/components/auth/LoginForm.test.tsx`
- `__tests__/hooks/useAuth.test.ts`
- `__tests__/hooks/useSession.test.ts`

**Test Examples:**
```typescript
// auth.test.ts
describe('Authentication', () => {
  it('should create session with correct token', async () => {
    const token = await signToken({ userId: '123', email: 'test@example.com' })
    expect(token).toBeDefined()
  })
})
```

### Integration Tests
- Test auth flow end-to-end
- Test tenant switching
- Test role-based access
- Test protected routes
- Test session management

### E2E Tests (Playwright)
```typescript
// e2e/auth.spec.ts
test('user can sign in and access dashboard', async ({ page }) => {
  await page.goto('/auth/signin')
  await page.fill('[data-testid="email"]', 'test@example.com')
  await page.fill('[data-testid="password"]', 'password123')
  await page.click('[data-testid="signin-button"]')
  await expect(page).toHaveURL('/dashboard')
})

test('tenant selector switches tenant', async ({ page }) => {
  await page.goto('/dashboard')
  await page.click('[data-testid="tenant-selector"]')
  await page.click('[data-value="tenant-2"]')
  expect(await page.getAttribute('[data-testid="current-tenant"]', 'data-value')).toBe('tenant-2')
})
```

**E2E Test Scenarios:**
1. Sign in with valid credentials
2. Sign in with invalid credentials
3. Sign out
4. Protected route access
5. Tenant switching
6. Role-based access
7. Session timeout
8. Multi-tab synchronization
9. Admin tenant management
10. Profile management

---

## 🔐 Security Considerations

### 1. JWT Security
- **Secret Management:** Use strong secrets, store in environment variables
- **Token Expiry:** Short-lived tokens (15 min) with refresh
- **Token Storage:** httpOnly cookies, not localStorage
- **Algorithm:** HS256 or RS256

### 2. Password Security
- **Hashing:** bcrypt with cost factor ≥ 12
- **Validation:** Strong password requirements
- **Rate Limiting:** Limit login attempts
- **MFA:** Consider for future (2FA)

### 3. Session Security
- **Session Fixation:** Regenerate session ID after login
- **CSRF Protection:** SameSite cookies
- **XSS Protection:** Content Security Policy
- **Secure Cookies:** Secure and httpOnly flags

### 4. Multi-Tenancy
- **Tenant Isolation:** All queries must include tenant_id
- **Data Leaking:** Strict tenant boundaries
- **Tenant Header:** Always send X-Tenant-ID
- **Authorization:** Verify tenant access

### 5. Access Control
- **Principle of Least Privilege:** Minimal necessary permissions
- **Role Verification:** Server-side role checks
- **UI Hiding:** Don't rely on UI for security
- **Audit Logging:** Log all access

### 6. Common Vulnerabilities
- **SQL Injection:** Parameterized queries
- **XSS:** Sanitize all inputs
- **CSRF:** Use CSRF tokens
- **Session Hijacking:** Secure cookies, HTTPS
- **Brute Force:** Rate limiting

---

## 📊 Performance Considerations

### 1. Authentication Performance
- **Token Validation:** O(1) with in-memory cache
- **Session Lookup:** Redis for distributed systems
- **Database Queries:** Optimize with indexes

### 2. Multi-Tenancy Performance
- **Tenant Cache:** Cache tenant data
- **Query Optimization:** Tenant_id indexes
- **Connection Pooling:** Reuse DB connections

### 3. UI Performance
- **Component Memoization:** React.memo
- **Lazy Loading:** Dynamic imports
- **Code Splitting:** Route-based
- **Caching:** React Query

### 4. Real-time Updates
- **WebSocket:** For session sync
- **Event Broadcasting:** For multi-tab
- **Optimistic Updates:** Better UX

---

## 🚨 Risks & Mitigation

### High Priority Risks

#### Risk 1: JWT Token Expiry
**Impact:** High  
**Probability:** Medium  
**Mitigation:**
- Implement sliding expiration
- Auto-refresh tokens
- Graceful renewal
- User notification

#### Risk 2: Tenant Data Leakage
**Impact:** Critical  
**Probability:** Low  
**Mitigation:**
- Strict tenant filtering
- SQL injection prevention
- Code reviews
- Security testing

#### Risk 3: Session Hijacking
**Impact:** High  
**Probability:** Medium  
**Mitigation:**
- Secure cookies (httpOnly, Secure, SameSite)
- HTTPS everywhere
- Token rotation
- Monitor suspicious activity

#### Risk 4: Role Escalation
**Impact:** High  
**Probability:** Low  
**Mitigation:**
- Server-side authorization
- Audit logging
- Code reviews
- Security testing

#### Risk 5: Performance Issues
**Impact:** Medium  
**Probability:** High  
**Mitigation:**
- Caching strategy
- Database optimization
- Load testing
- Monitoring

---

## 📈 Success Metrics

### Authentication Metrics
- **Login Success Rate:** ≥ 98%
- **Token Refresh Success:** ≥ 99%
- **Session Duration:** Average 4-6 hours
- **Failed Login Attempts:** < 5% of total

### Security Metrics
- **Vulnerabilities:** 0 critical, 0 high
- **Security Tests:** 100% pass rate
- **Audit Compliance:** 100% coverage
- **Pen Test Results:** No critical issues

### Performance Metrics
- **Login Time:** < 500ms
- **Token Validation:** < 50ms
- **Tenant Switch:** < 200ms
- **Page Load (protected):** < 1.5s

### User Experience Metrics
- **Task Completion Rate:** ≥ 95%
- **Error Rate:** < 1%
- **User Satisfaction:** ≥ 4.5/5
- **Support Tickets:** < 5% of users

---

## 📚 Documentation Required

### Developer Documentation
- [ ] Authentication flow diagram
- [ ] JWT token specification
- [ ] RBAC permission matrix
- [ ] API authentication guide
- [ ] Security best practices

### User Documentation
- [ ] Login guide
- [ ] Password reset guide
- [ ] Tenant management guide
- [ ] Profile management guide
- [ ] Role and permissions guide

### Operations Documentation
- [ ] Environment variables
- [ ] Database schema
- [ ] Deployment guide
- [ ] Monitoring setup
- [ ] Incident response

---

## 🔄 Post-Implementation Tasks

### Epic 02 Complete Checklist
- [ ] All 10 stories completed
- [ ] All tests pass (unit, integration, E2E)
- [ ] Security review passed
- [ ] Performance tests passed
- [ ] Documentation complete
- [ ] Code review complete
- [ ] No critical or high bugs
- [ ] Migration plan (if needed)
- [ ] Monitoring configured
- [ ] Support team trained

### Next Steps
1. **Epic 03: Dashboard & Widgets** - Can start after story 02.01
2. **Epic 07: API Integration** - Can start after story 02.02
3. **Performance Testing** - After all auth features
4. **Security Audit** - Before production deployment
5. **Load Testing** - Before going live

---

## 📞 Team & Responsibilities

### Core Team (2-3 developers)
- **Tech Lead:** Architecture, code reviews, security
- **Developer 1:** Auth configuration, JWT, sessions
- **Developer 2:** UI components, tenant selector, RBAC

### Supporting Team
- **QA Engineer:** Test planning, execution
- **Security Engineer:** Security review, testing
- **Product Owner:** Requirements, acceptance

### Communication
- **Daily Standups:** 15 min sync
- **Sprint Planning:** Begin of sprint
- **Sprint Review:** End of sprint
- **Retrospective:** After sprint review
- **Code Reviews:** Before merge

---

**Document Version:** 1.0  
**Last Updated:** 2025-11-07  
**Status:** Approved for Development  
**Next Review:** After Sprint 1

---

## ✅ Definition of Done

For Epic 02 to be complete:
- [ ] All 10 user stories completed
- [ ] All unit tests pass (≥ 85% coverage)
- [ ] All E2E tests pass
- [ ] All integration tests pass
- [ ] Security review completed
- [ ] Performance tests passed
- [ ] Code review completed
- [ ] Documentation updated
- [ ] No critical or high-severity bugs
- [ ] Authentication flow tested thoroughly
- [ ] Multi-tenancy isolation verified
- [ ] RBAC permissions tested
- [ ] Session management verified
- [ ] Login/logout flow working
- [ ] Protected routes secured
- [ ] Tenant selector functional
- [ ] Profile management working
- [ ] Admin features (if implemented) tested
- [ ] Production deployment ready
