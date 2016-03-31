fn check_easter_happened(year: i32) {
    if year < 33 {
        panic!(format!("Jesus isn't dead yet in the year {}", year));
    }
}

pub mod gregorian {
    /// Easter in the Gregorian calendar
    pub fn month_day(year: i32) -> (u32, u32) {
        super::check_easter_happened(year);
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
        (month as u32, day as u32)
    }
}

pub mod julian {
    /// Easter in the Julian calendar
    pub fn month_day(year: i32) -> (u32, u32) {
        super::check_easter_happened(year);
        let aa = year % 4;
        let bb = year % 7;
        let cc = year % 19;
        let dd = (19 * cc + 15) % 30;
        let ee = (2 * aa + 4 * bb - dd + 34) % 7;
        let ff = dd + ee + 114;
        let month = ff / 31;
        let day = ff % 31 + 1;
        (month as u32, day as u32)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gregorian_month_day() {
        use super::gregorian::month_day;
        assert_eq!(month_day(1961), (4, 2));
        assert_eq!(month_day(1996), (4, 7));
        assert_eq!(month_day(1997), (3, 30));
        assert_eq!(month_day(1998), (4, 12));
        assert_eq!(month_day(2000), (4, 23));
        assert_eq!(month_day(2001), (4, 15));
        assert_eq!(month_day(2002), (3, 31));
        assert_eq!(month_day(2003), (4, 20));
        assert_eq!(month_day(2004), (4, 11));
        assert_eq!(month_day(2005), (3, 27));
        assert_eq!(month_day(2006), (4, 16));
        assert_eq!(month_day(2007), (4, 8));
        assert_eq!(month_day(2008), (3, 23));
        assert_eq!(month_day(2009), (4, 12));
        assert_eq!(month_day(2010), (4, 4));
        assert_eq!(month_day(2011), (4, 24));
        assert_eq!(month_day(2012), (4, 8));
        assert_eq!(month_day(2013), (3, 31));
        assert_eq!(month_day(2014), (4, 20));
        assert_eq!(month_day(2015), (4, 5));
        assert_eq!(month_day(2016), (3, 27));
        assert_eq!(month_day(2017), (4, 16));
        assert_eq!(month_day(2018), (4, 1));
        assert_eq!(month_day(2019), (4, 21));
        assert_eq!(month_day(2020), (4, 12));
    }

    #[test]
    fn julian_month_day() {
        use super::julian::month_day;
        assert_eq!(month_day(1961), (3, 27));
        assert_eq!(month_day(1996), (4, 1));
        assert_eq!(month_day(1997), (4, 14));
        assert_eq!(month_day(1998), (4, 6));
        assert_eq!(month_day(1999), (3, 29));
        assert_eq!(month_day(2000), (4, 17));
        assert_eq!(month_day(2001), (4, 2));
        assert_eq!(month_day(2002), (4, 22));
        assert_eq!(month_day(2003), (4, 14));
        assert_eq!(month_day(2004), (3, 29));
        assert_eq!(month_day(2005), (4, 18));
        assert_eq!(month_day(2006), (4, 10));
        assert_eq!(month_day(2007), (3, 26));
        assert_eq!(month_day(2008), (4, 14));
        assert_eq!(month_day(2009), (4, 6));
        assert_eq!(month_day(2010), (3, 22));
        assert_eq!(month_day(2011), (4, 11));
        assert_eq!(month_day(2012), (4, 2));
        assert_eq!(month_day(2013), (4, 22));
        assert_eq!(month_day(2014), (4, 7));
        assert_eq!(month_day(2015), (3, 30));
        assert_eq!(month_day(2016), (4, 18));
        assert_eq!(month_day(2017), (4, 3));
        assert_eq!(month_day(2018), (3, 26));
        assert_eq!(month_day(2019), (4, 15));
        assert_eq!(month_day(2020), (4, 6));
    }
}
