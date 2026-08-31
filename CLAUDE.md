# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Personal blog built on the Astro blog starter, deployed to GitHub Pages at
`https://jonathan-d-zhang.github.io/blog`. Posts are Markdown/MDX with LaTeX support.

## Node version

Astro 7 requires node `>=22.12.0`. The machine default is 22.11.0, and `npm run build`
picks up the default even after `nvm use` — it fails with
`Node.js v22.11.0 is not supported by Astro!`. Either run `nvm alias default 22.23.2`
once, or invoke the binary directly:

```sh
export PATH="$HOME/.nvm/versions/node/v22.23.2/bin:$PATH"
node ./node_modules/astro/bin/astro.mjs build
```

If a build dies with `Cannot find native binding` / `@rolldown/binding-*`, the install is
half-written (npm optional-deps bug). Fix: `rm -rf node_modules package-lock.json && npm install`.

## Commands

```sh
npm run dev        # dev server on localhost:4321
npm run build      # -> ./dist
npm run preview    # serve ./dist
uvx zizmor@latest .github/workflows/deploy.yml   # lint the deploy workflow
```

There is no test suite and no typechecker installed (`astro check` would prompt to
add `@astrojs/check`). `src/content/blog/math-test.md` is the regression check for LaTeX:
after a build, `dist/blog/math-test/index.html` must contain 3 `class="katex"` spans.

## Base path — the main trap

`astro.config.mjs` sets `base: '/blog'` because the site is a GitHub *project* page.
`import.meta.env.BASE_URL` is `/blog` with **no trailing slash**, so concatenating
directly yields `/blogfavicon.svg`.

Never write a root-absolute internal link. Route every one through the `url()` helper in
`src/consts.ts`, which normalizes both sides of the join:

```astro
import { url } from '../consts';
<a href={url(`blog/${post.id}/`)}>
```

`HeaderLink.astro` applies `url()` itself, so pass it plain paths (`href="/blog"`) —
its active-state comparison strips `BASE_URL` from the current pathname and relies on
the un-prefixed form.

Note the doubled segment in built URLs is correct: base `/blog` + route `/blog/` gives
`/blog/blog/first-post/`.

## Content

Posts live in `src/content/blog/*.{md,mdx}`. Frontmatter is schema-validated in
`src/content.config.ts` — `title`, `description`, `pubDate` required; `updatedDate`,
`heroImage` optional. A frontmatter mismatch fails the build.

LaTeX comes from `remark-math` + `rehype-katex`, wired in `astro.config.mjs` under
`markdown`. KaTeX CSS is pulled in by the bare `@import "katex/dist/katex.min.css"` at the
top of `src/styles/global.css`. Write `$inline$` and `$$display$$`. Custom macros go in
`rehypeKatex({ macros: {...} })`.

## Fonts

Body font is Berkeley Mono Nerd Font, declared via Astro's local font provider in
`astro.config.mjs` (cssVariable `--font-berkeley`, consumed by `body` in
`src/styles/global.css`, preloaded by `<Font>` in `BaseHead.astro`).

Sources are `.otf` in `~/BerkeleyMonoNerdFont/`; only the converted `.woff2` files live in
`src/assets/fonts/`. To re-convert or add a style, subset with fonttools — no local tooling
is installed or needed:

```sh
uvx --from "fonttools[woff]" pyftsubset in.otf \
  --output-file=src/assets/fonts/out.woff2 --flavor=woff2 \
  --layout-features='*' --no-hinting \
  --unicodes='U+0000-00FF,U+2000-206F,U+2190-21FF,U+2200-22FF,U+25A0-25FF'
```

The subset range is deliberate. A straight woff2 compress of the Nerd Font is **1.4 MB per
style** (5.6 MB for four); the latin range above is 22 KB per style. The dropped weight is
almost entirely Nerd/Font-Awesome icon glyphs (`U+E000-E0C8`, `U+F000-F2FF`). If a post ever
needs one of those, widen `--unicodes` rather than shipping the unsubsetted font.

## Deploy

`.github/workflows/deploy.yml` builds with `withastro/action` on push to `main`, then
`actions/deploy-pages`. Repo Settings → Pages → Source must be **GitHub Actions**.

The workflow is kept zizmor-clean: actions are SHA-pinned with the tag in a trailing
comment, top-level `permissions: {}` with per-job grants, and `persist-credentials: false`
on checkout. When bumping an action, resolve the tag to a commit SHA (dereferencing
annotated tags) rather than writing the tag ref, and re-run zizmor.

## Still template defaults

`SITE_TITLE`/`SITE_DESCRIPTION` in `src/consts.ts`, the Astro-owned social links in
`Header.astro`, the sample posts in `src/content/blog/`, and `README.md` are all
unmodified starter content.

## Version control

This repo uses **jj** (`.jj/`), not git — there is no `.git` directory and no remote
configured yet. Git commands will fail.
