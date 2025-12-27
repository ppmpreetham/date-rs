#![deny(clippy::all)]

use napi::JsObject as Object;
use napi_derive::napi;
use time::util::{days_in_year, days_in_year_month};
use time::{Duration, Month, OffsetDateTime, Weekday};

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

fn to_ms(dt: OffsetDateTime) -> f64 {
  dt.unix_timestamp_nanos() as f64 / 1_000_000.0
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
  let (Some(mut start), Some(mut end)) = (from_ms(start_ms), from_ms(end_ms)) else {
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
pub fn each_day_of_interval(start_ms: f64, end_ms: f64, step: Option<i64>) -> Vec<f64> {
  let step = step.unwrap_or(1);
  let (Some(mut start), Some(mut end)) = (from_ms(start_ms), from_ms(end_ms)) else {
    return vec![];
  };

  let reverse = step < 0 || start > end;
  let abs_step = step.abs() as i64;
  if reverse {
    std::mem::swap(&mut start, &mut end);
  }

  let mut res = vec![];
  start = start.replace_time(time::Time::MIDNIGHT);
  end = end.replace_time(time::Time::MIDNIGHT);

  while start <= end {
    res.push(to_ms(start));
    start += Duration::days(abs_step);
  }

  if reverse {
    res.reverse();
  }
  res
}

#[napi]
pub fn each_week_of_interval(start_ms: f64, end_ms: f64, options: Option<Object>) -> Vec<f64> {
  let week_starts_on = options
    .as_ref()
    .and_then(|o| o.get_named_property::<u8>("weekStartsOn").ok())
    .unwrap_or(0);
  let step = options
    .as_ref()
    .and_then(|o| o.get_named_property::<i64>("step").ok())
    .unwrap_or(1);

  let (Some(mut start), Some(mut end)) = (from_ms(start_ms), from_ms(end_ms)) else {
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

  let target_weekday = Weekday::try_from(week_starts_on).unwrap_or(Weekday::Sunday);
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
#[napi]
pub fn each_month_of_interval(start_ms: f64, end_ms: f64, step_opt: Option<i32>) -> Vec<f64> {
  let step = step_opt.unwrap_or(1);
  if step == 0 {
    return vec![];
  }

  let (Some(mut start), Some(mut end)) = (from_ms(start_ms), from_ms(end_ms)) else {
    return vec![];
  };

  let mut reversed = start > end;
  let end_time = if reversed { start } else { end };
  let mut date = if reversed { end } else { start };

  if step < 0 {
    reversed = !reversed;
  }
  let step = step.abs();

  date = date
    .replace_time(time::Time::MIDNIGHT)
    .replace_day(1)
    .unwrap();

  let mut res = vec![];

  while date <= end_time {
    res.push(to_ms(date));
    date = date + Duration::days(0);
    let next_month = date.month() as i32 - 1 + step;
    let mut year = date.year() + next_month.div_euclid(12);
    let month = Month::try_from((next_month.rem_euclid(12) + 1) as u8).unwrap();
    date = date
      .replace_year(year)
      .and_then(|d| d.replace_month(month))
      .and_then(|d| d.replace_day(1))
      .unwrap();
  }

  if reversed {
    res.reverse();
  }

  res
}

#[napi]
pub fn each_quarter_of_interval(start_ms: f64, end_ms: f64, step: Option<i32>) -> Vec<f64> {
  let step = step.unwrap_or(1);
  let (Some(mut start), Some(mut end)) = (from_ms(start_ms), from_ms(end_ms)) else {
    return vec![];
  };

  if step == 0 {
    return vec![];
  }

  let reverse = step < 0 || start > end;
  let abs_step = step.abs() as i32;
  if reverse {
    std::mem::swap(&mut start, &mut end);
  }

  let mut year = start.year();
  let mut quarter = ((start.month() as i32 - 1) / 3) + 1;

  let mut res = vec![];

  loop {
    let month = Month::try_from(((quarter - 1) * 3 + 1) as u8).unwrap();
    let Ok(dt) = start
      .replace_year(year)
      .and_then(|d| d.replace_month(month))
      .and_then(|d| d.replace_day(1))
      .and_then(|d| d.replace_time(time::Time::MIDNIGHT))
    else {
      break;
    };

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

#[napi]
pub fn each_year_of_interval(start_ms: f64, end_ms: f64, step: Option<i32>) -> Vec<f64> {
  let step = step.unwrap_or(1);
  let (Some(start), Some(end)) = (from_ms(start_ms), from_ms(end_ms)) else {
    return vec![];
  };

  let reverse = step < 0 || start > end;
  let abs_step = step.abs() as i32;
  if reverse {
    let temp = start;
    start = end;
    end = temp;
  }

  let mut year = start.year();

  let mut res = vec![];

  loop {
    let Ok(dt) = start
      .replace_year(year)
      .and_then(|d| d.replace_month(Month::January))
      .and_then(|d| d.replace_day(1))
      .and_then(|d| d.replace_time(time::Time::MIDNIGHT))
    else {
      break;
    };
    if dt > end {
      break;
    }
    res.push(to_ms(dt));
    year += abs_step;
  }

  if reverse {
    res.reverse();
  }
  res
}

#[napi]
pub fn each_weekend_of_interval(start_ms: f64, end_ms: f64) -> Vec<f64> {
  each_day_of_interval(start_ms, end_ms, None)
    .into_iter()
    .filter(|&d| {
      from_ms(d).unwrap().weekday() == Weekday::Saturday
        || from_ms(d).unwrap().weekday() == Weekday::Sunday
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
  let (Some(mut s), Some(mut e)) = (from_ms(start_ms), from_ms(end_ms)) else {
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
