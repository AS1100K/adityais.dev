/**
 * Rehype plugin to add utm_source=adityais.dev to external links
 * if they don't already have a utm_source parameter
 */
import { visit } from "unist-util-visit";

export function rehypeUtm() {
  return (tree) => {
    visit(tree, "element", (node) => {
      if (node.tagName === "a" && node.properties && node.properties.href) {
        const href = node.properties.href;

        // Skip internal links, fragments, and mailto/tel links
        if (
          href.startsWith("/") ||
          href.startsWith("#") ||
          href.startsWith("mailto:") ||
          href.startsWith("tel:") ||
          href.startsWith("javascript:")
        ) {
          return;
        }

        try {
          const url = new URL(href);

          // Skip if it's the same domain
          if (
            url.hostname === "adityais.dev" ||
            url.hostname === "www.adityais.dev"
          ) {
            return;
          }

          // Check if utm_source already exists
          if (url.searchParams.has("utm_source")) {
            return;
          }

          // Add utm_source parameter
          url.searchParams.set("utm_source", "adityais.dev");
          node.properties.href = url.toString();
        } catch (error) {
          // Skip malformed URLs
          console.warn(`Failed to process URL: ${href}`, error);
        }
      }
    });
  };
}
