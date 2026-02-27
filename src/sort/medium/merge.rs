pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut last = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] <= intervals[last][1] {
                intervals[last][1] = intervals[last][1].max(intervals[i][1]);
            } else {
                last += 1;
                intervals[last][0] = intervals[i][0];
                intervals[last][1] = intervals[i][1]
            }
        }
        intervals.truncate(last );
        intervals
    }
}