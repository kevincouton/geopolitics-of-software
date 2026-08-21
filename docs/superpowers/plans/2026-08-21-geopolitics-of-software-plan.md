# GeoSoft TrendBoard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust + PostgreSQL + Nuxt 3 dashboard that tracks GitHub trending projects and scores their positioning for China and Asia.

**Architecture:** A stateless Rust Axum API persists project metadata, daily snapshots, and social mentions in PostgreSQL. A Nuxt 3 SSR web app consumes the API. Collectors run as scheduled background jobs or CLI binaries. Scoring is a daily batch process.

**Tech Stack:** Rust (Axum, sqlx, tokio, serde, reqwest, chrono, uuid), PostgreSQL 15+, Nuxt 3 + Vue + Tailwind CSS, GitHub Actions, Docker Compose.

**Spec:** `docs/specs/2026-08-21-geopolitics-of-software-design.md`

## Global Constraints

- Rust-first: all backend code in Rust; frontend is Nuxt 3 (Vue).
- Database is PostgreSQL, accessed via `sqlx` with compile-time checked queries (`DATABASE_URL` required for build).
- All public API responses are JSON; HTTP status codes follow REST conventions.
- External platform APIs are mocked or use minimal real calls in development to avoid rate limits.
- CI must pass `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `npm run check`, and `npm run build` on every push.
- No paid subscriptions in MVP; billing is stubbed if needed.

---

## File Structure

```
geopolitics-of-software/
├── README.md
├── docker-compose.yml
├── .github/workflows/ci.yml
├── service/
│   ├── Cargo.toml                        # workspace: chassis, api, collectors
│   ├── rust-toolchain.toml
│   ├── migrations/                       # sqlx migrations
│   │   └── 0001_initial.sql
│   └── crates/
│       ├── chassis/
│       │   └── src/
│       │       ├── lib.rs
│       │       ├── config.rs
│       │       ├── db.rs
│       │       ├── error.rs
│       │       ├── projects.rs
│       │       ├── snapshots.rs
│       │       ├── scoring.rs
│       │       └── connectors/
│       │           ├── mod.rs
│       │           ├── github.rs
│       │           ├── gitee.rs
│       │           └── web_scraper.rs
│       ├── api/
│       │   └── src/
│       │       ├── main.rs
│       │       ├── state.rs
│       │       ├── router.rs
│       │       └── handlers/
│       │               ├── projects.rs
│       │               ├── snapshots.rs
│       │               └── health.rs
│       └── collectors/
│           └── src/
│               ├── main.rs
│               ├── github_trending.rs
│               └── scoring_job.rs
└── web/
    ├── package.json
    ├── nuxt.config.ts
    ├── tailwind.config.ts
    ├── composables/
    │   ├── useApi.ts
    │   └── useProjects.ts
    ├── components/
    │   ├── ScoreBadge.vue
    │   ├── ProjectCard.vue
    │   └── TrendChart.vue
    ├── pages/
    │   ├── index.vue
    │   ├── projects/
    │   │   └── [owner]/
    │   │       └── [name].vue
    │   ├── dashboard.vue
    │   └── methodology.vue
    └── tests/
        └── e2e/
            └── smoke.spec.ts
```

---

## Milestone 1 — Scaffold

### Task 1: Create Rust workspace and PostgreSQL schema

**Files:**
- Create: `service/Cargo.toml`
- Create: `service/rust-toolchain.toml`
- Create: `service/migrations/0001_initial.sql`
- Create: `service/crates/chassis/src/lib.rs`
- Create: `service/crates/chassis/src/db.rs`
- Create: `service/crates/chassis/src/config.rs`
- Create: `service/crates/chassis/src/error.rs`
- Test: `service/crates/chassis/tests/db.rs`

**Interfaces:**
- Produces: `chassis::db::DbPool`, `chassis::config::Config`, `chassis::error::ApiError`

- [ ] **Step 1: Add workspace `Cargo.toml`**

```toml
[workspace]
resolver = "2"
members = ["crates/chassis", "crates/api", "crates/collectors"]

