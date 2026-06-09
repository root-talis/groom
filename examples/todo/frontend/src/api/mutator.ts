import type { AxiosError, AxiosRequestConfig } from 'axios'
import axios from '@/services/axios'

export type ApiResponse<T> = {
  status: number
  data: T | undefined
  error?: unknown
}

export const customInstance = <T>(
  config: AxiosRequestConfig,
  options?: AxiosRequestConfig,
): Promise<ApiResponse<T>> => {
  return axios({
    ...config,
    ...options,
    validateStatus: () => true,
  }).then((response) => {
    const isSuccess = response.status >= 200 && response.status < 300

    return {
      status: response.status,
      data: response.data,
      error: isSuccess ? undefined : response.data,
    }
  })
}

export type ErrorType<Error> = AxiosError<Error>
export type BodyType<BodyData> = BodyData
