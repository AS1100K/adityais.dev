/// <reference path="../.astro/types.d.ts" />

interface ImportMetaEnv {
    readonly GITHUB_API_KEY: string;
}

interface ImportMeta {
    readonly env: ImportMetaEnv;
}