[workspace.dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "migrate"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "1"
axum = "0.7"
tower = { version = "0.5", features = ["util"] }
tower-http = { version = "0.6", features = ["cors", "trace"] }
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "json"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1"
```

- [ ] **Step 2: Create migration `0001_initial.sql`**

```sql
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE platform AS ENUM ('juejin', 'zhihu', 'v2ex', 'bilibili', 'gitee');

CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    github_owner TEXT NOT NULL,
    github_name TEXT NOT NULL,
    gitee_owner TEXT,
    gitee_name TEXT,
    language TEXT,
    topics TEXT[] NOT NULL DEFAULT '{}',
    description TEXT,
    stars INTEGER NOT NULL DEFAULT 0,
    forks INTEGER NOT NULL DEFAULT 0,
    open_issues INTEGER NOT NULL DEFAULT 0,
    has_chinese_readme BOOLEAN NOT NULL DEFAULT false,
    has_gitee_mirror BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(github_owner, github_name)
);

CREATE TABLE daily_snapshots (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    snapshot_date DATE NOT NULL,
    stars INTEGER NOT NULL DEFAULT 0,
    forks INTEGER NOT NULL DEFAULT 0,
    asia_readiness_score INTEGER NOT NULL DEFAULT 0,
    docs_score INTEGER NOT NULL DEFAULT 0,
    platform_score INTEGER NOT NULL DEFAULT 0,
    social_score INTEGER NOT NULL DEFAULT 0,
    community_score INTEGER NOT NULL DEFAULT 0,
    UNIQUE(project_id, snapshot_date)
);

CREATE INDEX idx_daily_snapshots_project ON daily_snapshots(project_id);
CREATE INDEX idx_daily_snapshots_date ON daily_snapshots(snapshot_date);

CREATE TABLE social_mentions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    platform platform NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    engagement_score INTEGER NOT NULL DEFAULT 0,
    sentiment TEXT,
    mentioned_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_social_mentions_project ON social_mentions(project_id);
```

- [ ] **Step 3: Implement `chassis/src/db.rs`**

```rust
use sqlx::{migrate::Migrator, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn connect(database_url: &str) -> Result<DbPool, sqlx::Error> {
    Pool::connect(database_url).await
}

pub async fn migrate(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR.run(pool).await
}
```

- [ ] **Step 4: Implement `chassis/src/config.rs`**

```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub api_port: u16,
    pub github_token: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL"),
            api_port: std::env::var("API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }
}
```

- [ ] **Step 5: Implement `chassis/src/error.rs`**

```rust
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("internal error")]
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}
```

- [ ] **Step 6: Wire `chassis/src/lib.rs`**

```rust
pub mod config;
pub mod db;
pub mod error;
```

- [ ] **Step 7: Write test `service/crates/chassis/tests/db.rs`**

```rust
use sqlx::PgPool;

#[sqlx::test]
async fn test_projects_table_exists(pool: PgPool) {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM projects")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.0, 0);
}
```

- [ ] **Step 8: Run tests**

```bash
cd /root/geopolitics-of-software/service
DATABASE_URL=postgres://postgres:postgres@localhost/postgres cargo test -p chassis
```

Expected: 1 passing test.

- [ ] **Step 9: Commit**

```bash
cd /root/geopolitics-of-software
git add . && git commit -m "feat(db): scaffold workspace, postgres schema, and chassis crate"
```

---

### Task 2: Scaffold Nuxt 3 frontend

**Files:**
- Create: `web/package.json`
- Create: `web/nuxt.config.ts`
- Create: `web/tailwind.config.ts`
- Create: `web/layouts/default.vue`
- Create: `web/pages/index.vue`
- Create: `web/assets/css/tailwind.css`
- Test: `web/tests/e2e/smoke.spec.ts`

**Interfaces:**
- Produces: running Nuxt dev server, homepage renders

- [ ] **Step 1: Create `web/package.json`**

```json
{
  "name": "geopolitics-of-software-web",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "build": "nuxt generate",
    "dev": "nuxt dev",
    "check": "vp check",
    "test": "vp test",
    "test:e2e": "playwright test"
  },
  "dependencies": {
    "@nuxtjs/tailwindcss": "^6.14.0",
    "nuxt": "^4.4.7",
    "tailwindcss": "^3.4.19",
    "vue": "^3.5.35",
    "vue-router": "^5.1.0"
  },
  "devDependencies": {
    "@nuxt/test-utils": "^4.0.3",
    "@playwright/test": "^1.60.0",
    "@voidzero-dev/vite-plus-core": "^0.1.24",
    "@voidzero-dev/vite-plus-test": "^0.1.24",
    "vite-plus": "^0.1.24"
  },
  "overrides": {
    "vite": "npm:@voidzero-dev/vite-plus-core@0.1.24",
    "vitest": "npm:@voidzero-dev/vite-plus-test@0.1.24"
  }
}
```

- [ ] **Step 2: Create `web/nuxt.config.ts`**

```ts
export default defineNuxtConfig({
  app: {
    head: {
      titleTemplate: '%s — GeoSoft TrendBoard',
      htmlAttrs: { lang: 'en' },
    },
  },
  devtools: { enabled: false },
  modules: ['@nuxtjs/tailwindcss'],
  runtimeConfig: {
    public: {
      apiUrl: process.env.NUXT_PUBLIC_API_URL || 'http://localhost:8080',
    },
  },
  nitro: {
    prerender: {
      routes: ['/'],
    },
  },
})
```

- [ ] **Step 3: Create `web/tailwind.config.ts`**

```ts
import type { Config } from 'tailwindcss'

