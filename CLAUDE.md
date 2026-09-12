# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Personal blog built on the Astro blog starter, deployed to GitHub Pages at
`https://jonathan-d-zhang.github.io/blog`. Posts are Markdown/MDX with LaTeX support.

## Commands

```sh
npm run dev        # dev server on localhost:4321
npm run build      # -> ./dist
npm run preview    # serve ./dist
uvx zizmor@latest .github/workflows/deploy.yml   # lint the deploy workflow
```

There is no test suite and no typechecker installed (`astro check` would prompt to
add `@astrojs/check`).

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

This repo uses **jj** (`.jj/`), not git — there is no `.git` directory: Git commands will fail.
