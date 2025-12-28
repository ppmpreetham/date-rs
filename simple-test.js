const { addDays, addMonths, differenceInDays, min, max } = require('./index')

console.assert(addDays(0, 1) === 86400000, 'addDays test failed')
console.assert(differenceInDays(86400000, 0) === 1, 'differenceInDays test failed')
console.assert(min([100, 50, 200]) === 50, 'min test failed')
console.assert(max([100, 50, 200]) === 200, 'max test failed')

const testDate = new Date('2024-01-31').getTime()
const result = addMonths(testDate, 1)
const expected = new Date('2024-02-29').getTime()
console.assert(result === expected, 'addMonths leap year test failed')

console.info('Simple test passed')