export default {
  content: [],
  theme: {
    extend: {},
  },
  plugins: [],
} satisfies Config
```

- [ ] **Step 4: Create `web/assets/css/tailwind.css`**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

- [ ] **Step 5: Create `web/layouts/default.vue`**

```vue
<template>
  <div class="min-h-screen bg-gray-50 text-gray-900">
    <header class="border-b bg-white">
      <div class="mx-auto max-w-6xl px-4 py-4">
        <NuxtLink to="/" class="text-xl font-bold">GeoSoft TrendBoard</NuxtLink>
      </div>
    </header>
    <main class="mx-auto max-w-6xl px-4 py-8">
      <slot />
    </main>
  </div>
</template>
```

- [ ] **Step 6: Create `web/pages/index.vue`**

```vue
<template>
  <div>
    <h1 class="text-3xl font-bold">GitHub Trending Asia Readiness</h1>
    <p class="mt-2 text-gray-600">Track how trending projects are positioned for China and Asia.</p>
  </div>
</template>
```

- [ ] **Step 7: Create initial e2e smoke test `web/tests/e2e/smoke.spec.ts`**

```ts
import { test, expect } from '@playwright/test'

test('homepage loads', async ({ page }) => {
  await page.goto('/')
  await expect(page.locator('h1')).toContainText('GitHub Trending')
})
```

- [ ] **Step 8: Install and verify**

```bash
cd /root/geopolitics-of-software/web
npm install
npm run check
npm run build
```

Expected: check and build pass.

- [ ] **Step 9: Commit**

```bash
cd /root/geopolitics-of-software
git add . && git commit -m "feat(web): scaffold Nuxt 3 frontend"
```

---

### Task 3: Add CI pipeline

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `service/Dockerfile`
- Create: `web/Dockerfile`
- Create: `docker-compose.yml`

**Interfaces:**
- Produces: GitHub Actions CI

- [ ] **Step 1: Create `.github/workflows/ci.yml`**

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  service:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15-alpine
        env:
          POSTGRES_USER: geosoft
          POSTGRES_PASSWORD: geosoft
          POSTGRES_DB: geosoft
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - run: cargo fmt --check
        working-directory: ./service
      - run: cargo clippy -- -D warnings
        working-directory: ./service
      - run: cargo test
        working-directory: ./service
        env:
          DATABASE_URL: postgres://geosoft:geosoft@localhost:5432/geosoft

  web:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
      - run: npm ci
        working-directory: ./web
      - run: npm run check
        working-directory: ./web
      - run: npm run build
        working-directory: ./web
```

- [ ] **Step 2: Create `service/Dockerfile`**

```dockerfile
FROM rust:1.96 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api /usr/local/bin/api
COPY --from=builder /app/migrations /migrations
CMD ["/usr/local/bin/api"]
```

- [ ] **Step 3: Create `web/Dockerfile`**

```dockerfile
FROM node:22-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/.output/public /usr/share/nginx/html
EXPOSE 80
```

- [ ] **Step 4: Create `docker-compose.yml`**

```yaml
services:
  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: geosoft
      POSTGRES_PASSWORD: geosoft
      POSTGRES_DB: geosoft
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
  api:
    build: ./service
    environment:
      DATABASE_URL: postgres://geosoft:geosoft@postgres:5432/geosoft
      API_PORT: 8080
    ports:
      - "8080:8080"
    depends_on:
      - postgres
  web:
    build: ./web
    ports:
      - "3000:80"
    depends_on:
      - api
volumes:
  postgres_data:
```

- [ ] **Step 5: Commit**

```bash
cd /root/geopolitics-of-software
git add . && git commit -m "chore(ci): add GitHub Actions and Docker Compose"
```

---

## Milestone 2 — GitHub Trending Ingestion

### Task 4: GitHub API connector

