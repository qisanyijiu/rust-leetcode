struct Solution {}
impl Solution {

    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let suffix = s.parse::<i64>().unwrap();
        if finish <  suffix {
            return 0
        }
        let mut x = suffix;
        while x > 0 {
            if (x % 10) as i32 > limit {
                return 0
            }
            x = x / 10;
        }
        let mut count = 0;
        let mut min = suffix;

        0
    }
}