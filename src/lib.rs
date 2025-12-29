#![deny(clippy::all)]

use napi::bindgen_prelude::{Either, JsObjectValue, Object, Undefined};
use napi_derive::napi;
use time::format_description::well_known::Iso8601;
use time::util::days_in_month;
use time::Time;
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset, Weekday};

#[inline(always)]
fn from_ms_local(ms: f64) -> Option<OffsetDateTime> {
  if !ms.is_finite() {
    return None;
  }
  let utc = OffsetDateTime::from_unix_timestamp_nanos((ms * 1_000_000.0) as i128).ok()?;

  let offset = UtcOffset::local_offset_at(utc).ok()?;
  Some(utc.to_offset(offset))
}

#[inline(always)]
fn to_ms(dt: OffsetDateTime) -> f64 {
  dt.unix_timestamp_nanos() as f64 / 1_000_000.0
}

fn add_months_local_internal(ms: f64, amount: i32) -> Option<OffsetDateTime> {
  let dt = from_ms_local(ms)?;
  let mut year = dt.year();
  let month0 = dt.month() as i32 - 1;
  let day = dt.day();

  let total = month0 + amount;
  year += total.div_euclid(12);
  let month_index = total.rem_euclid(12);

  let month = Month::try_from((month_index + 1) as u8).ok()?;

  let dim = days_in_month(month, year);
  let clamped_day = day.min(dim);

  let tmp = dt.replace_day(1).ok()?;
  let tmp = tmp.replace_year(year).ok()?;
  let tmp = tmp.replace_month(month).ok()?;
  tmp.replace_day(clamped_day).ok()
}

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

  match add_months_local_internal(date_ms, amount.trunc() as i32) {
    Some(dt) => to_ms(dt),
    None => f64::NAN,
  }
}

#[napi]
pub fn add_quarters(date_ms: f64, amount: f64) -> f64 {
  add_months(date_ms, amount * 3.0)
}

#[napi]
pub fn add_years(date_ms: f64, amount: f64) -> f64 {
  add_months(date_ms, amount * 12.0)
}

#[napi(object)]
pub struct DurationInput {
  pub years: Option<f64>,
  pub months: Option<f64>,
  pub weeks: Option<f64>,
  pub days: Option<f64>,
  pub hours: Option<f64>,
  pub minutes: Option<f64>,
  pub seconds: Option<f64>,
}

#[napi]
pub fn add(date_ms: f64, duration: DurationInput) -> f64 {
  if !date_ms.is_finite() {
    return f64::NAN;
  }

  let years = duration.years.unwrap_or(0.0).trunc();
  let months = duration.months.unwrap_or(0.0).trunc();
  let weeks = duration.weeks.unwrap_or(0.0).trunc();
  let days = duration.days.unwrap_or(0.0).trunc();
  let hours = duration.hours.unwrap_or(0.0).trunc();
  let minutes = duration.minutes.unwrap_or(0.0).trunc();
  let seconds = duration.seconds.unwrap_or(0.0).trunc();

  let months_total = months + years * 12.0;
  let date_after_months = if months_total != 0.0 {
    add_months(date_ms, months_total)
  } else {
    date_ms
  };

  let days_total = days + weeks * 7.0;
  let date_after_days = if days_total != 0.0 {
    add_days(date_after_months, days_total)
  } else {
    date_after_months
  };

  let minutes_total = minutes + hours * 60.0;
  let seconds_total = seconds + minutes_total * 60.0;
  let ms_total = seconds_total * 1000.0;

  date_after_days + ms_total
}