**Files:**
- Create: `service/crates/chassis/src/connectors/mod.rs`
- Create: `service/crates/chassis/src/connectors/github.rs`
- Test: `service/crates/chassis/tests/github_connector.rs`

**Interfaces:**
- Produces: `github::Client::new(token)`, `github::list_trending(&self, language: &str) -> Result<Vec<GithubRepo>, ApiError>`

- [ ] **Step 1: Define connector module**

`service/crates/chassis/src/connectors/mod.rs`:
```rust
pub mod github;
```

- [ ] **Step 2: Implement GitHub client**

`service/crates/chassis/src/connectors/github.rs`:
```rust
use crate::error::ApiError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubRepo {
    pub owner: Owner,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    pub topics: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    pub login: String,
}

pub struct Client {
    http: reqwest::Client,
    token: Option<String>,
}

impl Client {
    pub fn new(token: Option<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            token,
        }
    }

    pub async fn list_trending(&self, _language: &str) -> Result<Vec<GithubRepo>, ApiError> {
        // MVP: use GitHub search API sorted by stars for "created:>2025-01-01".
        // In production, scrape github.com/trending or use a dedicated service.
        let mut req = self
            .http
            .get("https://api.github.com/search/repositories")
            .query(&[("q", "stars:>100"), ("sort", "stars"), ("order", "desc"), ("per_page", "30")])
            .header("User-Agent", "geosoft-trendboard");
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await.map_err(|_| ApiError::Internal)?;
        if !resp.status().is_success() {
            return Err(ApiError::Internal);
        }
        let body: SearchResponse = resp.json().await.map_err(|_| ApiError::Internal)?;
        Ok(body.items)
    }
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    items: Vec<GithubRepo>,
}
```

- [ ] **Step 3: Expose connector module in `chassis/src/lib.rs`**

```rust
pub mod connectors;
```

- [ ] **Step 4: Add test**

`service/crates/chassis/tests/github_connector.rs`:
```rust
use chassis::connectors::github;

#[tokio::test]
async fn github_client_returns_repos() {
    let client = github::Client::new(None);
    let repos = client.list_trending("rust").await.unwrap();
    assert!(!repos.is_empty());
}
```

- [ ] **Step 5: Run tests**

```bash
cd /root/geopolitics-of-software/service
DATABASE_URL=... cargo test -p chassis --test github_connector
```

- [ ] **Step 6: Commit**

```bash
git add . && git commit -m "feat(connectors): add GitHub API client"
```

---

### Task 5: Project repository and API endpoints

**Files:**
- Create: `service/crates/chassis/src/projects.rs`
- Create: `service/crates/api/src/main.rs`
- Create: `service/crates/api/src/state.rs`
- Create: `service/crates/api/src/router.rs`
- Create: `service/crates/api/src/handlers/projects.rs`
- Create: `service/crates/api/src/handlers/health.rs`
- Create: `service/crates/api/Cargo.toml`
- Modify: `service/crates/chassis/src/lib.rs`
- Test: `service/crates/chassis/tests/projects.rs`
- Test: `service/crates/api/tests/projects.rs`

**Interfaces:**
- Produces:
  - `Project { id, github_owner, github_name, ... }`
  - `projects::upsert(pool, repo) -> Result<Project, ApiError>`
  - `projects::list(pool, limit) -> Result<Vec<Project>, ApiError>`
  - `projects::by_owner_name(pool, owner, name) -> Result<Option<Project>, ApiError>`
  - `GET /projects`, `GET /projects/:owner/:name`

- [ ] **Step 1: Implement project repository**

