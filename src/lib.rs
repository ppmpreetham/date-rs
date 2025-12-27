#![deny(clippy::all)]

use napi_derive::napi;
use time::util::{days_in_year, days_in_year_month};
use time::{Duration, Month, OffsetDateTime};

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

  // truncate toward zero — same as date-fns
  let amount_i32 = amount.trunc() as i32;

  let total = month0 + amount_i32;

  year += total.div_euclid(12);
  let month_index = total.rem_euclid(12);

  let Ok(month) = Month::try_from((month_index + 1) as u8) else {
    return f64::NAN;
  };

  let days = days_in_year_month(year, month);
  let clamped_day = day.min(days as u8);

  let Ok(new_date) = date
    .replace_year(year)
    .and_then(|d| d.replace_month(month))
    .and_then(|d| d.replace_day(clamped_day))
  else {
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

#[napi]
pub fn difference_in_months(a_ms: f64, b_ms: f64) -> i64 {
  let (Some(a), Some(b)) = (from_ms(a_ms), from_ms(b_ms)) else {
    return 0;
  };

  let sign = if a_ms >= b_ms { 1 } else { -1 };

  let years = a.year() - b.year();
  let months = years * 12 + (a.month() as i32 - b.month() as i32);

  let shifted = add_months(b_ms, months as f64);

  let corrected = if (shifted > a_ms) != (sign > 0) {
    months - sign
  } else {
    months
  };

  corrected as i64
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
// parseISO,
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
