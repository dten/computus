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

pub mod gregorian {
    /// Easter in the Gregorian calendar
    pub fn month_day(year: i32) -> Result<::Date, &'static str> {
        if year < 1583 || year > 9999 {
            return Err(::OUT_OF_RANGE_ERR);
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
        Ok(::Date::ymd(year, month as u32, day as u32))
    }
}

pub mod julian {
    /// Easter in the Julian calendar
    pub fn month_day(year: i32) -> Result<::Date, &'static str> {
        if year < 1583 || year > 9999 {
            return Err(::OUT_OF_RANGE_ERR);
        }
        let aa = year % 4;
        let bb = year % 7;
        let cc = year % 19;
        let dd = (19 * cc + 15) % 30;
        let ee = (2 * aa + 4 * bb - dd + 34) % 7;
        let ff = dd + ee + 114;
        let month = ff / 31;
        let day = ff % 31 + 1;
        Ok(::Date::ymd(year, month as u32, day as u32))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gregorian_month_day() {
        use ::Date;
        use super::gregorian::month_day as gregorian;
        assert_eq!(gregorian(1961), Ok(Date::ymd(1961, 4, 2)));
        assert_eq!(gregorian(1996), Ok(Date::ymd(1996, 4, 7)));
        assert_eq!(gregorian(1997), Ok(Date::ymd(1997, 3, 30)));
        assert_eq!(gregorian(1998), Ok(Date::ymd(1998, 4, 12)));
        assert_eq!(gregorian(2000), Ok(Date::ymd(2000, 4, 23)));
        assert_eq!(gregorian(2001), Ok(Date::ymd(2001, 4, 15)));
        assert_eq!(gregorian(2002), Ok(Date::ymd(2002, 3, 31)));
        assert_eq!(gregorian(2003), Ok(Date::ymd(2003, 4, 20)));
        assert_eq!(gregorian(2004), Ok(Date::ymd(2004, 4, 11)));
        assert_eq!(gregorian(2005), Ok(Date::ymd(2005, 3, 27)));
        assert_eq!(gregorian(2006), Ok(Date::ymd(2006, 4, 16)));
        assert_eq!(gregorian(2007), Ok(Date::ymd(2007, 4, 8)));
        assert_eq!(gregorian(2008), Ok(Date::ymd(2008, 3, 23)));
        assert_eq!(gregorian(2009), Ok(Date::ymd(2009, 4, 12)));
        assert_eq!(gregorian(2010), Ok(Date::ymd(2010, 4, 4)));
        assert_eq!(gregorian(2011), Ok(Date::ymd(2011, 4, 24)));
        assert_eq!(gregorian(2012), Ok(Date::ymd(2012, 4, 8)));
        assert_eq!(gregorian(2013), Ok(Date::ymd(2013, 3, 31)));
        assert_eq!(gregorian(2014), Ok(Date::ymd(2014, 4, 20)));
        assert_eq!(gregorian(2015), Ok(Date::ymd(2015, 4, 5)));
        assert_eq!(gregorian(2016), Ok(Date::ymd(2016, 3, 27)));
        assert_eq!(gregorian(2017), Ok(Date::ymd(2017, 4, 16)));
        assert_eq!(gregorian(2018), Ok(Date::ymd(2018, 4, 1)));
        assert_eq!(gregorian(2019), Ok(Date::ymd(2019, 4, 21)));
        assert_eq!(gregorian(2020), Ok(Date::ymd(2020, 4, 12)));
        assert_eq!(gregorian(2021), Ok(Date::ymd(2021, 4, 4)));
        assert_eq!(gregorian(2022), Ok(Date::ymd(2022, 4, 17)));
        assert_eq!(gregorian(3035), Ok(Date::ymd(3035, 4, 19)));
        assert_eq!(gregorian(4000), Ok(Date::ymd(4000, 4, 9)));
        assert_eq!(gregorian(9999), Ok(Date::ymd(9999, 3, 28)));
    }

    #[test]
    fn julian_month_day() {
        use ::Date;
        use super::julian::month_day as julian;
        assert_eq!(julian(1961), Ok(Date::ymd(1961, 3, 27)));
        assert_eq!(julian(1996), Ok(Date::ymd(1996, 4, 1)));
        assert_eq!(julian(1997), Ok(Date::ymd(1997, 4, 14)));
        assert_eq!(julian(1998), Ok(Date::ymd(1998, 4, 6)));
        assert_eq!(julian(1999), Ok(Date::ymd(1999, 3, 29)));
        assert_eq!(julian(2000), Ok(Date::ymd(2000, 4, 17)));
        assert_eq!(julian(2001), Ok(Date::ymd(2001, 4, 2)));
        assert_eq!(julian(2002), Ok(Date::ymd(2002, 4, 22)));
        assert_eq!(julian(2003), Ok(Date::ymd(2003, 4, 14)));
        assert_eq!(julian(2004), Ok(Date::ymd(2004, 3, 29)));
        assert_eq!(julian(2005), Ok(Date::ymd(2005, 4, 18)));
        assert_eq!(julian(2006), Ok(Date::ymd(2006, 4, 10)));
        assert_eq!(julian(2007), Ok(Date::ymd(2007, 3, 26)));
        assert_eq!(julian(2008), Ok(Date::ymd(2008, 4, 14)));
        assert_eq!(julian(2009), Ok(Date::ymd(2009, 4, 6)));
        assert_eq!(julian(2010), Ok(Date::ymd(2010, 3, 22)));
        assert_eq!(julian(2011), Ok(Date::ymd(2011, 4, 11)));
        assert_eq!(julian(2012), Ok(Date::ymd(2012, 4, 2)));
        assert_eq!(julian(2013), Ok(Date::ymd(2013, 4, 22)));
        assert_eq!(julian(2014), Ok(Date::ymd(2014, 4, 7)));
        assert_eq!(julian(2015), Ok(Date::ymd(2015, 3, 30)));
        assert_eq!(julian(2016), Ok(Date::ymd(2016, 4, 18)));
        assert_eq!(julian(2017), Ok(Date::ymd(2017, 4, 3)));
        assert_eq!(julian(2018), Ok(Date::ymd(2018, 3, 26)));
        assert_eq!(julian(2019), Ok(Date::ymd(2019, 4, 15)));
        assert_eq!(julian(2020), Ok(Date::ymd(2020, 4, 6)));
    }
}
