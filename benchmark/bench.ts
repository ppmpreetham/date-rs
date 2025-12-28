import { Bench } from 'tinybench'
import {
  addDays,
  addMonths,
  differenceInDays,
  differenceInMonths,
  eachDayOfInterval,
} from '../index.js'

const bench = new Bench()

const testDate = new Date('2024-01-15').getTime()
const futureDate = new Date('2024-12-31').getTime()

bench
  .add('addDays (Rust)', () => {
    addDays(testDate, 30)
  })
  .add('addDays (Native JS)', () => {
    testDate + 30 * 86400000
  })
  .add('addMonths (Rust)', () => {
    addMonths(testDate, 3)
  })
  .add('differenceInDays (Rust)', () => {
    differenceInDays(futureDate, testDate)
  })
  .add('differenceInDays (Native JS)', () => {
    Math.trunc((futureDate - testDate) / 86400000)
  })
  .add('differenceInMonths (Rust)', () => {
    differenceInMonths(futureDate, testDate)
  })
  .add('eachDayOfInterval (Rust)', () => {
    eachDayOfInterval(testDate, addDays(testDate, 30))
  })

await bench.run()

console.table(bench.table())
