import { defineConfig } from '@hey-api/openapi-ts'

export default defineConfig({
  input: 'http://0.0.0.0:3000/docs/private/api.json', // sign up at app.heyapi.dev
  output: 'src/client',
  plugins: ['@tanstack/react-query'],
})
