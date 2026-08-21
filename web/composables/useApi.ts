export const useApi = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiUrl
  return { baseURL }
}
