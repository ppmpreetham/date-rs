#![deny(clippy::all)]

use napi::bindgen_prelude::Object;
use napi_derive::napi;
use time::format_description::well_known::Iso8601;
use time::util::days_in_month;
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset, Weekday};

// addMilliseconds, addSeconds, addMinutes, addHours,
// addDays, addWeeks, addMonths, addQuarters, addYears

#[napi]
pub fn add_milliseconds(date_ms: f64, amount: f64) -> f64 {
  date_ms + amount
}

#[napi]
pub fn add_seconds(date_ms: f64, amount: f64) -> f64 {
  date_ms + amount * 1000.0
}

#[napi]
pub fn add_minutes(date_ms: f64, amount: f64) -> f64 {
  date_ms + amount * 60_000.0
}

#[napi]
pub fn add_hours(date_ms: f64, amount: f64) -> f64 {
  date_ms + amount * 3_600_000.0
}

#[napi]
pub fn add_days(date_ms: f64, amount: f64) -> f64 {
  date_ms + amount * 86_400_000.0
}

#[napi]
pub fn add_weeks(date_ms: f64, amount: f64) -> f64 {
  date_ms + amount * 604_800_000.0
}

#[napi]
pub fn add_months(date_ms: f64, amount: f64) -> f64 {
  if !date_ms.is_finite() || !amount.is_finite() {
    return f64::NAN;
  }

  let Ok(date) = OffsetDateTime::from_unix_timestamp_nanos((date_ms * 1_000_000.0) as i128) else {
    return f64::NAN;
  };

  let mut year = date.year();
  let month0 = date.month() as i32 - 1;
  let day = date.day();

  let amount_i32 = amount.trunc() as i32;

  let total = month0 + amount_i32;

  year += total.div_euclid(12);
  let month_index = total.rem_euclid(12);

  let Ok(month) = Month::try_from((month_index + 1) as u8) else {
    return f64::NAN;
  };

  let Ok(tmp) = date.replace_day(1) else {
    return f64::NAN;
  };

  let Ok(tmp) = tmp.replace_year(year) else {
    return f64::NAN;
  };

  let Ok(tmp) = tmp.replace_month(month) else {
    return f64::NAN;
  };

  let days = days_in_month(month, year);
  let clamped_day = day.min(days);

  let Ok(new_date) = tmp.replace_day(clamped_day) else {
    return f64::NAN;
  };

  new_date.unix_timestamp_nanos() as f64 / 1_000_000.0
}

#[napi]
pub fn add_quarters(date_ms: f64, amount: f64) -> f64 {
  add_months(date_ms, amount * 3.0)
}

#[napi]
pub fn add_years(date_ms: f64, amount: f64) -> f64 {
  add_months(date_ms, amount * 12.0)
}

// subMilliseconds, subSeconds, subMinutes,
// subHours, subDays, subWeeks, subMonths,
// subQuarters, subYears

#[napi]
pub fn sub_milliseconds(date_ms: f64, amount: f64) -> f64 {
  date_ms - amount
}

#[napi]
pub fn sub_seconds(date_ms: f64, amount: f64) -> f64 {
  date_ms - amount * 1000.0
}

#[napi]
pub fn sub_minutes(date_ms: f64, amount: f64) -> f64 {
  date_ms - amount * 60_000.0
}

#[napi]
pub fn sub_hours(date_ms: f64, amount: f64) -> f64 {
  date_ms - amount * 3_600_000.0
}

#[napi]
pub fn sub_days(date_ms: f64, amount: f64) -> f64 {
  date_ms - amount * 86_400_000.0
}

#[napi]
pub fn sub_weeks(date_ms: f64, amount: f64) -> f64 {
  date_ms - amount * 604_800_000.0
}

#[napi]
pub fn sub_months(date_ms: f64, amount: f64) -> f64 {
  add_months(date_ms, -amount)
}

#[napi]
pub fn sub_quarters(date_ms: f64, amount: f64) -> f64 {
  add_quarters(date_ms, -amount)
}

#[napi]
pub fn sub_years(date_ms: f64, amount: f64) -> f64 {
  add_years(date_ms, -amount)
}