#[napi]
pub fn add_business_days(date_ms: f64, amount: f64) -> f64 {
  if !date_ms.is_finite() || amount.is_nan() {
    return f64::NAN;
  }

  let Some(dt) = from_ms_local(date_ms) else {
    return f64::NAN;
  };

  let current_weekday_iso = dt.weekday().number_from_monday();

  let mut current_idx = current_weekday_iso - 1;

  let sign = if amount < 0.0 { -1.0 } else { 1.0 };
  let amount_i = amount.trunc() as i32;
  let full_weeks = amount_i / 5;

  let mut result_ms = add_days(date_ms, (full_weeks * 7) as f64);

  let mut rest = (amount_i % 5).abs();

  while rest > 0 {
    result_ms = add_days(result_ms, sign);

    if sign > 0.0 {
      current_idx = (current_idx + 1) % 7;
    } else {
      if current_idx == 0 {
        current_idx = 6;
      } else {
        current_idx -= 1;
      }
    }

    let is_weekend = current_idx >= 5;

    if !is_weekend {
      rest -= 1;
    }
  }

  let started_on_weekend = (current_weekday_iso - 1) >= 5;
  let ended_on_weekend = current_idx >= 5;

  if started_on_weekend && ended_on_weekend && amount_i != 0 {
    if current_idx == 5 {
      result_ms = add_days(result_ms, if sign < 0.0 { 2.0 } else { -1.0 });
    }
    if current_idx == 6 {
      result_ms = add_days(result_ms, if sign < 0.0 { 1.0 } else { -2.0 });
    }
  }

  result_ms
}

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

#[napi]
pub fn difference_in_milliseconds(a_ms: f64, b_ms: f64) -> f64 {
  a_ms - b_ms
}

#[napi]
pub fn difference_in_seconds(a_ms: f64, b_ms: f64) -> i64 {
  ((a_ms - b_ms) / 1000.0) as i64
}

#[napi]
pub fn difference_in_minutes(a_ms: f64, b_ms: f64) -> i64 {
  ((a_ms - b_ms) / 60_000.0) as i64
}

#[napi]
pub fn difference_in_hours(a_ms: f64, b_ms: f64) -> i64 {
  ((a_ms - b_ms) / 3_600_000.0) as i64
}

#[napi]
pub fn difference_in_days(a_ms: f64, b_ms: f64) -> i64 {
  ((a_ms - b_ms) / 86_400_000.0) as i64
}

#[napi]
pub fn difference_in_weeks(a_ms: f64, b_ms: f64) -> i64 {
  ((a_ms - b_ms) / 604_800_000.0) as i64
}

#[napi]
pub fn difference_in_calendar_days(later_ms: f64, earlier_ms: f64) -> f64 {
  if !later_ms.is_finite() || !earlier_ms.is_finite() {
    return f64::NAN;
  }
  let (Some(later), Some(earlier)) = (from_ms_local(later_ms), from_ms_local(earlier_ms)) else {
    return f64::NAN;
  };

  let later_midnight = later
    .date()
    .with_time(time::Time::MIDNIGHT)
    .assume_offset(later.offset());
  let earlier_midnight = earlier
    .date()
    .with_time(time::Time::MIDNIGHT)
    .assume_offset(earlier.offset());

  let later_ts = to_ms(later_midnight);
  let earlier_ts = to_ms(earlier_midnight);

  ((later_ts - earlier_ts) / 86_400_000.0).round()
}

#[napi]
pub fn difference_in_business_days(later_ms: f64, earlier_ms: f64) -> f64 {
  if !later_ms.is_finite() || !earlier_ms.is_finite() {
    return f64::NAN;
  }
  let diff = difference_in_calendar_days(later_ms, earlier_ms);
  if diff == 0.0 {
    return 0.0;
  }

  let Some(earlier_dt) = from_ms_local(earlier_ms) else {
    return f64::NAN;
  };
  let mut current_idx = earlier_dt.weekday().number_from_monday() - 1;

  let sign = if diff < 0.0 { -1.0 } else { 1.0 };
  let weeks = (diff / 7.0).trunc();

  let mut result = weeks * 5.0;

  let remaining_days = (diff % 7.0).abs() as i32;

  for _ in 0..remaining_days {
    if sign > 0.0 {
      let is_weekend = current_idx >= 5;
      if !is_weekend {
        result += 1.0;
      }

      current_idx = (current_idx + 1) % 7;
    } else {
      if current_idx == 0 {
        current_idx = 6;
      } else {
        current_idx -= 1;
      }

      let is_weekend = current_idx >= 5;
      if !is_weekend {
        result -= 1.0;
      }
    }
  }

  if result == 0.0 {
    0.0
  } else {
    result
  }
}

