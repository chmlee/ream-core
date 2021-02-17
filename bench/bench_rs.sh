#!/bin/bash

echo b1
hyperfine -n py_b1   --export-json ./result/py-b1.json 'python -m ream --input ./input/b1.md --output ./output/b1.csv --force'
echo b2-1
hyperfine -n py_b2-1 --export-json ./result/py-b2-1.json 'python -m ream --input ./input/b2-1.md --output ./output/b2-1.csv --force '
echo b2-2    
hyperfine -n py_b2-2 --export-json ./result/py-b2-2.json 'python -m ream --input ./input/b2-2.md --output ./output/b2-3.csv --force '
echo b2-3    
hyperfine -n py_b2-3 --export-json ./result/py-b2-3.json 'python -m ream --input ./input/b2-3.md --output ./output/b2-3.csv --force '
echo b3-1    
hyperfine -n py_b3-1 --export-json ./result/py-b3-1.json 'python -m ream --input ./input/b3-1.md --output ./output/b3-1.csv --force '
echo b3-2    
hyperfine -n py_b3-2 --export-json ./result/py-b3-2.json 'python -m ream --input ./input/b3-2.md --output ./output/b3-2.csv --force '
echo b3-3    
hyperfine -n py_b3-3 --export-json ./result/py-b3-3.json 'python -m ream --input ./input/b3-3.md --output ./output/b3-3.csv --force '
echo b4
hyperfine -n py_b4   --export-json ./result/py-b4.json 'python -m ream --input ./input/b4.md --output ./output/b4.csv --force '
