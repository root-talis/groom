import { defineConfig } from 'orval'

export default defineConfig({
  todo: {
    input: {
      target: '../spec.yaml',
    },
    output: {
      mode: 'split',
      target: './src/api/generated/endpoints',
      schemas: './src/api/generated/models',
      client: 'axios-functions',
      httpClient: 'axios',
      clean: true,
      prettier: true,
      override: {
        mutator: {
          path: './src/api/mutator.ts',
          name: 'customInstance',
        },
      },
    },
  },
})
