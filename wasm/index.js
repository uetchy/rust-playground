async function wasm() {
  return new Promise((resolve) => import('./pkg/plum').then(resolve))
}

async function main() {
  const mod = await wasm()
  console.log('1 + 2 = ' + mod.sum(1, 2))
}

main().catch(console.error)
