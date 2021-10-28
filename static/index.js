async function main() {
  console.log("loading...")
  const lib = await import("../pkg/ream.js").catch(console.error);

  const f = lib.ream2csv;
  console.log(f(`
  # Data
  - country: Belgium
  - capital: Brussel
  `));
}

main();