#[napi]
pub fn difference_in_months(a_ms: f64, b_ms: f64) -> f64 {
  let (Some(a), Some(b)) = (from_ms_local(a_ms), from_ms_local(b_ms)) else {
    return f64::NAN;
  };
  let sign = if a >= b { 1 } else { -1 };
  let years = a.year() - b.year();
  let mut months = years * 12 + (a.month() as i32 - b.month() as i32);

  let shifted = add_months_local_internal(b_ms, months)
    .map(to_ms)
    .unwrap_or(f64::NAN);

  if sign > 0 && shifted > a_ms {
    months -= 1;
  } else if sign < 0 && shifted < a_ms {
    months += 1;
  }
  months as f64
}

#[napi]
pub fn difference_in_quarters(a_ms: f64, b_ms: f64) -> i64 {
  (difference_in_months(a_ms, b_ms) / 3.0) as i64
}

#[napi]
pub fn difference_in_years(a_ms: f64, b_ms: f64) -> i64 {
  (difference_in_months(a_ms, b_ms) / 12.0) as i64
}

#[napi]
pub fn is_equal(date_left: f64, date_right: f64) -> bool {
  if date_left.is_nan() || date_right.is_nan() {
    return false;
  }
  if date_left.is_infinite() || date_right.is_infinite() {
    return false;
  }
  date_left == date_right
}

#[napi]
pub fn is_after(date: f64, date_to_compare: f64) -> bool {
  if date.is_nan() || date_to_compare.is_nan() {
    return false;
  }
  if date.is_infinite() || date_to_compare.is_infinite() {
    return false;
  }
  date > date_to_compare
}

#[napi]
pub fn is_before(date: f64, date_to_compare: f64) -> bool {
  if date.is_nan() || date_to_compare.is_nan() {
    return false;
  }
  if date.is_infinite() || date_to_compare.is_infinite() {
    return false;
  }
  date < date_to_compare
}

#[napi]
pub fn compare_asc(date_left: f64, date_right: f64) -> f64 {
  if date_left.is_nan() || date_right.is_nan() {
    return f64::NAN;
  }
  if date_left.is_infinite() || date_right.is_infinite() {
    return f64::NAN;
  }
  if date_left < date_right {
    -1.0
  } else if date_left > date_right {
    1.0
  } else {
    0.0
  }
}

#[napi]
pub fn compare_desc(date_left: f64, date_right: f64) -> f64 {
  if date_left.is_nan() || date_right.is_nan() {
    return f64::NAN;
  }
  if date_left.is_infinite() || date_right.is_infinite() {
    return f64::NAN;
  }
  if date_left > date_right {
    -1.0
  } else if date_left < date_right {
    1.0
  } else {
    0.0
  }
}

#[napi]
pub fn clamp(date_ms: f64, start_ms: f64, end_ms: f64) -> f64 {
  if !date_ms.is_finite() || !start_ms.is_finite() || !end_ms.is_finite() {
    return f64::NAN;
  }
  let (start, end) = if start_ms <= end_ms {
    (start_ms, end_ms)
  } else {
    (end_ms, start_ms)
  };
  if date_ms < start {
    start
  } else if date_ms > end {
    end
  } else {
    date_ms
  }
}

#[napi]
pub fn min(dates: Vec<f64>) -> f64 {
  if dates.is_empty() {
    return f64::NAN;
  }
  if dates.iter().any(|d| d.is_nan()) {
    return f64::NAN;
  }
  dates.into_iter().fold(f64::INFINITY, f64::min)
}

#[napi]
pub fn max(dates: Vec<f64>) -> f64 {
  if dates.is_empty() {
    return f64::NAN;
  }
  if dates.iter().any(|d| d.is_nan()) {
    return f64::NAN;
  }
  dates.into_iter().fold(f64::NEG_INFINITY, f64::max)
}

enum SearchResult {
  Found(usize),
  InvalidData,
  Empty,
}

fn find_closest_idx_internal(target_ms: f64, dates: &[f64]) -> SearchResult {
  if !target_ms.is_finite() {
    return SearchResult::InvalidData;
  }
  if dates.is_empty() {
    return SearchResult::Empty;
  }
  let mut best_index: Option<usize> = None;
  let mut best_distance = f64::MAX;

  for (i, ts) in dates.iter().enumerate() {
    if !ts.is_finite() {
      return SearchResult::InvalidData;
    }
    let dist = (target_ms - ts).abs();
    if best_index.is_none() || dist < best_distance {
      best_index = Some(i);
      best_distance = dist;
    }
  }
  match best_index {
    Some(i) => SearchResult::Found(i),
    None => SearchResult::Empty,
  }
}

