// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  // Default GitHub Pages project hosting: coderturtle.github.io/borrow-native/.
  // No custom domain/DNS for this workshop (unlike terminal-velocity's later
  // terminal-velocity.coderturtle.io cutover, which required a real Route53
  // change via agentic-infra-lab) - out of scope for this scaffolding pass.
  // Every internal link MUST be base-aware (import.meta.env.BASE_URL), not a
  // bare "/path" - the single most common Pages-project breakage.
  site: "https://coderturtle.github.io",
  base: "/borrow-native/",
  integrations: [mdx(), tailwind()],
  output: "static",
});
