const OUT_OF_RANGE_ERR: &'static str = "Computus is only valid from 1583 to 9999";

pub mod gregorian {
    /// Easter in the Gregorian calendar
    pub fn month_day(year: i32) -> Result<(u32, u32), &'static str> {
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
        Ok((month as u32, day as u32))
    }
}

pub mod julian {
    /// Easter in the Julian calendar
    pub fn month_day(year: i32) -> Result<(u32, u32), &'static str> {
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
        Ok((month as u32, day as u32))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gregorian_month_day() {
        use super::gregorian::month_day as gregorian;
        assert_eq!(gregorian(1961), Ok((4, 2)));
        assert_eq!(gregorian(1996), Ok((4, 7)));
        assert_eq!(gregorian(1997), Ok((3, 30)));
        assert_eq!(gregorian(1998), Ok((4, 12)));
        assert_eq!(gregorian(2000), Ok((4, 23)));
        assert_eq!(gregorian(2001), Ok((4, 15)));
        assert_eq!(gregorian(2002), Ok((3, 31)));
        assert_eq!(gregorian(2003), Ok((4, 20)));
        assert_eq!(gregorian(2004), Ok((4, 11)));
        assert_eq!(gregorian(2005), Ok((3, 27)));
        assert_eq!(gregorian(2006), Ok((4, 16)));
        assert_eq!(gregorian(2007), Ok((4, 8)));
        assert_eq!(gregorian(2008), Ok((3, 23)));
        assert_eq!(gregorian(2009), Ok((4, 12)));
        assert_eq!(gregorian(2010), Ok((4, 4)));
        assert_eq!(gregorian(2011), Ok((4, 24)));
        assert_eq!(gregorian(2012), Ok((4, 8)));
        assert_eq!(gregorian(2013), Ok((3, 31)));
        assert_eq!(gregorian(2014), Ok((4, 20)));
        assert_eq!(gregorian(2015), Ok((4, 5)));
        assert_eq!(gregorian(2016), Ok((3, 27)));
        assert_eq!(gregorian(2017), Ok((4, 16)));
        assert_eq!(gregorian(2018), Ok((4, 1)));
        assert_eq!(gregorian(2019), Ok((4, 21)));
        assert_eq!(gregorian(2020), Ok((4, 12)));
        assert_eq!(gregorian(2021), Ok((4, 4)));
        assert_eq!(gregorian(2022), Ok((4, 17)));
        assert_eq!(gregorian(3035), Ok((4, 19)));
        assert_eq!(gregorian(4000), Ok((4, 9)));
        assert_eq!(gregorian(9999), Ok((3, 28)));
    }

    #[test]
    fn julian_month_day() {
        use super::julian::month_day as julian;
        assert_eq!(julian(1961), Ok((3, 27)));
        assert_eq!(julian(1996), Ok((4, 1)));
        assert_eq!(julian(1997), Ok((4, 14)));
        assert_eq!(julian(1998), Ok((4, 6)));
        assert_eq!(julian(1999), Ok((3, 29)));
        assert_eq!(julian(2000), Ok((4, 17)));
        assert_eq!(julian(2001), Ok((4, 2)));
        assert_eq!(julian(2002), Ok((4, 22)));
        assert_eq!(julian(2003), Ok((4, 14)));
        assert_eq!(julian(2004), Ok((3, 29)));
        assert_eq!(julian(2005), Ok((4, 18)));
        assert_eq!(julian(2006), Ok((4, 10)));
        assert_eq!(julian(2007), Ok((3, 26)));
        assert_eq!(julian(2008), Ok((4, 14)));
        assert_eq!(julian(2009), Ok((4, 6)));
        assert_eq!(julian(2010), Ok((3, 22)));
        assert_eq!(julian(2011), Ok((4, 11)));
        assert_eq!(julian(2012), Ok((4, 2)));
        assert_eq!(julian(2013), Ok((4, 22)));
        assert_eq!(julian(2014), Ok((4, 7)));
        assert_eq!(julian(2015), Ok((3, 30)));
        assert_eq!(julian(2016), Ok((4, 18)));
        assert_eq!(julian(2017), Ok((4, 3)));
        assert_eq!(julian(2018), Ok((3, 26)));
        assert_eq!(julian(2019), Ok((4, 15)));
        assert_eq!(julian(2020), Ok((4, 6)));
    }
}
