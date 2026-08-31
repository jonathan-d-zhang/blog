// @ts-check

import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import { defineConfig, fontProviders } from 'astro/config';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';

// https://astro.build/config
export default defineConfig({
	site: 'https://jonathan-d-zhang.github.io',
	base: '/blog',
	integrations: [mdx(), sitemap()],
	markdown: {
		remarkPlugins: [remarkMath],
		rehypePlugins: [rehypeKatex],
	},
	fonts: [
		{
			provider: fontProviders.local(),
			name: 'Berkeley Mono',
			cssVariable: '--font-berkeley',
			fallbacks: ['ui-monospace', 'SFMono-Regular', 'Menlo', 'monospace'],
			options: {
				variants: [
					{
						src: ['./src/assets/fonts/BerkeleyMonoNerdFont-Regular.woff2'],
						weight: 400,
						style: 'normal',
						display: 'swap',
					},
					{
						src: ['./src/assets/fonts/BerkeleyMonoNerdFont-Bold.woff2'],
						weight: 700,
						style: 'normal',
						display: 'swap',
					},
					{
						src: ['./src/assets/fonts/BerkeleyMonoNerdFont-Italic.woff2'],
						weight: 400,
						style: 'italic',
						display: 'swap',
					},
					{
						src: ['./src/assets/fonts/BerkeleyMonoNerdFont-BoldItalic.woff2'],
						weight: 700,
						style: 'italic',
						display: 'swap',
					},
				],
			},
		},
	],
});