#[napi]
pub fn closest_index_to(target_ms: f64, dates: Vec<f64>) -> Either<f64, Undefined> {
  match find_closest_idx_internal(target_ms, &dates) {
    SearchResult::Found(idx) => Either::A(idx as f64),
    SearchResult::InvalidData => Either::A(f64::NAN),
    SearchResult::Empty => Either::B(()),
  }
}

#[napi]
pub fn closest_to(target_ms: f64, dates: Vec<f64>) -> Either<f64, Undefined> {
  match find_closest_idx_internal(target_ms, &dates) {
    SearchResult::Found(idx) => Either::A(dates[idx]),
    SearchResult::InvalidData => Either::A(f64::NAN),
    SearchResult::Empty => Either::B(()),
  }
}

#[napi]
pub fn are_intervals_overlapping(
  left_start: f64,
  left_end: f64,
  right_start: f64,
  right_end: f64,
  inclusive: Option<bool>,
) -> bool {
  if !left_start.is_finite()
    || !left_end.is_finite()
    || !right_start.is_finite()
    || !right_end.is_finite()
  {
    return false;
  }
  let (l1, l2) = if left_start <= left_end {
    (left_start, left_end)
  } else {
    (left_end, left_start)
  };
  let (r1, r2) = if right_start <= right_end {
    (right_start, right_end)
  } else {
    (right_end, right_start)
  };

  if inclusive.unwrap_or(false) {
    l1 <= r2 && r1 <= l2
  } else {
    l1 < r2 && r1 < l2
  }
}

#[napi]
pub fn is_weekend(date_ms: f64) -> bool {
  if !date_ms.is_finite() {
    return false;
  }
  let dt = from_ms_local(date_ms).unwrap_or(OffsetDateTime::UNIX_EPOCH);
  matches!(dt.weekday(), Weekday::Saturday | Weekday::Sunday)
}

#[napi]
pub fn is_saturday(date_ms: f64) -> bool {
  if !date_ms.is_finite() {
    return false;
  }
  let dt = from_ms_local(date_ms).unwrap_or(OffsetDateTime::UNIX_EPOCH);
  dt.weekday() == Weekday::Saturday
}

#[napi]
pub fn is_sunday(date_ms: f64) -> bool {
  if !date_ms.is_finite() {
    return false;
  }
  let dt = from_ms_local(date_ms).unwrap_or(OffsetDateTime::UNIX_EPOCH);
  dt.weekday() == Weekday::Sunday
}

#[napi]
pub fn is_same_day(a_ms: f64, b_ms: f64) -> bool {
  let (Some(a), Some(b)) = (from_ms_local(a_ms), from_ms_local(b_ms)) else {
    return false;
  };
  a.date() == b.date()
}

#[napi(object)]
pub struct DurationParts {
  pub years: i32,
  pub months: i32,
  pub days: i32,
  pub hours: i32,
  pub minutes: i32,
  pub seconds: i32,
}

#[napi]
pub fn interval_to_duration(start_ms: f64, end_ms: f64) -> DurationParts {
  let (Some(mut start), Some(mut end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return DurationParts {
      years: 0,
      months: 0,
      days: 0,
      hours: 0,
      minutes: 0,
      seconds: 0,
    };
  };
  if start > end {
    std::mem::swap(&mut start, &mut end);
  }

  let years = difference_in_years(to_ms(end), to_ms(start)) as i32;
  let after_years = add_years(to_ms(start), years as f64);

  let months = difference_in_months(to_ms(end), after_years) as i32;
  let after_months = add_months(after_years, months as f64);

  let days = difference_in_days(to_ms(end), after_months) as i32;
  let after_days = add_days(after_months, days as f64);

  let hours = difference_in_hours(to_ms(end), after_days) as i32;
  let after_hours = add_hours(after_days, hours as f64);

  let minutes = difference_in_minutes(to_ms(end), after_hours) as i32;
  let after_minutes = add_minutes(after_hours, minutes as f64);

  let seconds = difference_in_seconds(to_ms(end), after_minutes) as i32;

  DurationParts {
    years,
    months,
    days,
    hours,
    minutes,
    seconds,
  }
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
  let date_obj = if reversed { end } else { start };

  let offset = date_obj.offset();
  let mut date = date_obj.date();

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
    date += Duration::days(step);
  }
  if reversed {
    res.reverse();
  }
  res
}

