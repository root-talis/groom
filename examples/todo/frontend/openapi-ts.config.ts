import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  input: '../spec.yaml',
  output: {
    path: 'src/api',
  },
  plugins: ['@hey-api/client-axios'],
  types: {
    enums: 'javascript',
  },
});
