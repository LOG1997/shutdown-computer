/* eslint-disable @typescript-eslint/no-explicit-any */
import request from './request'

export const getDeviceStatus = ({ data, config }: { data: any, config: any }) => {
    return request({
        baseUrl: config ? config.baseUrl : undefined,
        url: '/api/device/status',
        method: 'post',
        data
    })
}

export const sendShutDownCommand = ({ data, config }: { data: any, config: any }) => {
    return request({
        baseUrl: config ? config.baseUrl : undefined,
        url: '/api/device/shutdown',
        method: 'post',
        data
    })
}