#[napi]
pub fn each_weekend_of_interval(start_ms: f64, end_ms: f64) -> Vec<f64> {
  let (Some(start), Some(end)) = (from_ms_local(start_ms), from_ms_local(end_ms)) else {
    return vec![];
  };

  let mut reversed = start > end;
  let end_time = if reversed { start } else { end };
  let date_obj = if reversed { end } else { start };

  let offset = date_obj.offset();
  let mut date = date_obj.date();

  let mut res = vec![];
  loop {
    let dt = date.with_time(time::Time::MIDNIGHT).assume_offset(offset);
    if dt > end_time {
      break;
    }

    let wd = dt.weekday();
    if wd == Weekday::Saturday || wd == Weekday::Sunday {
      res.push(to_ms(dt));
    }

    date += Duration::days(1);
  }
  if reversed {
    res.reverse();
  }
  res
}

#[napi]
pub fn each_week_of_interval(start_ms: f64, end_ms: f64, options: Option<Object>) -> Vec<f64> {
  let week_starts_on: u8 = options
    .as_ref()
    .and_then(|o| o.get_named_property("weekStartsOn").ok())
    .unwrap_or(0);
  let step: i64 = options
    .as_ref()
    .and_then(|o| o.get_named_property("step").ok())
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

#[napi]
pub fn each_quarter_of_interval(start_ms: f64, end_ms: f64, step_opt: Option<i32>) -> Vec<f64> {
  let step = step_opt.unwrap_or(1);
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

  let mut year = start.year();
  let mut quarter = ((start.month() as i32 - 1) / 3) + 1;
  let mut res = vec![];

  loop {
    let month = Month::try_from(((quarter - 1) * 3 + 1) as u8).unwrap();
    let d = Date::from_calendar_date(year, month, 1).unwrap();
    let dt = d
      .with_time(time::Time::MIDNIGHT)
      .assume_offset(start.offset());

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

#[napi]
pub fn start_of_month(date_ms: f64) -> f64 {
  if !date_ms.is_finite() {
    return f64::NAN;
  }

  let nanos = (date_ms * 1_000_000.0) as i128;

  let Ok(dt) = OffsetDateTime::from_unix_timestamp_nanos(nanos) else {
    return f64::NAN;
  };

  match dt.replace_day(1) {
    Ok(start_of_month_date) => {
      let start_midnight = start_of_month_date.replace_time(Time::MIDNIGHT);

      (start_midnight.unix_timestamp_nanos() / 1_000_000) as f64
    }
    Err(_) => f64::NAN,
  }
}

#[napi]
pub fn end_of_month(date_ms: f64) -> f64 {
  if !date_ms.is_finite() {
    return f64::NAN;
  }

  let nanos = (date_ms * 1_000_000.0) as i128;
  let Ok(dt) = time::OffsetDateTime::from_unix_timestamp_nanos(nanos) else {
    return f64::NAN;
  };

  let year = dt.year();
  let month = dt.month();
  let days = time::util::days_in_month(month, year);

  match dt.replace_day(days) {
    Ok(last_day) => {
      let end_time = time::Time::from_hms_milli(23, 59, 59, 999).unwrap();
      let end = last_day.replace_time(end_time);
      (end.unix_timestamp_nanos() / 1_000_000) as f64
    }
    Err(_) => f64::NAN,
  }
}

#[napi]
pub fn last_day_of_month(date_ms: f64) -> f64 {
  if !date_ms.is_finite() {
    return f64::NAN;
  }

  let nanos = (date_ms * 1_000_000.0) as i128;
  let Ok(dt) = time::OffsetDateTime::from_unix_timestamp_nanos(nanos) else {
    return f64::NAN;
  };

  let year = dt.year();
  let month = dt.month();
  let days = time::util::days_in_month(month, year);

  match dt.replace_day(days) {
    Ok(last_day) => (last_day.unix_timestamp_nanos() / 1_000_000) as f64,
    Err(_) => f64::NAN,
  }
}
