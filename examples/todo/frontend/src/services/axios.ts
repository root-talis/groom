import axios from 'axios'
import qs from 'qs';

const client = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  timeout: import.meta.env.VITE_API_TIMEOUT ? parseInt(import.meta.env.VITE_API_TIMEOUT) : 1000,
  paramsSerializer: {
    serialize: (params) => {
      console.log('!!', params, qs.stringify(params, { arrayFormat: 'brackets' }))
      return qs.stringify(params, { arrayFormat: 'brackets' })
    }
  }
})

console.log('API base URL:', client.defaults.baseURL);

export default client
