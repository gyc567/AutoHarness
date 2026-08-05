---
name: e2e-setup
description: >
  Set up an end-to-end test suite in any repo, following practices that make e2e a reliable
  per-PR gate: real flows over bypass, layered assertions, a reusable auth/session helper,
  video+trace evidence, and a compounding suite. Use when a repo has no e2e (or weak e2e)
  and you want system-level tests.
user_invocable: true
---

# Set up an E2E Test Suite

E2E tests verify the whole running system through the app (browser/API), not one module. They are the **per-PR gate**.

## When to use

- "set up e2e"
- "add end-to-end tests"
- "scaffold a test gate"
- "we need e2e coverage"

## Prerequisites

- `dev-local` setup is complete (see `../dev-local/SKILL.md`)
- `verify` skill exists (see `../verify/SKILL.md`)

## Where E2E Lives

Unit/integration tests stay inside each app/package — they own one module.

**System e2e is a dedicated top-level package** (`e2e/` or `tests/e2e/`) — it spans all apps.

## Steps

### 1. Create the E2E directory structure

```bash
mkdir -p e2e
cd e2e

# Initialize based on project type
# For Rust/API: use a separate test binary
# For Node.js: npm init -y && npm install -D playwright
# For Python: pip install playwright
```

### 2. Set up the test framework

For browser apps:
```bash
npm install -D @playwright/test
npx playwright install --with-deps
```

For API apps:
```bash
# Use your preferred HTTP client
pip install requests httpx  # Python
npm install axios          # Node.js
```

### 3. Create the e2e script

Create `scripts/run-e2e.sh`:

```bash
#!/bin/bash
set -e

echo "=== E2E Test Suite ==="

# Ensure dev stack is running
if ! ../scripts/dev-local.sh status 2>/dev/null | grep -q "running"; then
    echo "Starting dev stack..."
    ../scripts/dev-local.sh up
fi

# Run e2e tests
echo "Running E2E tests..."

# For Playwright:
# npx playwright test

# For API tests:
# cargo test --test e2e

echo "=== E2E Complete ==="
```

### 4. Create the first test spec

```typescript
// e2e/specs/example.spec.ts
import { test, expect } from '@playwright/test';

test.describe('Example Flow', () => {
  test('complete user journey', async ({ page }) => {
    // 1. Navigate
    await page.goto('/');
    
    // 2. Action
    await page.click('[data-testid="action-button"]');
    
    // 3. Layered assertions: UI + Server + Product
    await expect(page.locator('.success-message')).toBeVisible();
    
    // Verify server state
    const response = await page.request.get('/api/status');
    expect(response.ok()).toBeTruthy();
  });
});
```

### 5. Add to CI

```yaml
# .github/workflows/e2e.yml
name: E2E Tests
on: [pull_request]
jobs:
  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run E2E
        run: |
          ./scripts/dev-local.sh up
          ./scripts/run-e2e.sh
```

## Best Practices

### ✅ Do: Real flows, not bypasses

```typescript
// BAD: Hardcoded test code
const code = '123456';

// GOOD: Read from local mail server
const code = await readEmailCode('test@example.com');
```

### ✅ Do: Auth helper once

```typescript
// auth.helper.ts
export async function loginAs(page: Page, user: TestUser) {
  await page.goto('/login');
  await page.fill('[name="email"]', user.email);
  await page.fill('[name="password"]', user.password);
  await page.click('[type="submit"]');
  await page.waitForURL('/dashboard');
}
```

### ✅ Do: Layered assertions

```typescript
// 1. UI changed
await expect(page.locator('.status')).toHaveText('Active');

// 2. Server agrees
const apiResponse = await api.get('/user/status');
expect(apiResponse.status).toBe('active');

// 3. Product outcome
const dbRecord = await db.query('SELECT status FROM users');
expect(dbRecord.status).toBe('active');
```

### ❌ Don't: Brittle selectors

```typescript
// BAD: CSS path
await page.click('#app > div > div:nth-child(2) > button');

// GOOD: Role/label/data-testid
await page.getByRole('button', { name: 'Submit' }).click();
// or
await page.click('[data-testid="submit-button"]');
```

### ❌ Don't: Shared test data

```typescript
// BAD: Collision on reruns
const userId = 12345;

// GOOD: Unique per run
const userId = `test-${Date.now()}-${Math.random()}`;
```

## Troubleshooting

| Issue | Cause | Fix |
|-------|-------|-----|
| Flaky tests | Timing issues | Add `waitFor` instead of `sleep` |
| Auth fails | Rate limits | Add delays between tests |
| External API down | 3rd party | Use mock/stub |

## Integration with AutoHarness

AutoHarness can generate test specs for your code:

```bash
autoharness synthesize --code "fn process_user() {}"
```

Then add the generated test to your E2E suite.

## Output

- `e2e/` directory with tests
- `scripts/run-e2e.sh` runner script
- `e2e.config.ts` framework config
