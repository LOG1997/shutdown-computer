import axios from 'axios'
import type { AxiosRequestConfig, InternalAxiosRequestConfig, AxiosResponse } from 'axios'

// 定义扩展配置接口
interface CustomRequestConfig extends AxiosRequestConfig {
    baseUrl?: string
}

// 创建 axios 实例
const service = axios.create({
    timeout: 5000,
    // 这里不设置 baseURL，因为每个请求可能不同，或者在拦截器中处理
})

// 添加请求拦截器
service.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
        // 1. 强制设置 Content-Type
        // 注意：如果是 FormData，不要设置 Content-Type，让浏览器自动设置 boundary
        if (!(config.data instanceof FormData)) {
            config.headers['Content-Type'] = 'application/json'
        }

        // 2. 处理自定义的 baseUrl
        // 如果配置中有 baseUrl，则覆盖实例的 baseURL 或拼接 URL
        // 注意：axios 优先使用 config.baseURL，如果 config 中没有，则使用实例创建时的 baseURL
        // 这里我们假设 custom config 中的 baseUrl 优先级最高
        const customBaseUrl = (config as any).baseUrl
        if (customBaseUrl) {
            // 如果 url 是相对路径，则拼接 baseUrl
            if (config.url && !config.url.startsWith('http')) {
                // 确保 baseUrl 不以 / 结尾，url 以 / 开头，避免双斜杠或漏斜杠
                const base = customBaseUrl.replace(/\/$/, '')
                const path = config.url.startsWith('/') ? config.url : `/${config.url}`
                config.url = `${base}${path}`
            } else if (config.url) {
                // 如果 url 已经是绝对路径，baseUrl 可能被忽略，或者你需要替换域名
                // 这里简单处理：如果 url 是绝对路径，通常不需要拼接，除非你要代理
            }
            // 清理掉自定义属性，避免 axios 报错未知属性
            delete (config as any).baseUrl
        }

        return config
    },
    (error) => {
        return Promise.reject(error)
    }
)

// 添加响应拦截器（可选，用于统一处理错误）
service.interceptors.response.use(
    (response: AxiosResponse) => {
        return response.data
    },
    (error) => {
        console.error('Request error:', error.message)
        return Promise.reject(error)
    }
)

// 导出请求函数
export default function request<T = any>(config: CustomRequestConfig): Promise<T> {
    return service.request(config)
}