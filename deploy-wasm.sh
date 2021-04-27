#!/bin/bash

wasm-pack build --target web

cd pkg

git init
git add -A
git commit -m "wasm"
git push -f git@github.com:chmlee/ream-wasm.git master

cd -
