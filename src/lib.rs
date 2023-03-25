#[cfg(test)]
use rstest_reuse;

#[derive(Debug, Eq, PartialEq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn ymd(y: i32, m: u32, d: u32) -> Self {
        Date {
            year: y,
            month: m,
            day: d,
        }
    }
}

const OUT_OF_RANGE_ERR: &'static str = "Computus is only valid from 1583 to 9999";

/// Easter in the Gregorian calendar
pub fn gregorian(year: i32) -> Result<Date, &'static str> {
    if year < 1583 || year > 9999 {
        return Err(OUT_OF_RANGE_ERR);
    }
    let aa = year % 19;
    let bb = year / 100;
    let cc = year % 100;
    let dd = bb / 4;
    let ee = bb % 4;
    let ff = (bb + 8) / 25;
    let gg = (bb - ff + 1) / 3;
    let hh = (19 * aa + bb - dd - gg + 15) % 30;
    let ii = cc / 4;
    let kk = cc % 4;
    let ll = (32 + 2 * ee + 2 * ii - hh - kk) % 7;
    let mm = (aa + 11 * hh + 22 * ll) / 451;
    let month = (hh + ll - 7 * mm + 114) / 31;
    let day = (hh + ll - 7 * mm + 114) % 31 + 1;
    Ok(Date::ymd(year, month as u32, day as u32))
}

/// Easter in the Julian calendar
pub fn julian(year: i32) -> Result<Date, &'static str> {
    if year < 1583 || year > 9999 {
        return Err(OUT_OF_RANGE_ERR);
    }
    let aa = year % 4;
    let bb = year % 7;
    let cc = year % 19;
    let dd = (19 * cc + 15) % 30;
    let ee = (2 * aa + 4 * bb - dd + 34) % 7;
    let ff = dd + ee + 114;
    let month = ff / 31;
    let day = ff % 31 + 1;
    Ok(Date::ymd(year, month as u32, day as u32))
}

/// Easter in the Gregorian calendar. Requires `chrono` feature to provide a `chrono::NaiveDate`.
#[cfg(feature = "chrono")]
pub fn gregorian_naive(year: i32) -> Result<chrono::NaiveDate, &'static str> {
    let Date { year, month, day } = gregorian(year)?;
    chrono::NaiveDate::from_ymd_opt(year, month, day).ok_or("invalid date")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use rstest_reuse::{apply, template};

    #[template]
    #[rstest]
    #[case(1961, 4, 2)]
    #[case(1996, 4, 7)]
    #[case(1997, 3, 30)]
    #[case(1998, 4, 12)]
    #[case(2000, 4, 23)]
    #[case(2001, 4, 15)]
    #[case(2002, 3, 31)]
    #[case(2003, 4, 20)]
    #[case(2004, 4, 11)]
    #[case(2005, 3, 27)]
    #[case(2006, 4, 16)]
    #[case(2007, 4, 8)]
    #[case(2008, 3, 23)]
    #[case(2009, 4, 12)]
    #[case(2010, 4, 4)]
    #[case(2011, 4, 24)]
    #[case(2012, 4, 8)]
    #[case(2013, 3, 31)]
    #[case(2014, 4, 20)]
    #[case(2015, 4, 5)]
    #[case(2016, 3, 27)]
    #[case(2017, 4, 16)]
    #[case(2018, 4, 1)]
    #[case(2019, 4, 21)]
    #[case(2020, 4, 12)]
    #[case(2021, 4, 4)]
    #[case(2022, 4, 17)]
    #[case(3035, 4, 19)]
    #[case(4000, 4, 9)]
    #[case(9999, 3, 28)]
    fn gregorian_data(#[case] y: i32, #[case] m: u32, #[case] d: u32) {}

    #[apply(gregorian_data)]
    fn gregorian_month_day(y: i32, m: u32, d: u32) {
        use super::{gregorian, Date};
        assert_eq!(gregorian(y), Ok(Date::ymd(y, m, d)));
    }

    #[cfg(feature = "chrono")]
    #[apply(gregorian_data)]
    fn gregorian_naive(y: i32, m: u32, d: u32) {
        assert_eq!(
            super::gregorian_naive(y),
            Ok(chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap())
        );
    }

    #[rstest]
    #[case(1961, 3, 27)]
    #[case(1996, 4, 1)]
    #[case(1997, 4, 14)]
    #[case(1998, 4, 6)]
    #[case(1999, 3, 29)]
    #[case(2000, 4, 17)]
    #[case(2001, 4, 2)]
    #[case(2002, 4, 22)]
    #[case(2003, 4, 14)]
    #[case(2004, 3, 29)]
    #[case(2005, 4, 18)]
    #[case(2006, 4, 10)]
    #[case(2007, 3, 26)]
    #[case(2008, 4, 14)]
    #[case(2009, 4, 6)]
    #[case(2010, 3, 22)]
    #[case(2011, 4, 11)]
    #[case(2012, 4, 2)]
    #[case(2013, 4, 22)]
    #[case(2014, 4, 7)]
    #[case(2015, 3, 30)]
    #[case(2016, 4, 18)]
    #[case(2017, 4, 3)]
    #[case(2018, 3, 26)]
    #[case(2019, 4, 15)]
    #[case(2020, 4, 6)]
    fn julian_month_day(#[case] y: i32, #[case] m: u32, #[case] d: u32) {
        use super::{julian, Date};
        assert_eq!(julian(y), Ok(Date::ymd(y, m, d)));
    }
}
