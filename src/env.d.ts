/// <reference path="../.astro/types.d.ts" />

type Env = {
  GITHUB_API_KEY: string;
};

type Runtime = import("@astrojs/cloudflare").Runtime<Env>;

declare namespace App {
  interface Locals extends Runtime {}
}
