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
