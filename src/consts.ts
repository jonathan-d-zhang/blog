// Place any global data in this file.
// You can import this data from anywhere in your site by using the `import` keyword.

export const SITE_TITLE = 'Astro Blog';
export const SITE_DESCRIPTION = 'Welcome to my website!';

// Prefix a site-root path with the deploy base (BASE_URL has no trailing slash).
export const url = (p: string) => import.meta.env.BASE_URL.replace(/\/$/, '') + '/' + String(p).replace(/^\//, '');
