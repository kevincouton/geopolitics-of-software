CREATE TABLE tracked_projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id TEXT NOT NULL,
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, project_id)
);

CREATE INDEX idx_tracked_projects_user ON tracked_projects(user_id);
CREATE INDEX idx_tracked_projects_project ON tracked_projects(project_id);