// differenceInMilliseconds,
// differenceInSeconds,
// differenceInMinutes,
// differenceInHours,
// differenceInDays,
// differenceInWeeks,
// differenceInMonths,
// differenceInQuarters,
// differenceInYears

// -- start helper fns
fn from_ms_local(ms: f64) -> Option<OffsetDateTime> {
  if !ms.is_finite() {
    return None;
  }

  let utc = OffsetDateTime::from_unix_timestamp_nanos((ms * 1_000_000.0) as i128).ok()?;
  let offset = UtcOffset::current_local_offset().ok()?;
  Some(utc.to_offset(offset))
}

fn from_ms(ms: f64) -> Option<OffsetDateTime> {
  if !ms.is_finite() {
    return None;
  }
  OffsetDateTime::from_unix_timestamp_nanos((ms * 1_000_000.0) as i128).ok()
}

fn trunc_toward_zero(v: f64) -> i64 {
  if v.is_sign_negative() {
    v.ceil() as i64
  } else {
    v.floor() as i64
  }
}

fn to_ms(dt: OffsetDateTime) -> f64 {
  dt.unix_timestamp_nanos() as f64 / 1_000_000.0
}

fn add_months_local(ms: f64, amount: i32) -> Option<OffsetDateTime> {
  let dt = from_ms_local(ms)?;

  let mut year = dt.year();
  let month0 = dt.month() as i32 - 1;
  let day = dt.day();

  let total = month0 + amount;
  year += total.div_euclid(12);
  let month_index = total.rem_euclid(12);

  let month = Month::try_from((month_index + 1) as u8).ok()?;
  let dim = days_in_month(month, year);
  let clamped = day.min(dim as u8);

  let tmp = dt.replace_day(1).ok()?;
  let tmp = tmp.replace_year(year).ok()?;
  let tmp = tmp.replace_month(month).ok()?;
  tmp.replace_day(clamped).ok()
}

// -- end helper fns

#[napi]
pub fn difference_in_milliseconds(a_ms: f64, b_ms: f64) -> f64 {
  a_ms - b_ms
}

#[napi]
pub fn difference_in_seconds(a_ms: f64, b_ms: f64) -> i64 {
  trunc_toward_zero((a_ms - b_ms) / 1000.0)
}

#[napi]
pub fn difference_in_minutes(a_ms: f64, b_ms: f64) -> i64 {
  trunc_toward_zero((a_ms - b_ms) / 60_000.0)
}

#[napi]
pub fn difference_in_hours(a_ms: f64, b_ms: f64) -> i64 {
  trunc_toward_zero((a_ms - b_ms) / 3_600_000.0)
}

#[napi]
pub fn difference_in_days(a_ms: f64, b_ms: f64) -> i64 {
  trunc_toward_zero((a_ms - b_ms) / 86_400_000.0)
}

#[napi]
pub fn difference_in_weeks(a_ms: f64, b_ms: f64) -> i64 {
  trunc_toward_zero((a_ms - b_ms) / 604_800_000.0)
}

// LOCAL CALENDAR DIFFERENCE (date-fns compatible)
// ----------------------------------------------
// This implementation intentionally mirrors `date-fns`:
//
//   differenceInMonths(a, b)
//
// which computes the difference using LOCAL TIME semantics.
//
// Key behavior:
//
//  • Inputs are converted via from_ms_local(...)
//      → timestamps are interpreted in the system time zone
//
//  • Calendar math is done on LOCAL YEAR + MONTH values
//
//  • The fractional month remainder is discarded toward zero
//      (i.e., the result is an integer count of whole calendar months)
//
//  • Correction logic adjusts for overshoot:
//      difference = months between calendar labels
//      then shift b forward by that many months in LOCAL TIME
//      and step back/forward if we crossed past `a`
//
// This matches:
//
//   date-fns.differenceInMonths()
//
// including DST transitions & odd-length months.
//
// ----------------------------------------------
// HOW TO MAKE THIS UTC (future /utc build)
// ----------------------------------------------
//
// 1) Replace from_ms_local(...) → from_ms(...)
//      → interpret timestamps as UTC, not local
//
// 2) Perform the same calendar math on UTC components
//
// 3) Keep the overshoot-correction logic the same
//
// Result will match:
//
//   @date-fns/utc.differenceInMonths()
//
// ----------------------------------------------
// TL;DR
// Current: local calendar months (== date-fns)
// Future:  UTC calendar months (== @date-fns/utc)
#[napi]
pub fn difference_in_months(a_ms: f64, b_ms: f64) -> i64 {
  let (Some(a), Some(b)) = (from_ms_local(a_ms), from_ms_local(b_ms)) else {
    return 0;
  };

  let sign = if a >= b { 1 } else { -1 };

  let years = a.year() - b.year();
  let mut months = years * 12 + (a.month() as i32 - b.month() as i32);

  let shifted = add_months_local(b_ms, months).unwrap();

  if sign > 0 && shifted > a {
    months -= 1;
  } else if sign < 0 && shifted < a {
    months += 1;
  }

  months as i64
}