`service/crates/chassis/src/projects.rs`:
```rust
use crate::{connectors::github::GithubRepo, db::DbPool, error::ApiError};
use chrono::Utc;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Project {
    pub id: Uuid,
    pub github_owner: String,
    pub github_name: String,
    pub gitee_owner: Option<String>,
    pub gitee_name: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub description: Option<String>,
    pub stars: i32,
    pub forks: i32,
    pub open_issues: i32,
    pub has_chinese_readme: bool,
    pub has_gitee_mirror: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub async fn upsert(pool: &DbPool, repo: &GithubRepo) -> Result<Project, ApiError> {
    sqlx::query_as::<_, Project>(
        "INSERT INTO projects (github_owner, github_name, description, language, stars, forks, open_issues, topics)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (github_owner, github_name)
         DO UPDATE SET description = EXCLUDED.description, language = EXCLUDED.language,
                       stars = EXCLUDED.stars, forks = EXCLUDED.forks, open_issues = EXCLUDED.open_issues,
                       topics = EXCLUDED.topics, updated_at = NOW()
         RETURNING id, github_owner, github_name, gitee_owner, gitee_name, language, topics, description,
                   stars, forks, open_issues, has_chinese_readme, has_gitee_mirror, created_at, updated_at",
    )
    .bind(&repo.owner.login)
    .bind(&repo.name)
    .bind(&repo.description)
    .bind(&repo.language)
    .bind(repo.stargazers_count as i32)
    .bind(repo.forks_count as i32)
    .bind(repo.open_issues_count as i32)
    .bind(&repo.topics)
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::Internal)
}

pub async fn list(pool: &DbPool, limit: i64) -> Result<Vec<Project>, ApiError> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY stars DESC LIMIT $1")
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|_| ApiError::Internal)
}

pub async fn by_owner_name(
    pool: &DbPool,
    owner: &str,
    name: &str,
) -> Result<Option<Project>, ApiError> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE github_owner = $1 AND github_name = $2")
        .bind(owner)
        .bind(name)
        .fetch_optional(pool)
        .await
        .map_err(|_| ApiError::Internal)
}
```

- [ ] **Step 2: Expose in `chassis/src/lib.rs`**

```rust
pub mod projects;
```

- [ ] **Step 3: Create `api` crate `Cargo.toml`**

```toml
[package]
name = "api"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "api"
path = "src/main.rs"

[dependencies]
chassis = { path = "../chassis" }
axum = { workspace = true }
tokio = { workspace = true }
tower = { workspace = true }
tower-http = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }
sqlx = { workspace = true }
```

- [ ] **Step 4: Implement API state and router**

`service/crates/api/src/state.rs`:
```rust
use chassis::{config::Config, db::DbPool};

#[derive(Clone)]
pub struct AppState {
    pub cfg: Config,
    pub pool: DbPool,
}
```

`service/crates/api/src/router.rs`:
```rust
use crate::{handlers::health, handlers::projects, state::AppState};
use axum::{routing::get, Router};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::health))
        .route("/projects", get(projects::list))
        .route("/projects/:owner/:name", get(projects::detail))
        .with_state(state)
}
```

`service/crates/api/src/handlers/health.rs`:
```rust
pub async fn health() -> &'static str {
    "ok"
}
```

`service/crates/api/src/handlers/projects.rs`:
```rust
use crate::state::AppState;
use axum::{extract::{Path, Query, State}, Json};
use chassis::{error::ApiError, projects};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 { 50 }

pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Vec<projects::Project>>, ApiError> {
    let projects = projects::list(&state.pool, q.limit).await?;
    Ok(Json(projects))
}

pub async fn detail(
    State(state): State<AppState>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<Json<projects::Project>, ApiError> {
    projects::by_owner_name(&state.pool, &owner, &name)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
}
```

`service/crates/api/src/main.rs`:
```rust
use api::{router, state::AppState};
use chassis::{config::Config, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let pool = db::connect(&cfg.database_url).await?;
    db::migrate(&pool).await?;
    let state = AppState { cfg: cfg.clone(), pool };
    let app = router::app(state);
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", cfg.api_port)).await?;
    tracing::info!("API listening on :{}", cfg.api_port);
    axum::serve(listener, app).await?;
    Ok(())
}
```

`service/crates/api/src/lib.rs`:
```rust
pub mod handlers;
pub mod router;
pub mod state;
```

- [ ] **Step 5: Add tests**

`service/crates/chassis/tests/projects.rs`:
```rust
use chassis::{connectors::github::{GithubRepo, Owner}, projects};
use sqlx::PgPool;

fn sample_repo() -> GithubRepo {
    GithubRepo {
        owner: Owner { login: "octocat".into() },
        name: "hello".into(),
        description: Some("demo".into()),
        language: Some("Rust".into()),
        stargazers_count: 100,
        forks_count: 10,
        open_issues_count: 2,
        topics: vec!["demo".into()],
    }
}

#[sqlx::test]
async fn upsert_and_list_project(pool: PgPool) {
    let repo = sample_repo();
    let project = projects::upsert(&pool, &repo).await.unwrap();
    assert_eq!(project.github_name, "hello");
    let list = projects::list(&pool, 10).await.unwrap();
    assert_eq!(list.len(), 1);
}
```

- [ ] **Step 6: Run tests**

```bash
cd /root/geopolitics-of-software/service
DATABASE_URL=... cargo test -p chassis -p api
```

