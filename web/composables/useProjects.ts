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
  return { getProjects, getProject }
}
