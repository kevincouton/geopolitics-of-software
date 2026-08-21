export interface Project {
  id: string
  github_owner: string
  github_name: string
  gitee_owner: string | null
  gitee_name: string | null
  language: string | null
  topics: string[]
  description: string | null
  stars: number
  forks: number
  open_issues: number
  has_chinese_readme: boolean
  has_gitee_mirror: boolean
  score: number | null
  created_at: string
  updated_at: string
}

export interface Snapshot {
  id: string
  project_id: string
  snapshot_date: string
  stars: number
  forks: number
  asia_readiness_score: number
  docs_score: number
  platform_score: number
  social_score: number
  community_score: number
}

export interface ProjectsQuery {
  limit?: number
  offset?: number
}

export interface TrackedProject extends Project {
  tracked_at: string
}

export const useProjects = () => {
  const { baseURL } = useApi()
  const getProjects = async (query: ProjectsQuery = {}): Promise<Project[]> => {
    return $fetch(`${baseURL}/projects`, { query })
  }
  const getProject = async (owner: string, name: string): Promise<Project> => {
    return $fetch(`${baseURL}/projects/${owner}/${name}`)
  }
  const getProjectSnapshots = async (
    owner: string,
    name: string,
  ): Promise<Snapshot[]> => {
    return $fetch(`${baseURL}/projects/${owner}/${name}/snapshots`)
  }
  const getTrackedProjects = async (): Promise<TrackedProject[]> => {
    return $fetch(`${baseURL}/me/tracked`, { credentials: 'include' })
  }
  const trackProject = async (projectId: string): Promise<TrackedProject> => {
    return $fetch(`${baseURL}/me/tracked`, {
      method: 'POST',
      body: { project_id: projectId },
      credentials: 'include',
    })
  }
  const untrackProject = async (projectId: string): Promise<void> => {
    await $fetch(`${baseURL}/me/tracked/${projectId}`, {
      method: 'DELETE',
      credentials: 'include',
    })
  }
  return {
    getProjects,
    getProject,
    getProjectSnapshots,
    getTrackedProjects,
    trackProject,
    untrackProject,
  }
}
