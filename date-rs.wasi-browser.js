import {
  createOnMessage as __wasmCreateOnMessageForFsProxy,
  getDefaultContext as __emnapiGetDefaultContext,
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'



const __wasi = new __WASI({
  version: 'preview1',
})

const __wasmUrl = new URL('./date-rs.wasm32-wasi.wasm', import.meta.url).href
const __emnapiContext = __emnapiGetDefaultContext()


const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    for (const name of Object.keys(instance.exports)) {
      if (name.startsWith('__napi_register__')) {
        instance.exports[name]()
      }
    }
  },
})
export default __napiModule.exports
export const add = __napiModule.exports.add
export const addBusinessDays = __napiModule.exports.addBusinessDays
export const addDays = __napiModule.exports.addDays
export const addHours = __napiModule.exports.addHours
export const addMilliseconds = __napiModule.exports.addMilliseconds
export const addMinutes = __napiModule.exports.addMinutes
export const addMonths = __napiModule.exports.addMonths
export const addQuarters = __napiModule.exports.addQuarters
export const addSeconds = __napiModule.exports.addSeconds
export const addWeeks = __napiModule.exports.addWeeks
export const addYears = __napiModule.exports.addYears
export const areIntervalsOverlapping = __napiModule.exports.areIntervalsOverlapping
export const clamp = __napiModule.exports.clamp
export const closestIndexTo = __napiModule.exports.closestIndexTo
export const closestTo = __napiModule.exports.closestTo
export const compareAsc = __napiModule.exports.compareAsc
export const compareDesc = __napiModule.exports.compareDesc
export const differenceInBusinessDays = __napiModule.exports.differenceInBusinessDays
export const differenceInCalendarDays = __napiModule.exports.differenceInCalendarDays
export const differenceInDays = __napiModule.exports.differenceInDays
export const differenceInHours = __napiModule.exports.differenceInHours
export const differenceInMilliseconds = __napiModule.exports.differenceInMilliseconds
export const differenceInMinutes = __napiModule.exports.differenceInMinutes
export const differenceInMonths = __napiModule.exports.differenceInMonths
export const differenceInQuarters = __napiModule.exports.differenceInQuarters
export const differenceInSeconds = __napiModule.exports.differenceInSeconds
export const differenceInWeeks = __napiModule.exports.differenceInWeeks
export const differenceInYears = __napiModule.exports.differenceInYears
export const eachDayOfInterval = __napiModule.exports.eachDayOfInterval
export const eachMonthOfInterval = __napiModule.exports.eachMonthOfInterval
export const eachQuarterOfInterval = __napiModule.exports.eachQuarterOfInterval
export const eachWeekendOfInterval = __napiModule.exports.eachWeekendOfInterval
export const eachWeekOfInterval = __napiModule.exports.eachWeekOfInterval
export const eachYearOfInterval = __napiModule.exports.eachYearOfInterval
export const endOfMonth = __napiModule.exports.endOfMonth
export const getDaysInMonth = __napiModule.exports.getDaysInMonth
export const intervalToDailyIntervals = __napiModule.exports.intervalToDailyIntervals
export const intervalToDuration = __napiModule.exports.intervalToDuration
export const isAfter = __napiModule.exports.isAfter
export const isBefore = __napiModule.exports.isBefore
export const isEqual = __napiModule.exports.isEqual
export const isSameDay = __napiModule.exports.isSameDay
export const isSaturday = __napiModule.exports.isSaturday
export const isSunday = __napiModule.exports.isSunday
export const isWeekend = __napiModule.exports.isWeekend
export const lastDayOfMonth = __napiModule.exports.lastDayOfMonth
export const max = __napiModule.exports.max
export const min = __napiModule.exports.min
export const parseIso = __napiModule.exports.parseIso
export const startOfDay = __napiModule.exports.startOfDay
export const startOfMonth = __napiModule.exports.startOfMonth
export const subDays = __napiModule.exports.subDays
export const subHours = __napiModule.exports.subHours
export const subMilliseconds = __napiModule.exports.subMilliseconds
export const subMinutes = __napiModule.exports.subMinutes
export const subMonths = __napiModule.exports.subMonths
export const subQuarters = __napiModule.exports.subQuarters
export const subSeconds = __napiModule.exports.subSeconds
export const subWeeks = __napiModule.exports.subWeeks
export const subYears = __napiModule.exports.subYears
