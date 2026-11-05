//! Pure dated contributions and bounded week/day geometry.
use crate::dates::{CivilDate, Weekday};
use std::collections::BTreeMap;
pub const MAX_HEATMAP_DAYS: usize = 366;
pub const MAX_HEATMAP_VALUES: usize = 366;
const MAX_VALUE: f64 = 1_000_000_000_000.;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatmapError {
    Range,
    Budget,
    Value,
    Duplicate,
}
impl std::fmt::Display for HeatmapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Range => "heatmap range must contain 1–366 civil days",
            Self::Budget => "heatmap accepts at most 366 dated values",
            Self::Value => "heatmap values and domain must be finite and bounded",
            Self::Duplicate => "heatmap date declarations must be unique",
        })
    }
}
impl std::error::Error for HeatmapError {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeatmapRange {
    start: CivilDate,
    end: CivilDate,
}
impl HeatmapRange {
    pub fn new(start: CivilDate, end: CivilDate) -> Result<Self, HeatmapError> {
        let count = start.days_until(end);
        if count < 0 || count >= MAX_HEATMAP_DAYS as i32 {
            return Err(HeatmapError::Range);
        }
        Ok(Self { start, end })
    }
    pub const fn start(self) -> CivilDate {
        self.start
    }
    pub const fn end(self) -> CivilDate {
        self.end
    }
    pub fn len(self) -> usize {
        self.start.days_until(self.end) as usize + 1
    }
    pub const fn is_empty(self) -> bool {
        false
    }
    pub fn contains(self, date: CivilDate) -> bool {
        date >= self.start && date <= self.end
    }
    pub fn date(self, index: usize) -> Option<CivilDate> {
        if index >= self.len() {
            None
        } else {
            self.start.checked_add_days(index as i32).ok()
        }
    }
    pub fn weeks(self, first: Weekday) -> usize {
        (self.start.weekday().index_from(first) as usize + self.len()).div_ceil(7)
    }
    pub fn position(self, date: CivilDate, first: Weekday) -> Option<(usize, usize)> {
        if !self.contains(date) {
            return None;
        }
        let offset =
            self.start.weekday().index_from(first) as usize + self.start.days_until(date) as usize;
        Some((offset / 7, offset % 7))
    }
    pub fn at(self, week: usize, row: usize, first: Weekday) -> Option<CivilDate> {
        if week >= self.weeks(first) || row >= 7 {
            return None;
        }
        let index = (week * 7 + row) as i32 - i32::from(self.start.weekday().index_from(first));
        if index < 0 {
            return None;
        }
        self.date(index as usize)
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatmapDomain {
    min: f64,
    max: f64,
}
impl HeatmapDomain {
    pub fn new(min: f64, max: f64) -> Result<Self, HeatmapError> {
        if !valid(min) || !valid(max) || min > max {
            return Err(HeatmapError::Value);
        }
        Ok(Self { min, max })
    }
    pub const fn min(self) -> f64 {
        self.min
    }
    pub const fn max(self) -> f64 {
        self.max
    }
    pub fn level(self, value: f64, colors: usize) -> Option<usize> {
        if colors == 0 || !valid(value) {
            return None;
        }
        if self.min == self.max {
            return Some(0);
        }
        Some((((value - self.min) / (self.max - self.min)).clamp(0., 1.) * colors as f64) as usize)
            .map(|level| level.min(colors - 1))
    }
}
fn valid(value: f64) -> bool {
    value.is_finite() && value.abs() <= MAX_VALUE
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeatmapData {
    values: BTreeMap<CivilDate, f64>,
}
impl HeatmapData {
    pub fn new(values: impl IntoIterator<Item = (CivilDate, f64)>) -> Result<Self, HeatmapError> {
        let mut data = Self::default();
        for (index, (date, value)) in values.into_iter().take(MAX_HEATMAP_VALUES + 1).enumerate() {
            if index == MAX_HEATMAP_VALUES {
                return Err(HeatmapError::Budget);
            }
            if data.values.contains_key(&date) {
                return Err(HeatmapError::Duplicate);
            }
            data.set(date, Some(value))?;
        }
        Ok(data)
    }
    pub fn value(&self, date: CivilDate) -> Option<f64> {
        self.values.get(&date).copied()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = (CivilDate, f64)> + '_ {
        self.values.iter().map(|(date, value)| (*date, *value))
    }
    pub fn set(&mut self, date: CivilDate, value: Option<f64>) -> Result<(), HeatmapError> {
        if let Some(value) = value {
            if !valid(value) {
                return Err(HeatmapError::Value);
            }
            if self.values.len() == MAX_HEATMAP_VALUES && !self.values.contains_key(&date) {
                return Err(HeatmapError::Budget);
            }
            self.values.insert(date, value);
        } else {
            self.values.remove(&date);
        }
        Ok(())
    }
    pub fn domain(&self, range: HeatmapRange) -> HeatmapDomain {
        let mut domain: Option<(f64, f64)> = None;
        for (_, value) in self.values.range(range.start..=range.end) {
            domain = Some(domain.map_or((*value, *value), |(min, max)| {
                (min.min(*value), max.max(*value))
            }));
        }
        let (min, max) = domain.unwrap_or((0., 1.));
        HeatmapDomain { min, max }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn d(s: &str) -> CivilDate {
        CivilDate::parse_iso(s).unwrap()
    }
    #[test]
    fn every_date_has_one_weekday_position_including_leap_and_civil_edges() {
        for (start, end) in [
            (d("2024-01-01"), d("2024-12-31")),
            (
                CivilDate::MIN,
                CivilDate::MIN.checked_add_days(365).unwrap(),
            ),
            (
                CivilDate::MAX.checked_add_days(-365).unwrap(),
                CivilDate::MAX,
            ),
        ] {
            let range = HeatmapRange::new(start, end).unwrap();
            for first in Weekday::ALL {
                assert!(range.weeks(first) <= 54);
                let mut count = 0;
                for week in 0..range.weeks(first) {
                    for row in 0..7 {
                        if let Some(date) = range.at(week, row, first) {
                            assert_eq!(range.position(date, first), Some((week, row)));
                            assert_eq!(date.weekday().index_from(first), row as u8);
                            count += 1;
                        }
                    }
                }
                assert_eq!(count, range.len());
            }
        }
        assert!(HeatmapRange::new(d("2024-01-01"), d("2025-01-01")).is_err());
        assert!(HeatmapRange::new(CivilDate::MAX, CivilDate::MIN).is_err());
    }
    #[test]
    fn sparse_missing_zero_domains_and_atomic_value_rejection_remain_distinct() {
        let first = d("2024-02-28");
        let leap = d("2024-02-29");
        let range = HeatmapRange::new(first, leap).unwrap();
        let mut data = HeatmapData::new([(first, 0.)]).unwrap();
        assert_eq!(data.value(first), Some(0.));
        assert_eq!(data.value(leap), None);
        let old = data.clone();
        assert!(data.set(first, Some(f64::NAN)).is_err());
        assert_eq!(data, old);
        data.set(leap, Some(10.)).unwrap();
        let domain = data.domain(range);
        assert_eq!((domain.min(), domain.max()), (0., 10.));
        assert_eq!(domain.level(-20., 4), Some(0));
        assert_eq!(domain.level(10., 4), Some(3));
        assert_eq!(domain.level(2.5, 4), Some(1));
        assert_eq!(HeatmapDomain::new(2., 2.).unwrap().level(2., 4), Some(0));
        assert!(HeatmapDomain::new(3., 2.).is_err());
        assert!(HeatmapData::new([(first, 1.), (first, 2.)]).is_err());
        let mut reads = 0;
        assert!(
            HeatmapData::new(std::iter::repeat_with(|| {
                reads += 1;
                (first, 1.)
            }))
            .is_err()
        );
        assert_eq!(reads, 2);
        let mut reads = 0;
        assert!(
            HeatmapData::new((0..).map(|i| {
                reads += 1;
                (CivilDate::MIN.checked_add_days(i).unwrap(), 1.)
            }))
            .is_err()
        );
        assert_eq!(reads, MAX_HEATMAP_VALUES + 1);
    }
}