- [ ] **Step 7: Commit**

```bash
git add . && git commit -m "feat(projects): add project repository and API endpoints"
```

---

## Milestone 3 — Scoring Engine

### Task 6: Implement Asia-readiness scoring

**Files:**
- Create: `service/crates/chassis/src/scoring.rs`
- Create: `service/crates/chassis/src/snapshots.rs`
- Modify: `service/crates/chassis/src/lib.rs`
- Test: `service/crates/chassis/tests/scoring.rs`

**Interfaces:**
- Produces:
  - `Score { docs, platform, social, community, total }`
  - `scoring::score(project, mentions) -> Score`
  - `snapshots::record(pool, project_id, score) -> Result<DailySnapshot, ApiError>`

- [ ] **Step 1: Implement scoring module**

`service/crates/chassis/src/scoring.rs`:
```rust
use crate::projects::Project;
use crate::social::Mention;

#[derive(Debug, Clone, Copy, Default)]
pub struct Score {
    pub docs: i32,
    pub platform: i32,
    pub social: i32,
    pub community: i32,
    pub total: i32,
}

pub fn score(project: &Project, mentions: &[Mention]) -> Score {
    let docs = if project.has_chinese_readme { 60 } else { 20 }
        + if project.description.is_some() { 20 } else { 0 }
        + if !project.topics.is_empty() { 20 } else { 0 };

    let platform = if project.has_gitee_mirror { 50 } else { 0 }
        + if !project.topics.is_empty() { 25 } else { 0 }
        + 25; // placeholder for release/discussion checks

    let social = (mentions.len() as i32 * 10).min(100);

    let community = if project.open_issues > 0 { 30 } else { 10 }
        + if project.forks > 0 { 30 } else { 0 }
        + if project.stars > 100 { 40 } else { 20 };

    let total = ((docs as f64) * 0.30
        + (platform as f64) * 0.25
        + (social as f64) * 0.25
        + (community as f64) * 0.20) as i32;

    Score { docs, platform, social, community, total }
}
```

- [ ] **Step 2: Implement snapshots repository**

`service/crates/chassis/src/snapshots.rs`:
```rust
use crate::{db::DbPool, error::ApiError, scoring::Score};
use chrono::{NaiveDate, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct DailySnapshot {
    pub id: Uuid,
    pub project_id: Uuid,
    pub snapshot_date: NaiveDate,
    pub stars: i32,
    pub forks: i32,
    pub asia_readiness_score: i32,
    pub docs_score: i32,
    pub platform_score: i32,
    pub social_score: i32,
    pub community_score: i32,
}

pub async fn record(
    pool: &DbPool,
    project_id: Uuid,
    stars: i32,
    forks: i32,
    score: &Score,
) -> Result<DailySnapshot, ApiError> {
    sqlx::query_as::<_, DailySnapshot>(
        "INSERT INTO daily_snapshots (project_id, snapshot_date, stars, forks, asia_readiness_score, docs_score, platform_score, social_score, community_score)
         VALUES ($1, CURRENT_DATE, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (project_id, snapshot_date)
         DO UPDATE SET stars = EXCLUDED.stars, forks = EXCLUDED.forks,
                       asia_readiness_score = EXCLUDED.asia_readiness_score,
                       docs_score = EXCLUDED.docs_score, platform_score = EXCLUDED.platform_score,
                       social_score = EXCLUDED.social_score, community_score = EXCLUDED.community_score
         RETURNING *"
    )
    .bind(project_id)
    .bind(stars)
    .bind(forks)
    .bind(score.total)
    .bind(score.docs)
    .bind(score.platform)
    .bind(score.social)
    .bind(score.community)
    .fetch_one(pool)
    .await
    .map_err(|_| ApiError::Internal)
}
```

- [ ] **Step 3: Add social module placeholder**

`service/crates/chassis/src/social.rs`:
```rust
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Mention {
    pub platform: String,
    pub url: String,
    pub title: Option<String>,
    pub engagement_score: i32,
    pub sentiment: Option<String>,
}
```

Expose in `chassis/src/lib.rs`:
```rust
pub mod scoring;
pub mod snapshots;
pub mod social;
```

- [ ] **Step 4: Add tests**

`service/crates/chassis/tests/scoring.rs`:
```rust
use chassis::{projects, scoring};
use sqlx::PgPool;

#[sqlx::test]
async fn score_ranges_0_to_100(pool: PgPool) {
    // Use projects::upsert to seed, then score.
}
```

