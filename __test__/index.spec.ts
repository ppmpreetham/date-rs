import test from 'ava'
import * as dateFns from 'date-fns'
import * as dateRs from '../index.js'

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
test('subMilliseconds matches date-fns', (t) => {
  const date = new Date('2024-01-15T10:30:00')
  const amount = 5000
  
  const rsResult = dateRs.subMilliseconds(date.getTime(), amount)
  const fnsResult = dateFns.subMilliseconds(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subSeconds matches date-fns', (t) => {
  const date = new Date('2024-01-15T10:30:00')
  const amount = 30
  
  const rsResult = dateRs.subSeconds(date.getTime(), amount)
  const fnsResult = dateFns.subSeconds(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subMinutes matches date-fns', (t) => {
  const date = new Date('2024-01-15T10:30:00')
  const amount = 15
  
  const rsResult = dateRs.subMinutes(date.getTime(), amount)
  const fnsResult = dateFns.subMinutes(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subHours matches date-fns', (t) => {
  const date = new Date('2024-01-15T10:00:00')
  const amount = 3
  
  const rsResult = dateRs.subHours(date.getTime(), amount)
  const fnsResult = dateFns.subHours(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subDays matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 5
  
  const rsResult = dateRs.subDays(date.getTime(), amount)
  const fnsResult = dateFns.subDays(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subWeeks matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 2
  
  const rsResult = dateRs.subWeeks(date.getTime(), amount)
  const fnsResult = dateFns.subWeeks(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subMonths matches date-fns', (t) => {
  const date = new Date('2024-03-31')
  const amount = 1
  
  const rsResult = dateRs.subMonths(date.getTime(), amount)
  const fnsResult = dateFns.subMonths(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subQuarters matches date-fns', (t) => {
  const date = new Date('2024-07-15')
  const amount = 1
  
  const rsResult = dateRs.subQuarters(date.getTime(), amount)
  const fnsResult = dateFns.subQuarters(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('subYears matches date-fns', (t) => {
  const date = new Date('2024-01-15')
  const amount = 2
  
  const rsResult = dateRs.subYears(date.getTime(), amount)
  const fnsResult = dateFns.subYears(date, amount).getTime()
  
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

test('differenceInCalendarDays matches date-fns', (t) => {
  const date1 = new Date('2024-01-20')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInCalendarDays(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInCalendarDays(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('differenceInBusinessDays matches date-fns', (t) => {
  const date1 = new Date('2024-01-20')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.differenceInBusinessDays(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.differenceInBusinessDays(date1, date2)
  
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

test('closestTo matches date-fns', (t) => {
  const target = new Date('2024-01-15')
  const dates = [
    new Date('2024-01-10'),
    new Date('2024-01-20'),
    new Date('2024-01-14'),
  ]
  
  const rsResult = dateRs.closestTo(target.getTime(), dates.map(d => d.getTime()))
  const fnsResult = dateFns.closestTo(target, dates)?.getTime()
  
  t.is(rsResult, fnsResult)
})

test('closestIndexTo matches date-fns', (t) => {
  const target = new Date('2024-01-15')
  const dates = [
    new Date('2024-01-10'),
    new Date('2024-01-20'),
    new Date('2024-01-14'),
  ]
  
  const rsResult = dateRs.closestIndexTo(target.getTime(), dates.map(d => d.getTime()))
  const fnsResult = dateFns.closestIndexTo(target, dates)
  
  t.is(rsResult, fnsResult)
})

test('compareAsc matches date-fns', (t) => {
  const date1 = new Date('2024-01-15')
  const date2 = new Date('2024-01-20')
  
  const rsResult = dateRs.compareAsc(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.compareAsc(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('compareDesc matches date-fns', (t) => {
  const date1 = new Date('2024-01-15')
  const date2 = new Date('2024-01-20')
  
  const rsResult = dateRs.compareDesc(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.compareDesc(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('isEqual matches date-fns', (t) => {
  const date1 = new Date('2024-01-15')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.isEqual(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.isEqual(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('isAfter matches date-fns', (t) => {
  const date1 = new Date('2024-01-20')
  const date2 = new Date('2024-01-15')
  
  const rsResult = dateRs.isAfter(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.isAfter(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('isBefore matches date-fns', (t) => {
  const date1 = new Date('2024-01-15')
  const date2 = new Date('2024-01-20')
  
  const rsResult = dateRs.isBefore(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.isBefore(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('isWeekend matches date-fns', (t) => {
  const saturday = new Date('2024-01-06') // Saturday
  const monday = new Date('2024-01-08') // Monday
  
  t.is(dateRs.isWeekend(saturday.getTime()), dateFns.isWeekend(saturday))
  t.is(dateRs.isWeekend(monday.getTime()), dateFns.isWeekend(monday))
})

test('isSaturday matches date-fns', (t) => {
  const saturday = new Date('2024-01-06')
  
  const rsResult = dateRs.isSaturday(saturday.getTime())
  const fnsResult = dateFns.isSaturday(saturday)
  
  t.is(rsResult, fnsResult)
})

test('isSunday matches date-fns', (t) => {
  const sunday = new Date('2024-01-07')
  
  const rsResult = dateRs.isSunday(sunday.getTime())
  const fnsResult = dateFns.isSunday(sunday)
  
  t.is(rsResult, fnsResult)
})

test('isSameDay matches date-fns', (t) => {
  const date1 = new Date('2024-01-15T10:00:00')
  const date2 = new Date('2024-01-15T15:00:00')
  
  const rsResult = dateRs.isSameDay(date1.getTime(), date2.getTime())
  const fnsResult = dateFns.isSameDay(date1, date2)
  
  t.is(rsResult, fnsResult)
})

test('clamp matches date-fns', (t) => {
  const date = new Date('2024-01-20')
  const start = new Date('2024-01-10')
  const end = new Date('2024-01-15')
  
  const rsResult = dateRs.clamp(date.getTime(), start.getTime(), end.getTime())
  const fnsResult = dateFns.clamp(date, { start, end }).getTime()
  
  t.is(rsResult, fnsResult)
})

test('areIntervalsOverlapping matches date-fns', (t) => {
  const interval1 = { start: new Date('2024-01-10'), end: new Date('2024-01-20') }
  const interval2 = { start: new Date('2024-01-15'), end: new Date('2024-01-25') }
  
  const rsResult = dateRs.areIntervalsOverlapping(
    interval1.start.getTime(),
    interval1.end.getTime(),
    interval2.start.getTime(),
    interval2.end.getTime()
  )
  const fnsResult = dateFns.areIntervalsOverlapping(interval1, interval2)
  
  t.is(rsResult, fnsResult)
})

test('addBusinessDays matches date-fns', (t) => {
  const date = new Date('2024-01-15') // Monday
  const amount = 5
  
  const rsResult = dateRs.addBusinessDays(date.getTime(), amount)
  const fnsResult = dateFns.addBusinessDays(date, amount).getTime()
  
  t.is(rsResult, fnsResult)
})

test('intervalToDuration basic test', (t) => {
  const start = new Date('2024-01-01')
  const end = new Date('2024-01-02')
  
  const result = dateRs.intervalToDuration(start.getTime(), end.getTime())
  
  t.true(typeof result === 'object')
  t.true(result.days !== undefined)
})

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