function __const__helloWorld() {
  return 5
}

async function main() {
  const a = __const__helloWorld()

  console.log('Wut', a)
}

export const b = 5