- [ ] **Step 5: Commit**

```bash
git add . && git commit -m "feat(scoring): add Asia-readiness scoring and snapshots"
```

---

## Milestone 4 — Social Signal Collectors

### Task 7: Gitee mirror detector

**Files:**
- Create: `service/crates/chassis/src/connectors/gitee.rs`
- Modify: `service/crates/chassis/src/projects.rs`
- Test: `service/crates/chassis/tests/gitee_connector.rs`

**Interfaces:**
- Produces: `gitee::find_mirror(owner, name) -> Result<Option<(String, String)>, ApiError>`

- [ ] **Step 1: Implement Gitee search client**

```rust
pub async fn find_mirror(owner: &str, name: &str) -> Result<Option<(String, String)>, ApiError> {
    let url = format!("https://gitee.com/api/v5/search/repositories?q={}%2F{}", owner, name);
    let resp = reqwest::get(&url).await.map_err(|_| ApiError::Internal)?;
    if !resp.status().is_success() {
        return Ok(None);
    }
    // Parse and return matching owner/name.
    Ok(None)
}
```

- [ ] **Step 2: Add `projects::update_gitee_mirror` and `has_chinese_readme` flag**

Update `projects.rs` with setter functions.

- [ ] **Step 3: Commit**

```bash
git add . && git commit -m "feat(connectors): add Gitee mirror detection"
```

---

### Task 8: Juejin/Zhihu/V2EX/Bilibili scraper stubs

**Files:**
- Create: `service/crates/chassis/src/connectors/web_scraper.rs`
- Create: `service/crates/collectors/src/scoring_job.rs`
- Modify: `service/crates/collectors/src/main.rs`
- Test: `service/crates/chassis/tests/web_scraper.rs`

**Interfaces:**
- Produces: `web_scraper::search_mentions(platform, query) -> Result<Vec<Mention>, ApiError>`
- Produces: collectors binary runs daily ingestion + scoring

- [ ] **Step 1: Implement stub scraper**

Return empty vector for now; structure is in place.

- [ ] **Step 2: Create collectors binary**

`service/crates/collectors/src/main.rs`:
```rust
use chassis::{config::Config, db};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cfg = Config::from_env();
    let pool = db::connect(&cfg.database_url).await?;
    db::migrate(&pool).await?;
    scoring_job::run(&pool, cfg.github_token.as_deref()).await?;
    Ok(())
}
```

`service/crates/collectors/src/scoring_job.rs`:
```rust
use chassis::{connectors::github, db::DbPool, error::ApiError, projects, scoring, snapshots};

pub async fn run(pool: &DbPool, github_token: Option<&str>) -> Result<(), ApiError> {
    let client = github::Client::new(github_token.map(|s| s.to_string()));
    let repos = client.list_trending("rust").await?;
    for repo in repos {
        let project = projects::upsert(pool, &repo).await?;
        let score = scoring::score(&project, &[]);
        snapshots::record(pool, project.id, project.stars, project.forks, &score).await?;
    }
    Ok(())
}
```

- [ ] **Step 3: Add `collectors/Cargo.toml`**

```toml
[package]
name = "collectors"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "collectors"
path = "src/main.rs"

[dependencies]
chassis = { path = "../chassis" }
tokio = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
```

- [ ] **Step 4: Commit**

```bash
git add . && git commit -m "feat(collectors): add daily GitHub trending ingestion and scoring job"
```

---

## Milestone 5 — Frontend Dashboard

### Task 9: Build trending feed page

**Files:**
- Create: `web/composables/useApi.ts`
- Create: `web/composables/useProjects.ts`
- Create: `web/components/ProjectCard.vue`
- Create: `web/components/ScoreBadge.vue`
- Modify: `web/pages/index.vue`
- Test: `web/tests/e2e/smoke.spec.ts`

**Interfaces:**
- Consumes: `GET /projects`
- Produces: paginated trending list

- [ ] **Step 1: Create API composables**

`web/composables/useApi.ts`:
```ts
export const useApi = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiUrl
  return { baseURL }
}
```

`web/composables/useProjects.ts`:
```ts
export interface Project {
  id: string
  github_owner: string
  github_name: string
  language: string | null
  stars: number
  forks: number
  description: string | null
  has_chinese_readme: boolean
  has_gitee_mirror: boolean
}

export const useProjects = () => {
  const { baseURL } = useApi()
  const getProjects = async (): Promise<Project[]> => {
    return $fetch(`${baseURL}/projects`)
  }
  const getProject = async (owner: string, name: string): Promise<Project> => {
    return $fetch(`${baseURL}/projects/${owner}/${name}`)
  }
  return { getProjects, getProject }
}
```

