# date-rs
[![npm version](https://img.shields.io/npm/v/date-rs.svg?style=flat-square)](https://www.npmjs.com/package/date-rs)
[![npm downloads](https://img.shields.io/npm/dm/date-rs.svg?style=flat-square)](https://www.npmjs.com/package/date-rs)
[![GitHub stars](https://img.shields.io/github/stars/ppmpreetham/date-rs?style=flat-square)](https://github.com/ppmpreetham/date-rs/stargazers)
[![License](https://img.shields.io/github/license/ppmpreetham/date-rs.svg?style=flat-square)](https://github.com/ppmpreetham/date-rs/blob/main/LICENSE)

A date utility library made in Rust, inspired by date-fns.

> [!WARNING]
> Pure UTC timestamp operations, always exact 24h days, no local DST wall-clock shifts (matches @date-fns/utc, recommended for predictable behavior).


**date-rs** is a high-performance native Node.js addon that brings the familiar API of **[date-fns](https://date-fns.org)** to your projects, but compiled in Rust using **napi-rs** and powered by the rock-solid **`time`** crate.

- **Predictable UTC behavior**: exact 24-hour days, no DST wall-clock surprises (matches `@date-fns/utc` philosophy).
- **Lightning fast**: native Rust performance for heavy date calculations.
- **Drop-in friendly**: functions like `addDays`, `differenceInMonths`, `intervalToDuration`, and more.
- **Zero dependencies**: tiny bundle size, tree-shakable via individual exports (planned).

Perfect for servers, APIs, CLI tools, or any project tired of JavaScript date quirks.
> If you love **date-fns** but want speed + predictability, this is for you.

## Installation

```bash
npm install date-rs
# or
yarn add date-rs
# or
pnpm add date-rs
```

Pre-built binaries for macOS, Linux, and Windows (x64/arm64).

## Quick Start

```js
const {
  addDays,
  addMonths,
  differenceInMonths,
  intervalToDuration,
  eachDayOfInterval,
} = require('date-rs');

// works with JS Date timestamps (ms)
const now = Date.now();

console.log(addDays(now, 7));                    // +7 days (exact 24h * 7)
console.log(addMonths(now, 1));                  // month addition with day clamping
console.log(differenceInMonths(now, now - 30*24*60*60*1000)); // calendar-aware

console.log(intervalToDuration(now - 365*24*60*60*1000, now));
// → { years: 1 } (sparse object, zeros omitted)

console.log(eachDayOfInterval(now - 5*24*60*60*1000, now));
// → array of timestamps for each day (midnight UTC)
```

All functions accept/return **milliseconds since Unix epoch** (`number`), just like `new Date().getTime()`.

## Supported Functions

Core set (growing fast!):

- `addMilliseconds`, `addSeconds`, `addMinutes`, `addHours`, `addDays`, `addWeeks`, `addMonths`, `addQuarters`, `addYears`
- `subMilliseconds`, `subSeconds`, ..., `subYears`
- `differenceInMilliseconds`, `differenceInSeconds`, ..., `differenceInYears`
- `intervalToDuration`, calendar-aware breakdown (sparse output)
- `min`, `max`
- `eachDayOfInterval`, `eachWeekOfInterval`, `eachMonthOfInterval`, `eachQuarterOfInterval`, `eachYearOfInterval`
- `eachWeekendOfInterval`
- `intervalToDailyIntervals` (custom utility)

More coming: `format`, `parseISO`, `startOfDay`, `isBefore`, etc.

## Performance

Performance Comparison (date-rs vs date-fns)
| #  | Operation                | date-rs (ops/sec) | date-fns (ops/sec) | Speedup |
| -- | ------------------------ | -------------- | ------------------ | ------: |
| 0  | addDays                  | 9,735,210      | 4,436,033          |   2.19x |
| 1  | addMonths                | 1,290,617      | 2,405,921          |   0.54x |
| 2  | addYears                 | 1,304,327      | 2,435,951          |   0.54x |
| 3  | differenceInDays         | 10,046,153     | 476,835            |  21.07x |
| 4  | differenceInMonths       | 534,292        | 536,591            |   1.00x |
| 5  | differenceInYears        | 531,558        | 841,040            |   0.63x |
| 6  | min                      | 2,843,677      | 1,171,226          |   2.43x |
| 7  | max                      | 2,760,072      | 1,204,440          |   2.29x |
| 8  | isWeekend                | 1,457,684      | 7,288,419          |   0.20x |
| 9  | eachDayOfInterval        | 206,257        | 94,628             |   2.18x |
| 10 | eachMonthOfInterval      | 346,165        | 271,307            |   1.28x |
| 11 | compareAsc               | 10,045,283     | 3,557,989          |   2.82x |
| 12 | isAfter                  | 10,702,804     | 3,648,799          |   2.93x |
| 13 | isBefore                 | 9,944,303      | 3,563,595          |   2.79x |
| 14 | addBusinessDays          | 1,378,186      | 2,398,528          |   0.57x |
| 15 | differenceInBusinessDays | 532,662        | 261,855            |   2.03x |

## Contributing

Contributions welcome! See [CONTRIBUTING.md](./CONTRIBUTING.md).

- Add more date-fns functions
- Benchmarks
- Docs/examples

## License

MIT © Preetham Pemmasani

---

**Built with ❤️ in Rust for the Node.js ecosystem.**

Star this repo if you like it, helps spread the word! 