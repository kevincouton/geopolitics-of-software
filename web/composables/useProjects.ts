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
