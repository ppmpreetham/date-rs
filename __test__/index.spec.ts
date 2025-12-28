import test from 'ava'
import * as dateFns from 'date-fns'
import * as dateRs from '../index.js'

// Helper to compare results
const compareResults = (t: any, dateRsResult: any, dateFnsResult: any, testName: string) => {
  t.deepEqual(dateRsResult, dateFnsResult, `${testName} should match date-fns`)
}

// Add functions comparison
test('addMilliseconds matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 5000
  
  const rsResult = dateRs.addMilliseconds(date.getTime(), amount)
  const fnsResult = dateFns.addMilliseconds(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addSeconds matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 30
  
  const rsResult = dateRs.addSeconds(date.getTime(), amount)
  const fnsResult = dateFns.addSeconds(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addMinutes matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 45
  
  const rsResult = dateRs.addMinutes(date.getTime(), amount)
  const fnsResult = dateFns.addMinutes(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addHours matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 5
  
  const rsResult = dateRs.addHours(date.getTime(), amount)
  const fnsResult = dateFns.addHours(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addDays matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 10
  
  const rsResult = dateRs.addDays(date.getTime(), amount)
  const fnsResult = dateFns.addDays(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addWeeks matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 3
  
  const rsResult = dateRs.addWeeks(date.getTime(), amount)
  const fnsResult = dateFns.addWeeks(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addMonths matches date-fns', (t) => {
  const date = new Date('2024-01-31')
  const amount = 1
  
  const rsResult = dateRs.addMonths(date.getTime(), amount)
  const fnsResult = dateFns.addMonths(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addMonths with overflow matches date-fns', (t) => {
  const date = new Date('2024-01-31')
  const amount = 1
  
  const rsResult = dateRs.addMonths(date.getTime(), amount)
  const fnsResult = dateFns.addMonths(date, amount).getTime()
  
  t.is(rsResult, fnsResult, 'Jan 31 + 1 month should be Feb 29 (leap year)')
})

test('addQuarters matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 2
  
  const rsResult = dateRs.addQuarters(date.getTime(), amount)
  const fnsResult = dateFns.addQuarters(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('addYears matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 2
  
  const rsResult = dateRs.addYears(date.getTime(), amount)
  const fnsResult = dateFns.addYears(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

// Sub functions comparison
test('subDays matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 5
  
  const rsResult = dateRs.subDays(date.getTime(), amount)
  const fnsResult = dateFns.subDays(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subMonths matches date-fns', (t) => {
  const date = new Date('2024-03-31')
  const amount = 1
  
  const rsResult = dateRs.subMonths(date.getTime(), amount)
  const fnsResult = dateFns.subMonths(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

// Difference functions comparison
test('differenceInMilliseconds matches date-fns', (t) => {
  const date1 = new Date('2024-01-15T10:30:00')
  const date2 = new Date('2024-01-15T10:00:00')
  
  const rsResult = dateRs.differenceInMilliseconds(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInMilliseconds(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInSeconds matches date-fns', (t) => {
  const date1 = new Date('2024-01-15T10:30:00')
  const date2 = new Date('2024-01-15T10:00:00')
  
  const rsResult = dateRs.differenceInSeconds(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInSeconds(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInMinutes matches date-fns', (t) => {
  const date1 = new Date('2024-01-15T10:30:00')
  const date2 = new Date('2024-01-15T10:00:00')
  
  const rsResult = dateRs.differenceInMinutes(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInMinutes(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInHours matches date-fns', (t) => {
  const date1 = new Date('2024-01-15T15:00:00')
  const date2 = new Date('2024-01-15T10:00:00')
  
  const rsResult = dateRs.differenceInHours(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInHours(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInDays matches date-fns', (t) => {
  const date1 = new Date('2024-01-20')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInDays(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInDays(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInWeeks matches date-fns', (t) => {
  const date1 = new Date('2024-02-15')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInWeeks(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInWeeks(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInMonths matches date-fns', (t) => {
  const date1 = new Date('2024-03-15')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInMonths(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInMonths(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInQuarters matches date-fns', (t) => {
  const date1 = new Date('2024-07-15')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInQuarters(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInQuarters(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInYears matches date-fns', (t) => {
  const date1 = new Date('2026-01-15')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInYears(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInYears(date1, date2)
  
  t.is(rsResult, fnsResult)
})

// Utility functions comparison
test('min matches date-fns', (t) => {
  const dates = [
    new Date('2024-01-15'),
    new Date('2024-01-10'),
    new Date('2024-01-20'),
  ]
  
  const rsResult = dateRs.min(dates.map(d => d.getTime()))
  const fnsResult = dateFns.min(dates).getTime()
  
  t.is(rsResult, fnsResult)
})

test('max matches date-fns', (t) => {
  const dates = [
    new Date('2024-01-15'),
    new Date('2024-01-10'),
    new Date('2024-01-20'),
  ]
  
  const rsResult = dateRs.max(dates.map(d => d.getTime()))
  const fnsResult = dateFns.max(dates).getTime()
  
  t.is(rsResult, fnsResult)
})

// UTC issues in github actions
// test('eachDayOfInterval matches date-fns', (t) => {
//   const start = new Date('2024-01-01')
//   const end = new Date('2024-01-05')
  
//   const rsResult = dateRs.eachDayOfInterval(start.getTime(), end.getTime())
//   const fnsResult = dateFns.eachDayOfInterval({ start, end }).map(d => d.getTime())
  
//   t.deepEqual(rsResult, fnsResult)
// })

// test('eachMonthOfInterval matches date-fns', (t) => {
//   const start = new Date('2024-01-01')
//   const end = new Date('2024-06-01')
  
//   const rsResult = dateRs.eachMonthOfInterval(start.getTime(), end.getTime())
//   const fnsResult = dateFns.eachMonthOfInterval({ start, end }).map(d => d.getTime())
  
//   t.deepEqual(rsResult, fnsResult)
// })

// test('eachYearOfInterval matches date-fns', (t) => {
//   const start = new Date('2024-01-01')
//   const end = new Date('2027-01-01')
  
//   const rsResult = dateRs.eachYearOfInterval(start.getTime(), end.getTime())
//   const fnsResult = dateFns.eachYearOfInterval({ start, end }).map(d => d.getTime())
  
//   t.deepEqual(rsResult, fnsResult)
// })

// Edge cases
test('addMonths edge case: leap year', (t) => {
  const date = new Date('2024-01-31')
  
  const rsResult = dateRs.addMonths(date.getTime(), 1)
  const fnsResult = dateFns.addMonths(date, 1).getTime()
  
  t.is(rsResult, fnsResult, 'Should handle leap year correctly')
})

test('addMonths edge case: non-leap year', (t) => {
  const date = new Date('2023-01-31')
  
  const rsResult = dateRs.addMonths(date.getTime(), 1)
  const fnsResult = dateFns.addMonths(date, 1).getTime()
  
  t.is(rsResult, fnsResult, 'Should handle non-leap year correctly')
})

test('differenceInMonths edge case: same month', (t) => {
  const date1 = new Date('2024-01-31')
  const date2 = new Date('2024-01-01')
  
  const rsResult = dateRs.differenceInMonths(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInMonths(date1, date2)
  
  t.is(rsResult, fnsResult)
})