import { Bench } from 'tinybench'
import * as dateFns from 'date-fns'
import * as dateRs from '../index.js'

const bench = new Bench({ time: 1000 })

const testDate = new Date('2024-01-15').getTime()
const futureDate = new Date('2024-12-31').getTime()
const dates = [
  new Date('2024-01-10').getTime(),
  new Date('2024-01-20').getTime(),
  new Date('2024-01-15').getTime(),
]

// Add operations
bench
  .add('addDays (Rust)', () => {
    dateRs.addDays(testDate, 30)
  })
  .add('addDays (date-fns)', () => {
    dateFns.addDays(testDate, 30)
  })
  .add('addMonths (Rust)', () => {
    dateRs.addMonths(testDate, 3)
  })
  .add('addMonths (date-fns)', () => {
    dateFns.addMonths(testDate, 3)
  })
  .add('addYears (Rust)', () => {
    dateRs.addYears(testDate, 2)
  })
  .add('addYears (date-fns)', () => {
    dateFns.addYears(testDate, 2)
  })

// Difference operations
bench
  .add('differenceInDays (Rust)', () => {
    dateRs.differenceInDays(futureDate, testDate)
  })
  .add('differenceInDays (date-fns)', () => {
    dateFns.differenceInDays(futureDate, testDate)
  })
  .add('differenceInMonths (Rust)', () => {
    dateRs.differenceInMonths(futureDate, testDate)
  })
  .add('differenceInMonths (date-fns)', () => {
    dateFns.differenceInMonths(futureDate, testDate)
  })
  .add('differenceInYears (Rust)', () => {
    dateRs.differenceInYears(futureDate, testDate)
  })
  .add('differenceInYears (date-fns)', () => {
    dateFns.differenceInYears(futureDate, testDate)
  })

// Utility operations
bench
  .add('min (Rust)', () => {
    dateRs.min(dates)
  })
  .add('min (date-fns)', () => {
    dateFns.min(dates.map(d => new Date(d)))
  })
  .add('max (Rust)', () => {
    dateRs.max(dates)
  })
  .add('max (date-fns)', () => {
    dateFns.max(dates.map(d => new Date(d)))
  })
  .add('isWeekend (Rust)', () => {
    dateRs.isWeekend(testDate)
  })
  .add('isWeekend (date-fns)', () => {
    dateFns.isWeekend(testDate)
  })

// Interval operations
bench
  .add('eachDayOfInterval (Rust)', () => {
    dateRs.eachDayOfInterval(testDate, dateRs.addDays(testDate, 30))
  })
  .add('eachDayOfInterval (date-fns)', () => {
    dateFns.eachDayOfInterval({
      start: testDate,
      end: dateFns.addDays(testDate, 30),
    })
  })
  .add('eachMonthOfInterval (Rust)', () => {
    dateRs.eachMonthOfInterval(testDate, futureDate)
  })
  .add('eachMonthOfInterval (date-fns)', () => {
    dateFns.eachMonthOfInterval({ start: testDate, end: futureDate })
  })

// Comparison operations
bench
  .add('compareAsc (Rust)', () => {
    dateRs.compareAsc(testDate, futureDate)
  })
  .add('compareAsc (date-fns)', () => {
    dateFns.compareAsc(testDate, futureDate)
  })
  .add('isAfter (Rust)', () => {
    dateRs.isAfter(futureDate, testDate)
  })
  .add('isAfter (date-fns)', () => {
    dateFns.isAfter(futureDate, testDate)
  })
  .add('isBefore (Rust)', () => {
    dateRs.isBefore(testDate, futureDate)
  })
  .add('isBefore (date-fns)', () => {
    dateFns.isBefore(testDate, futureDate)
  })

// Business days
bench
  .add('addBusinessDays (Rust)', () => {
    dateRs.addBusinessDays(testDate, 10)
  })
  .add('addBusinessDays (date-fns)', () => {
    dateFns.addBusinessDays(testDate, 10)
  })
  .add('differenceInBusinessDays (Rust)', () => {
    dateRs.differenceInBusinessDays(futureDate, testDate)
  })
  .add('differenceInBusinessDays (date-fns)', () => {
    dateFns.differenceInBusinessDays(futureDate, testDate)
  })

await bench.run()
console.log('\n📊 Benchmark Results\n')
console.table(
  bench.tasks.map((task) => {
    const state = task.result?.state

    // Only show statistics for completed or aborted-with-statistics tasks
    if (state === 'aborted-with-statistics' || state === 'completed') {
      const ops = task.result?.throughput?.mean
      const avgMs = task.result?.latency?.mean
      const rme = task.result?.throughput?.rme

      return {
        'Task Name': task.name,
        'ops/sec': typeof ops === 'number' ? Math.round(ops).toFixed(0) : 'N/A',
        'Average Time (ms)': typeof avgMs === 'number' ? avgMs.toFixed(4) : 'N/A',
        'Margin': typeof rme === 'number' ? `±${rme.toFixed(2)}%` : 'N/A',
      }
    }

    // Non-completed states
    return {
      'Task Name': task.name,
      'ops/sec': 'N/A',
      'Average Time (ms)': 'N/A',
      'Margin': 'N/A',
      Remarks: state ?? 'N/A',
    }
  })
)

// Group by operation and show Rust vs date-fns comparison
console.log('\n🚀 Performance Comparison (Rust vs date-fns)\n')

const grouped: Record<string, { rust?: number; dateFns?: number }> = {}

const hasThroughput = (r: unknown): r is { throughput: { mean: number } } =>
  !!r && typeof (r as any).throughput?.mean === 'number'

bench.tasks.forEach((task) => {
  const name = task.name.replace(/ \((Rust|date-fns)\)/, '')
  const type = task.name.includes('(Rust)') ? 'rust' : 'dateFns'

  if (!grouped[name]) grouped[name] = {}
  grouped[name][type] = hasThroughput(task.result) ? task.result.throughput.mean : undefined
})

console.table(
  Object.entries(grouped).map(([name, data]) => {
    const rust = data.rust
    const dateFns = data.dateFns
    const speedup = typeof rust === 'number' && typeof dateFns === 'number' ? (rust / dateFns).toFixed(2) : 'N/A'
    return {
      Operation: name,
      'Rust (ops/sec)': typeof rust === 'number' ? Math.round(rust).toFixed(0) : 'N/A',
      'date-fns (ops/sec)': typeof dateFns === 'number' ? Math.round(dateFns).toFixed(0) : 'N/A',
      'Speedup': speedup === 'N/A' ? 'N/A' : `${speedup}x`,
    }
  })
)