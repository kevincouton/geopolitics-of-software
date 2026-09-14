# Security Policy

GeoSoft TrendBoard is a dashboard that tracks GitHub trending projects and
scores their readiness for China and Asia (Rust/Axum + PostgreSQL backend,
Nuxt 3 frontend, deployed via Docker Compose).

## Reporting a Vulnerability

Please report vulnerabilities privately through GitHub:

- Use **private vulnerability reporting** on this repository
  (Security tab → "Report a vulnerability"), or
- Open a **private security advisory** draft and invite the maintainer.

Do **not** open a public issue for a vulnerability.

## Scope

In scope:

- The Rust backend under `service/` (API, collectors, chassis), including
  SQL injection, authentication/authorization flaws, and unsafe handling of
  scraped data.
- The Nuxt frontend under `web/`, including XSS and CSRF.
- CI/CD configuration under `.github/` (e.g. secret exposure, workflow
  injection).
- Deployment configuration (`docker-compose.yml`, Dockerfiles, nginx.conf).

Out of scope: vulnerabilities in third-party dependencies that have a public
advisory and no GeoSoft-specific exploit path (report those upstream); issues
requiring physical access to the deployment host.

## Response Expectations

This is a solo-maintainer project worked on part-time. You can expect:

- Acknowledgement within **7 days**.
- An assessment and remediation plan within **30 days** for confirmed issues.

Fixes land on `main` once ready; there is no release-backport process.
