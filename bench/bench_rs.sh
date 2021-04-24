#!/bin/bash

echo b1
hyperfine -n rs_b1   --export-json ./result/04_24_2021/rs-b1-json.json    '../target/release/ream-core --input ./input/b1.md   --format JSON --output ./output/b1.json    '
hyperfine -n rs_b1   --export-json ./result/04_24_2021/rs-b1-csv.json     '../target/release/ream-core --input ./input/b1.md   --format CSV  --output ./output/b1.csv     '
echo b2-1
hyperfine -n rs_b2-1 --export-json ./result/04_24_2021/rs-b2-1-json.json  '../target/release/ream-core --input ./input/b2-1.md --format JSON --output ./output/b2-1.json  '
hyperfine -n rs_b2-1 --export-json ./result/04_24_2021/rs-b2-1-csv.json   '../target/release/ream-core --input ./input/b2-1.md --format CSV  --output ./output/b2-1.csv   '
echo b2-2
hyperfine -n rs_b2-2 --export-json ./result/04_24_2021/rs-b2-2-json.json  '../target/release/ream-core --input ./input/b2-2.md --format JSON --output ./output/b2-2.json  '
hyperfine -n rs_b2-2 --export-json ./result/04_24_2021/rs-b2-2-csv.json   '../target/release/ream-core --input ./input/b2-2.md --format CSV  --output ./output/b2-2.csv   '
echo b2-3
hyperfine -n rs_b2-3 --export-json ./result/04_24_2021/rs-b2-3-json.json  '../target/release/ream-core --input ./input/b2-3.md --format JSON --output ./output/b2-3.json  '
hyperfine -n rs_b2-3 --export-json ./result/04_24_2021/rs-b2-3-csv.json   '../target/release/ream-core --input ./input/b2-3.md --format CSV --output ./output/b2-3.csv   '
echo b3-1
hyperfine -n rs_b3-1 --export-json ./result/04_24_2021/rs-b3-1-json.json  '../target/release/ream-core --input ./input/b3-1.md --format JSON --output ./output/b3-1.json  '
hyperfine -n rs_b3-1 --export-json ./result/04_24_2021/rs-b3-1-csv.json   '../target/release/ream-core --input ./input/b3-1.md --format CSV --output ./output/b3-1.csv   '
echo b3-2
hyperfine -n rs_b3-2 --export-json ./result/04_24_2021/rs-b3-2-json.json  '../target/release/ream-core --input ./input/b3-2.md --format JSON --output ./output/b3-2.json  '
hyperfine -n rs_b3-2 --export-json ./result/04_24_2021/rs-b3-2-csv.json   '../target/release/ream-core --input ./input/b3-2.md --format CSV --output ./output/b3-2.csv   '
echo b3-3
hyperfine -n rs_b3-3 --export-json ./result/04_24_2021/rs-b3-3-json.json  '../target/release/ream-core --input ./input/b3-3.md --format JSON --output ./output/b3-3.json  '
hyperfine -n rs_b3-3 --export-json ./result/04_24_2021/rs-b3-3-csv.json   '../target/release/ream-core --input ./input/b3-3.md --format CSV  --output ./output/b3-3.csv   '
echo b4
hyperfine -n rs_b4   --export-json ./result/04_24_2021/rs-b4-json.json    '../target/release/ream-core --input ./input/b4.md   --format JSON --output ./output/b4.json  '
hyperfine -n rs_b4   --export-json ./result/04_24_2021/rs-b4-csv.json     '../target/release/ream-core --input ./input/b4.md   --format CSV  --output ./output/b4.csv   '
