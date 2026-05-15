import axios from 'axios'

const client = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  timeout: import.meta.env.VITE_API_TIMEOUT ? parseInt(import.meta.env.VITE_API_TIMEOUT) : 1000
})

console.log('API base URL:', client.defaults.baseURL);

export default client
