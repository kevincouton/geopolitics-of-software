# GeoSoft TrendBoard

> **Geopolitics of Software** — a dashboard that tracks GitHub trending projects and scores their readiness for China and Asia.

## What it does

GeoSoft TrendBoard surfaces open-source projects that are trending on GitHub and scores how well they are positioned for adoption across Chinese and Asian developer ecosystems.

It answers questions like:

- Which trending repos already have Chinese READMEs or Gitee mirrors?
- What is the Asia-readiness score of my project?
- What are the top 3 actions I should take to improve positioning in China?
- How does my project compare to competitors on Juejin, Zhihu, and GitHub?

## Stack

- **Backend:** Rust (Axum) + PostgreSQL
- **Frontend:** Nuxt 3 + Tailwind CSS
- **Data sources:** GitHub API, Gitee API, Juejin, Zhihu, V2EX, Bilibili (selective scraping/APIs)
- **Deployment:** Docker Compose + GitHub Actions

## Status

Design spec is in `docs/specs/2026-08-21-geopolitics-of-software-design.md`. Implementation plan will follow.
