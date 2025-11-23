import { defineConfig } from 'orval';

export default defineConfig({
    auth: {
        input: {
            target: 'https://auth.thmsn.local/openapi',
        },
        output: {
            target: './src/oai/auth/api.ts',
            client: 'fetch',
            mode: 'tags-split',
            schemas: './src/oai/auth/models',
        },
    },
    manage: {
        input: {
            target: 'https://manage.auth.thmsn.local/openapi',
        },
        output: {
            target: './src/oai/manage/api.ts',
            client: 'fetch',
            mode: 'tags-split',
            schemas: './src/oai/manage/models',
        },
    },
});