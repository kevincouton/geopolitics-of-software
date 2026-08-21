import { test, expect } from '@playwright/test'

test('project detail page loads', async ({ page }) => {
  await page.goto('/projects/octocat/hello')
  await expect(page.locator('text=Loading project details')).toBeVisible()
})

test('project detail page shows not found state for missing project', async ({ page }) => {
  await page.goto('/projects/nobody/nothing')
  await expect(page.locator('text=Could not load project')).toBeVisible()
})
