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
  score: number | null
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
  return { getProjects, getProject, getProjectSnapshots }
}
