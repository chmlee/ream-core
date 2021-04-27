#!/bin/bash

wasm-pack build --target web

cd pkg

rm .gitignore

git init
git add -A
git commit
git push git@github.com:chmlee/ream-wasm.git master

cd -
