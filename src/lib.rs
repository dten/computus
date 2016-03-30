pub mod gregorian {
    /// Easter in the Gregorian calendar
    pub fn month_day(year: i32) -> (u32, u32) {
        let a: i32 = year % 19;
        let b: i32 = year / 100;
        let c: i32 = year % 100;
        let d: i32 = b / 4;
        let e: i32 = b % 4;
        let f: i32 = (b + 8) / 25;
        let g: i32 = (b - f + 1) / 3;
        let h: i32 = (19 * a + b - d - g + 15) % 30;
        let i: i32 = c / 4;
        let k: i32 = c % 4;
        let l: i32 = (32 + 2 * e + 2 * i - h - k) % 7;
        let m: i32 = (a + 11 * h + 22 * l) / 451;
        let month: i32 = (h + l - 7 * m + 114) / 31;
        let day: i32 = (h + l - 7 * m + 114) % 31 + 1;
        (month as u32, day as u32)
    }
}

pub mod julian {
    /// Easter in the Julian calendar
    pub fn month_day(year: i32) -> (u32, u32) {
        let a: i32 = year % 4;
        let b: i32 = year % 7;
        let c: i32 = year % 19;
        let d: i32 = (19 * c + 15) % 30;
        let e: i32 = (2 * a + 4 * b - d + 34) % 7;
        let f: i32 = d + e + 114;
        let month: i32 = f / 31;
        let day: i32 = f % 31 + 1;
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
