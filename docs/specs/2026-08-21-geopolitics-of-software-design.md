# GeoSoft TrendBoard — Design Spec

> **Date:** 2026-08-21  
> **Status:** Draft — ready for review  
> **Stack:** Rust (Axum) + PostgreSQL + Nuxt 3 + Tailwind CSS

## 1. Problem

Open-source projects are global by default, but distribution is local. A project can trend on GitHub and still miss the Chinese/Asian market because of language, platform, payment, and compliance gaps.

Maintainers do not have a simple way to answer:

- Is my project discoverable in China?
- What are my competitors doing on Juejin / Zhihu / Gitee?
- Which 2–3 positioning changes would have the biggest impact?

## 2. Product

**GeoSoft TrendBoard** is a dashboard that tracks GitHub trending projects and scores their Asia readiness.

### Core features

1. **Trending feed**
   - Daily import of GitHub Trending repos.
   - Filters: language, region (China/India/Japan/Korea/SEA/Global), Asia-readiness score range.

2. **Asia-readiness score (0–100)**
   - **Documentation (30):** English README quality, Chinese README presence, architecture docs.
   - **Platform presence (25):** Gitee mirror, GitHub topics, release notes, discussions enabled.
   - **Social signals (25):** Mentions/engagement on Juejin, Zhihu, V2EX, Bilibili.
   - **Community health (20):** Issue response time, contributor count, license, security policy.

3. **Project detail page**
   - Score breakdown with traffic-light indicators.
   - Actionable recommendations: “Add Chinese README,” “Mirror to Gitee,” “Post demo on Bilibili,” etc.
   - Historical score chart.

4. **Tracked projects**
   - Users can add their own repos.
   - Daily recalculation of scores.
   - Competitor comparison table.

5. **Reports (MVP-later)**
   - Weekly email/Slack report of score changes and new competitors.

## 3. Actors

- **Open-source maintainer:** wants to grow stars and adoption in Asia.
- **Developer relations / growth lead:** tracks portfolio projects and competitors.
- **Platform admin (us):** manages data ingestion, scoring weights, featured projects.

## 4. Architecture

```
┌─────────────────┐      ┌──────────────────┐      ┌─────────────────┐
│   Nuxt 3 Web    │──────▶   Rust API       │──────▶   PostgreSQL    │
│  (Dashboard)    │      │   (Axum)         │      │   (analytics)   │
└─────────────────┘      └──────────────────┘      └─────────────────┘
                                │
                                ▼
                    ┌──────────────────────┐
                    │  Collectors (Rust)   │
                    │  - GitHub API        │
                    │  - Gitee API         │
                    │  - Juejin/Zhihu/V2EX │
                    │  - Bilibili          │
                    └──────────────────────┘
```

## 5. Data model

### `projects`
- `id` (UUID)
- `github_owner`, `github_name`
- `gitee_owner`, `gitee_name` (nullable)
- `language`, `topics`, `description`
- `stars`, `forks`, `open_issues`
- `has_chinese_readme` (bool)
- `has_gitee_mirror` (bool)
- `created_at`, `updated_at`

### `daily_snapshots`
- `id`, `project_id`, `snapshot_date`
- `stars`, `forks`, `asia_readiness_score`
- Component scores (docs, platform, social, community)

### `social_mentions`
- `id`, `project_id`, `platform` (juejin/zhihu/v2ex/bilibili)
- `url`, `title`, `engagement_score`, `sentiment`
- `mentioned_at`

### `tracked_projects`
- `id`, `user_id`, `project_id`
- `notify_email`, `created_at`

## 6. Scoring algorithm (MVP)

Simple weighted sum, recalculated daily.

```
asia_readiness_score =
  docs_score * 0.30 +
  platform_score * 0.25 +
  social_score * 0.25 +
  community_score * 0.20
```

Each sub-score is normalized 0–100.

## 7. Pages

- `/` — Trending feed with filters and leaderboard.
- `/projects/[owner]/[name]` — Project detail + score + recommendations.
- `/dashboard` — User’s tracked projects and competitor comparisons.
- `/methodology` — How scoring works.
- `/about` — Geopolitics of Software thesis.

## 8. MVP scope (5 weeks)

1. **Week 1:** Scaffold Rust API + Nuxt frontend + PostgreSQL.
2. **Week 2:** GitHub trending collector + project import.
3. **Week 3:** Scoring engine + project detail page.
4. **Week 4:** Social signal collectors (Juejin/Zhihu/Gitee).
5. **Week 5:** Dashboard, tracked projects, reports, polish.

## 9. Out of scope

- Real-time data (daily batch is enough).
- Paid subscriptions (stubbed for MVP).
- Automated posting to platforms.
- Full multi-language support beyond English + Chinese.

## 10. Commercialization

- **Freemium:** public trending feed is free; tracked projects and reports require account.
- **Paid tier:** more tracked projects, competitor alerts, API access.
- **Enterprise:** white-label positioning reports for devrel teams.

## 11. Risks

- GitHub API rate limits.
- Scraping Chinese platforms may break; prefer official APIs.
- Score algorithm can feel arbitrary; publish methodology and iterate publicly.

## 12. Next step

Review this spec. If approved, write the implementation plan and start Week 1.
