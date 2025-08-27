import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const blog_schema = z.object({
  title: z.string(),
  description: z.optional(z.string()),
  pubDate: z.date(),
  v1Data: z.optional(
    z.object({
      devtoUrl: z.string(),
      readingTimeMinutes: z.number(),
      coverImage: z.optional(z.string()),
    }),
  ),
});

const blog = defineCollection({
  loader: glob({ pattern: "**/*.{md, mdx}", base: "./src/content/blog" }),
  schema: blog_schema,
});

const draft_blog = defineCollection({
  loader: glob({
    pattern: "**/*.{md, mdx}",
    base: "./src/content/draft",
  }),
  schema: blog_schema,
});

export const collections = { blog, draft_blog };
