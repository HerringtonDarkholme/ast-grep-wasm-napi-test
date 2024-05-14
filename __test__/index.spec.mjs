import test from 'ava'

import { parse } from '../index.js'

test('should be able to parse Rust', (t) => {
  t.snapshot(parse(`fn main() { println!("Hello, World!"); }`))
})