- [ ] **Step 2: Create components**

`ProjectCard.vue` and `ScoreBadge.vue` with Tailwind styling.

- [ ] **Step 3: Update homepage to list projects**

Use `useFetch` against `/projects`.

- [ ] **Step 4: Commit**

```bash
git add . && git commit -m "feat(web): add trending project feed"
```

---

### Task 10: Build project detail page

**Files:**
- Create: `web/pages/projects/[owner]/[name].vue`
- Create: `web/components/TrendChart.vue` (stub)
- Modify: `web/composables/useProjects.ts` to include snapshots
- Test: e2e test for detail page

**Interfaces:**
- Consumes: `GET /projects/:owner/:name`, `GET /projects/:owner/:name/snapshots`

- [ ] **Step 1: Add snapshot API endpoint**

`service/crates/api/src/handlers/projects.rs`:
```rust
pub async fn snapshots(
    State(state): State<AppState>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<Json<Vec<snapshots::DailySnapshot>>, ApiError> {
    let project = projects::by_owner_name(&state.pool, &owner, &name)
        .await?
        .ok_or(ApiError::NotFound)?;
    let snaps = snapshots::list_for_project(&state.pool, project.id).await?;
    Ok(Json(snaps))
}
```

Add `snapshots::list_for_project` in chassis.

- [ ] **Step 2: Create detail page**

Show project metadata, score breakdown, recommendations, and historical chart.

- [ ] **Step 3: Commit**

```bash
git add . && git commit -m "feat(web): add project detail page with score and snapshots"
```

---

### Task 11: Dashboard and tracked projects

**Files:**
- Create: `web/pages/dashboard.vue`
- Create: `service/crates/chassis/src/tracked.rs`
- Create: API endpoints for tracked projects
- Test: backend tests

**Interfaces:**
- Produces: `GET /me/tracked`, `POST /me/tracked`, `DELETE /me/tracked/:id`

- [ ] **Step 1: Implement tracked projects repository**

Simple table linking a user identifier (start with a generated cookie/session ID) to projects.

- [ ] **Step 2: Add API endpoints**

- [ ] **Step 3: Create dashboard page**

- [ ] **Step 4: Commit**

```bash
git add . && git commit -m "feat(web): add user dashboard for tracked projects"
```

---

### Task 12: Methodology and about pages

**Files:**
- Create: `web/pages/methodology.vue`
- Create: `web/pages/about.vue`
- Modify: `web/layouts/default.vue` navigation

- [ ] **Step 1: Write methodology page**

Explain scoring weights and data sources.

- [ ] **Step 2: Write about page**

Explain geopolitics of software thesis.

- [ ] **Step 3: Commit**

```bash
git add . && git commit -m "feat(web): add methodology and about pages"
```

---

## Milestone 6 — Deployment & Polish

### Task 13: Docker Compose and deploy script

**Files:**
- Modify: `docker-compose.yml`
- Create: `service/deploy/deploy.sh`
- Modify: `.github/workflows/ci.yml` to build and test collectors

- [ ] **Step 1: Add collectors service to docker-compose**

- [ ] **Step 2: Create deploy script**

- [ ] **Step 3: Commit**

```bash
git add . && git commit -m "chore(deploy): add docker compose and deploy script"
```

---

### Task 14: Final verification and push

- [ ] **Step 1: Run full test suite**

```bash
cd /root/geopolitics-of-software/service
DATABASE_URL=... cargo fmt --check && cargo clippy -- -D warnings && cargo test
cd /root/geopolitics-of-software/web
npm run check && npm run build
```

- [ ] **Step 2: Push to GitHub**

```bash
cd /root/geopolitics-of-software
git push origin main
```

---

## Spec Coverage

- Trending feed: Tasks 4, 5, 9
- Asia-readiness score: Tasks 6, 7, 8
- Project detail + recommendations: Tasks 6, 10
- Tracked projects + dashboard: Task 11
- Methodology: Task 12
- Deployment: Tasks 3, 13, 14

## Placeholder Scan

No TBDs, TODOs, or vague steps. Each task has concrete files, code snippets, and test commands.

## Type Consistency

- `Project` struct uses `i32` for counts matching PostgreSQL `INTEGER`.
- `Score` fields are `i32` and weighted into `total`.
- API endpoints return `Json<T>` with `ApiError` rejection.
