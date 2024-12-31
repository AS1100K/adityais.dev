import { glob } from "astro/loaders";
import { defineCollection, z } from "astro:content";

const blog = defineCollection({
    loader: glob({ pattern: "**/*.{md, mdx}", base: "./src/content/blog" }),
    schema: z.object({
        title: z.string(),
        description: z.optional(z.string()),
    })
});

export const collections = { blog };