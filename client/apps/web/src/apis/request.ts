/* eslint-disable @typescript-eslint/no-explicit-any */
import axios from 'axios'

export default function request(config: any) {
    const instance = axios.create({
        baseURL: config.baseUrl ? config.baseUrl : 'http://localhost:3000',
        timeout: 5000
    })

    return instance(config)
}