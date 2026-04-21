import path from "path"
import tailwindcss from "@tailwindcss/vite"
import viteReact from '@vitejs/plugin-react'
import { defineConfig } from "vite"
import { tanstackRouter } from '@tanstack/router-plugin/vite'
// https://vite.dev/config/
export default defineConfig({
    base: './',
    plugins: [
        tailwindcss(),
        tanstackRouter({
            target: 'react',
            autoCodeSplitting: true,
        }),
        viteReact(),
    ],
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./src"),
        },
    },
    build: {
        outDir: 'dist',
        assetsDir: 'assets',
    },
})
