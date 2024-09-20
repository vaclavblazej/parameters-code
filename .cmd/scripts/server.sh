#!/usr/bin/env bash

#description: Run local development server

cd "$(dirname "$0")/../../web" || exit

echo "Hugo server start"
hugo server --disableFastRender --buildFuture --buildDrafts --gc -s "." --port=1313 --bind="127.0.0.1" --watch
