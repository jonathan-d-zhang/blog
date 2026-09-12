#!/bin/sh
# PostToolUse: lint the file that was just edited.
f=$(sed -n 's/.*"file_path"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p')
cd "$CLAUDE_PROJECT_DIR" || exit 0
case "$f" in
  *.github/workflows/*.y*ml) uvx zizmor@latest "$f" ;;
  *src/*.astro|*src/*.ts|*src/*.mdx|*astro.config.mjs)
    export PATH="$HOME/.nvm/versions/node/v22.23.2/bin:$PATH"
    node ./node_modules/astro/bin/astro.mjs check ;;
esac
exit $(( $? ? 2 : 0 ))  # 2 = feed output back to Claude
