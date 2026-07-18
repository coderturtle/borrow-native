// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  // Custom domain (borrow-native.coderturtle.io) via GitHub Pages + Route53 CNAME,
  // see .hekton/project.yaml's `deployment` block and agentic-infra-lab's
  // patterns/github-pages-dns. Site now serves at the domain root, not under
  // /borrow-native/ on coderturtle.github.io. Every internal link MUST still be
  // base-aware (import.meta.env.BASE_URL), not a bare "/path".
  site: "https://borrow-native.coderturtle.io",
  base: "/",
  integrations: [mdx(), tailwind()],
  output: "static",
});
