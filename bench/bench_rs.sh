#!/bin/bash

echo b1
hyperfine -n rs_b1   --export-json ./result/rs-b1.json   '../target/release/ream-core --input ./input/b1.md    >> /dev/null '
echo b2-1
hyperfine -n rs_b2-1 --export-json ./result/rs-b2-1.json '../target/release/ream-core --input ./input/b2-1.md  >> /dev/null'
echo b2-2    
hyperfine -n rs_b2-2 --export-json ./result/rs-b2-2.json '../target/release/ream-core --input ./input/b2-2.md  >> /dev/null'
echo b2-3    
hyperfine -n rs_b2-3 --export-json ./result/rs-b2-3.json '../target/release/ream-core --input ./input/b2-3.md  >> /dev/null'
echo b3-1    
hyperfine -n rs_b3-1 --export-json ./result/rs-b3-1.json '../target/release/ream-core --input ./input/b3-1.md  >> /dev/null'
echo b3-2    
hyperfine -n rs_b3-2 --export-json ./result/rs-b3-2.json '../target/release/ream-core --input ./input/b3-2.md  >> /dev/null'
echo b3-3    
hyperfine -n rs_b3-3 --export-json ./result/rs-b3-3.json '../target/release/ream-core --input ./input/b3-3.md  >> /dev/null'
echo b4
hyperfine -n rs_b4   --export-json ./result/rs-b4.json   '../target/release/ream-core --input ./input/b4.md    >> /dev/null'