#[napi]
pub fn difference_in_quarters(a_ms: f64, b_ms: f64) -> i64 {
  trunc_toward_zero(difference_in_months(a_ms, b_ms) as f64 / 3.0)
}

#[napi]
pub fn difference_in_years(a_ms: f64, b_ms: f64) -> i64 {
  let months = difference_in_months(a_ms, b_ms);
  trunc_toward_zero(months as f64 / 12.0)
}

// intervalToDuration,
// intervalToIntervals,
// min, max,
// eachDayOfInterval,
// eachWeekOfInterval,
// eachMonthOfInterval,
// eachQuarterOfInterval,
// eachYearOfInterval,
// eachWeekendOfInterval

#[napi(object)]
pub struct DurationParts {
  pub years: Option<i32>,
  pub months: Option<i32>,
  pub days: Option<i32>,
  pub hours: Option<i32>,
  pub minutes: Option<i32>,
  pub seconds: Option<i32>,
}

#[napi]
pub fn interval_to_duration(start_ms: f64, end_ms: f64) -> DurationParts {
  let (Some(mut start), Some(mut end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return DurationParts {
      years: None,
      months: None,
      days: None,
      hours: None,
      minutes: None,
      seconds: None,
    };
  };

  if start > end {
    std::mem::swap(&mut start, &mut end);
  }

  let mut duration = DurationParts {
    years: None,
    months: None,
    days: None,
    hours: None,
    minutes: None,
    seconds: None,
  };

  let years = difference_in_years(to_ms(end), to_ms(start)) as i32;
  if years != 0 {
    duration.years = Some(years);
  }
  let after_years = add_years(to_ms(start), years as f64);

  let months = difference_in_months(to_ms(end), after_years) as i32;
  if months != 0 {
    duration.months = Some(months);
  }
  let after_months = add_months(after_years, months as f64);

  let days = difference_in_days(to_ms(end), after_months) as i32;
  if days != 0 {
    duration.days = Some(days);
  }
  let after_days = add_days(after_months, days as f64);

  let hours = difference_in_hours(to_ms(end), after_days) as i32;
  if hours != 0 {
    duration.hours = Some(hours);
  }
  let after_hours = add_hours(after_days, hours as f64);

  let minutes = difference_in_minutes(to_ms(end), after_hours) as i32;
  if minutes != 0 {
    duration.minutes = Some(minutes);
  }
  let after_minutes = add_minutes(after_hours, minutes as f64);

  let seconds = difference_in_seconds(to_ms(end), after_minutes) as i32;
  if seconds != 0 {
    duration.seconds = Some(seconds);
  }

  duration
}

#[napi]
pub fn min(dates: Vec<f64>) -> f64 {
  if dates.is_empty() || dates.iter().any(|&d| !d.is_finite()) {
    return f64::NAN;
  }
  dates.into_iter().fold(f64::INFINITY, f64::min)
}

#[napi]
pub fn max(dates: Vec<f64>) -> f64 {
  if dates.is_empty() || dates.iter().any(|&d| !d.is_finite()) {
    return f64::NAN;
  }
  dates.into_iter().fold(f64::NEG_INFINITY, f64::max)
}

#[napi]
pub fn each_week_of_interval(start_ms: f64, end_ms: f64, options: Option<Object>) -> Vec<f64> {
  use napi::bindgen_prelude::JsObjectValue;

  let week_starts_on: u8 = options
    .as_ref()
    .and_then(|o| o.get_named_property::<u8>("weekStartsOn").ok())
    .unwrap_or(0);

  let step: i64 = options
    .as_ref()
    .and_then(|o| o.get_named_property::<i64>("step").ok())
    .unwrap_or(1);

  if step == 0 {
    return vec![];
  }

  let (Some(mut start), Some(mut end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  let reverse = step < 0 || start > end;
  let abs_step = step.abs();

  if reverse {
    std::mem::swap(&mut start, &mut end);
  }

  let target_weekday = match week_starts_on {
    0 => Weekday::Sunday,
    1 => Weekday::Monday,
    2 => Weekday::Tuesday,
    3 => Weekday::Wednesday,
    4 => Weekday::Thursday,
    5 => Weekday::Friday,
    6 => Weekday::Saturday,
    _ => Weekday::Sunday,
  };
  while start.weekday() != target_weekday {
    start -= Duration::days(1);
  }

  let mut res = vec![];
  start = start.replace_time(time::Time::MIDNIGHT);

  while start <= end {
    res.push(to_ms(start));
    start += Duration::weeks(abs_step);
  }

  if reverse {
    res.reverse();
  }

  res
}

// NOTE ABOUT LOCAL VS UTC
// -----------------------
// This function currently mirrors `date-fns` behavior,
// which works in LOCAL TIME.
//
// Meaning:
//  - Input timestamps are converted using from_ms_local()
//  - Month boundaries are computed in local calendar time
//  - Generated values correspond to *local month start midnight*
//
// This ensures:
//
//   eachMonthOfInterval === date-fns.eachMonthOfInterval
//
// -----------------------
// HOW TO MAKE THIS UTC (future /utc build)
// -----------------------
// 1️ Replace from_ms_local(...) with from_ms(...)
//    -> timestamps stay in UTC
//
// 2️ When constructing month boundaries:
//      Date::from_calendar_date(...)
//    should be considered UTC directly
//
// 3️ You no longer need to preserve or reapply offsets
//
// -----------------------
// TL;DR
// Current: Local calendar months (matches date-fns)
// Future:  Pure UTC calendar months (matches @date-fns/utc)

#[napi]
pub fn each_month_of_interval(start_ms: f64, end_ms: f64, step_opt: Option<i32>) -> Vec<f64> {
  let step = step_opt.unwrap_or(1);
  if step == 0 {
    return vec![];
  }

  let (Some(start), Some(end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  let mut reversed = start > end;
  let end_time = if reversed { start } else { end };
  let date = if reversed { end } else { start };

  let offset = date.offset();

  let mut year = date.year();
  let mut month = date.month();

  let mut step = step;
  if step < 0 {
    step = -step;
    reversed = !reversed;
  }

  let mut res = vec![];

  loop {
    let d = Date::from_calendar_date(year, month, 1).unwrap();
    let dt = d.with_time(time::Time::MIDNIGHT).assume_offset(offset);

    if dt > end_time {
      break;
    }

    res.push(to_ms(dt));

    let idx = month as i32 - 1 + step;
    year += idx.div_euclid(12);
    month = Month::try_from((idx.rem_euclid(12) + 1) as u8).unwrap();
  }

  if reversed {
    res.reverse();
  }

  res
}

#[napi]
pub fn each_quarter_of_interval(start_ms: f64, end_ms: f64, step: Option<i32>) -> Vec<f64> {
  let step = step.unwrap_or(1);
  let (Some(mut start), Some(mut end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  if step == 0 {
    return vec![];
  }

  let reverse = step < 0 || start > end;
  let abs_step = step.abs();
  if reverse {
    std::mem::swap(&mut start, &mut end);
  }

  let mut year = start.year();
  let mut quarter = ((start.month() as i32 - 1) / 3) + 1;

  let mut res = vec![];

  loop {
    let month = Month::try_from(((quarter - 1) * 3 + 1) as u8).unwrap();
    let Ok(d) = start.replace_year(year) else {
      break;
    };
    let Ok(d) = d.replace_month(month) else {
      break;
    };
    let Ok(d) = d.replace_day(1) else {
      break;
    };

    let dt = d.replace_time(time::Time::MIDNIGHT);

    if dt > end {
      break;
    }
    res.push(to_ms(dt));

    quarter += abs_step;
    while quarter > 4 {
      quarter -= 4;
      year += 1;
    }
  }

  if reverse {
    res.reverse();
  }
  res
}

// LOCAL TIME BEHAVIOR
// -------------------
// This function follows `date-fns` semantics,
// which operate entirely in LOCAL TIME.
//
// That means yearly iteration happens like:
//
//   Jan 1st LOCAL MIDNIGHT each year
//
// and depends on the system time zone.
//
// This guarantees test equivalence with:
//
//   date-fns.eachYearOfInterval
//
// -----------------------
// HOW TO CONVERT TO UTC (future /utc lib)
// -----------------------
// 1️ Use from_ms(...) instead of from_ms_local(...)
//    -> read timestamps as UTC
//
// 2️ Construct Jan 1 UTC midnight boundaries
//
// 3️ Remove all reliance on UtcOffset
//
// -----------------------
// TL;DR
// Current: Local-timezone year boundaries
// Future:  Strict UTC year boundaries

#[napi]
pub fn each_year_of_interval(start_ms: f64, end_ms: f64, step_opt: Option<i32>) -> Vec<f64> {
  let step = step_opt.unwrap_or(1);
  if step == 0 {
    return vec![];
  }

  let (Some(start), Some(end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  let mut reversed = start > end;
  let end_time = if reversed { start } else { end };
  let date = if reversed { end } else { start };

  let offset = date.offset();
  let mut year = date.year();

  let mut step = step;
  if step < 0 {
    step = -step;
    reversed = !reversed;
  }

  let mut res = vec![];

  loop {
    let d = Date::from_calendar_date(year, Month::January, 1).unwrap();
    let dt = d.with_time(time::Time::MIDNIGHT).assume_offset(offset);

    if dt > end_time {
      break;
    }

    res.push(to_ms(dt));

    year += step;
  }

  if reversed {
    res.reverse();
  }

  res
}

// NOTE ABOUT TIME ZONES
// ---------------------
// This function intentionally follows the behavior of `date-fns`
// which operates in LOCAL TIME, not UTC.
//
// That means:
//  - We convert timestamps using from_ms_local()
//  - We strip time to midnight LOCAL TIME
//  - Iteration happens at local-day boundaries
//
// This is why the results MATCH `date-fns` but differ from
// `@date-fns/utc`, which treats all dates as UTC.
//
// ---------------------
// HOW TO CONVERT THIS TO UTC (for a future /utc build)
// ---------------------
// 1️ Replace from_ms_local(...) with from_ms(...)
//    -> this removes the local offset
//
// 2️ Normalize to UTC midnight instead of local midnight
//    dt.replace_time(Time::MIDNIGHT) already does this correctly
//    once the OffsetDateTime is UTC
//
// 3️ Remove all UtcOffset handling — it becomes unnecessary
//
// ---------------------
// TL;DR
// Current: LOCAL midnight iteration  == date-fns
// Future:  UTC midnight iteration    == @date-fns/utc

#[napi]
pub fn each_day_of_interval(start_ms: f64, end_ms: f64, step_opt: Option<i64>) -> Vec<f64> {
  let step = step_opt.unwrap_or(1);
  if step == 0 {
    return vec![];
  }

  let (Some(start), Some(end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  let mut reversed = start > end;
  let end_time = if reversed { start } else { end };
  let date = if reversed { end } else { start };

  let offset = date.offset();

  let mut date = date.date();

  let mut step = step;
  if step < 0 {
    step = -step;
    reversed = !reversed;
  }

  let mut res = vec![];

  loop {
    let dt = date.with_time(time::Time::MIDNIGHT).assume_offset(offset);

    if dt > end_time {
      break;
    }

    res.push(to_ms(dt));

    date += time::Duration::days(step);
  }

  if reversed {
    res.reverse();
  }

  res
}

#[napi]
pub fn each_weekend_of_interval(start_ms: f64, end_ms: f64) -> Vec<f64> {
  each_day_of_interval(start_ms, end_ms, None)
    .into_iter()
    .filter(|&ms| {
      from_ms_local(ms).map_or(false, |dt| {
        dt.weekday() == Weekday::Saturday || dt.weekday() == Weekday::Sunday
      })
    })
    .collect()
}

#[napi(object)]
pub struct Interval {
  pub start: f64,
  pub end: f64,
}

#[napi]
pub fn interval_to_daily_intervals(start_ms: f64, end_ms: f64) -> Vec<Interval> {
  let (Some(mut s), Some(mut e)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  let reverse = s > e;
  if reverse {
    std::mem::swap(&mut s, &mut e);
  }

  let mut res = vec![];

  while s < e {
    let next = (s + Duration::days(1)).min(e);
    res.push(Interval {
      start: to_ms(s),
      end: to_ms(next),
    });
    s = next;
  }

  if reverse {
    res.reverse();
  }
  res
}

#[napi]
pub fn parse_iso(iso_string: String) -> f64 {
  let mut string_to_parse = iso_string;
  // Handle date-only strings (YYYY-MM-DD)
  if !string_to_parse.contains('T') && string_to_parse.len() == 10 {
    string_to_parse = format!("{}T00:00:00Z", string_to_parse);
  }
  // Add Z to datetime strings without timezone indicator
  else if string_to_parse.contains('T') && !has_timezone(&string_to_parse) {
    string_to_parse.push('Z');
  }

  match OffsetDateTime::parse(&string_to_parse, &Iso8601::DEFAULT) {
    Ok(dt) => {
      let secs = dt.unix_timestamp() as f64;
      let millis = dt.millisecond() as f64;
      secs * 1000.0 + millis
    }
    Err(_) => f64::NAN,
  }
}

fn has_timezone(s: &str) -> bool {
  let after_t = match s.find('T') {
    Some(pos) => &s[pos..],
    None => return false,
  };

  if after_t.ends_with('Z') {
    return true;
  }

  // Check for +/- timezone offset (must be followed by digit)
  for (i, c) in after_t.chars().enumerate() {
    if (c == '+' || c == '-') && i > 0 {
      // Check if next char is a digit
      if let Some(next) = after_t.chars().nth(i + 1) {
        if next.is_numeric() {
          return true;
        }
      }
    }
  }

  false
}

#[napi]
pub fn start_of_day(timestamp: f64) -> f64 {
  if !timestamp.is_finite() {
    return f64::NAN;
  }

  // Convert milliseconds to seconds and nanoseconds
  let secs = (timestamp / 1000.0).floor() as i64;
  let nanos = ((timestamp % 1000.0) * 1_000_000.0) as i32;

  match OffsetDateTime::from_unix_timestamp_nanos(secs as i128 * 1_000_000_000 + nanos as i128) {
    Ok(dt) => {
      // Get start of day in UTC (00:00:00.000)
      let start = dt.replace_time(time::Time::MIDNIGHT);
      let result_secs = start.unix_timestamp() as f64;
      let result_millis = start.millisecond() as f64;
      result_secs * 1000.0 + result_millis
    }
    Err(_) => f64::NAN,
  }
}

// compareAsc, compareDesc,
// isEqual, isBefore, isAfter,

// isValid,
// isToday,
// isTomorrow,
// isYesterday,
// isThisWeek,
// isThisMonth,
// isThisQuarter,
// isThisYear,
// isLeapYear,
// isWithinInterval,
// areIntervalsOverlapping,

// format,
// formatISO,
// formatISO9075,
// formatRFC3339,
// formatRelative,
// formatDistance,
// formatDistanceStrict,
// formatDistanceToNow,

// parse,
// utcToZonedTime,
// zonedTimeToUtc,…

// startOfSecond, endOfSecond,
// startOfMinute, endOfMinute,
// startOfHour,   endOfHour,
// startOfDay,    endOfDay,
// startOfWeek,   endOfWeek,
// startOfMonth,  endOfMonth,
// startOfQuarter,endOfQuarter,
// startOfYear,   endOfYear

// eachDayOfInterval,
// eachWeekOfInterval,
// eachMonthOfInterval,
// eachQuarterOfInterval,
// eachYearOfInterval,
// isWithinInterval,
// areIntervalsOverlapping

// getDate,
// getDay,
// getDaysInMonth,
// getHours,
// getMilliseconds,
// getMinutes,
// getMonth,
// getQuarter,
// getSeconds,
// getYear,

// intervalToDuration,
// formatDistanceStrict,
// formatDistanceToNowStrict,
