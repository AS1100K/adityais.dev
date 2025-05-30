import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const blog = defineCollection({
  loader: glob({ pattern: "**/*.{md, mdx}", base: "./src/content/blog" }),
  schema: z.object({
    title: z.string(),
    description: z.optional(z.string()),
    pubDate: z.date(),
  }),
});

export const collections = { blog };
