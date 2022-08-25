import Path from 'path'
import Compiler from './compiler'
import rewire from 'rewire'
import { EvaluatedConstFunction } from '../generated'
// import TsNode from 'ts-node'

async function resolveMaybePromise(val: Promise<any> | any) {
  return val
}

async function main() {
  // const modulePath = Path.join(__dirname, '..', '..', 'examples', 'index.js')
  const modulePath = Path.join(process.cwd(), process.argv[2])
  const rewired = rewire(modulePath)

  const result = await Compiler.findConstFunctions(modulePath, async constFunctions => {
    console.log('FROM NODE | Got const functions! ', constFunctions)
    const res: EvaluatedConstFunction[] = []

    for (const constFunction of constFunctions) {
      const const_fn = rewired.__get__(constFunction)

      console.log('FROM NODE | Resolving const funcs')
      const result = await resolveMaybePromise(const_fn())
      console.log('FROM NODE | Done')

      res.push({ name: constFunction, value: result })

      console.log({ res })
    }

    return res
  })
}

main().catch(console.error)
