pub struct Solution {}

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let data = (0..12u8)
        .flat_map(move |hour| {
            (0..60u8).map(move |minute| (hour, minute))
        });
        data.filter(|(hour, minute)| {
            hour.count_ones() + minute.count_ones() == num as u32
        })
        .map(|(hour, minute)| {
            format!("{hour}:{minute:02}")
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Case {
        num: i32,
        result: Vec<String>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{num: 1, result: vec![
                String::from("0:01"), 
                String::from("0:02"), 
                String::from("0:04"), 
                String::from("0:08"), 
                String::from("0:16"), 
                String::from("0:32"), 
                String::from("1:00"), 
                String::from("2:00"), 
                String::from("4:00"), 
                String::from("8:00")]},
            Case{num: 2, result: vec![
                String::from("0:03"), 
                String::from("0:05"), 
                String::from("0:06"), 
                String::from("0:09"), 
                String::from("0:10"), 
                String::from("0:12"), 
                String::from("0:17"), 
                String::from("0:18"), 
                String::from("0:20"), 
                String::from("0:24"), 
                String::from("0:33"), 
                String::from("0:34"), 
                String::from("0:36"), 
                String::from("0:40"), 
                String::from("0:48"), 
                String::from("1:01"), 
                String::from("1:02"), 
                String::from("1:04"), 
                String::from("1:08"), 
                String::from("1:16"), 
                String::from("1:32"), 
                String::from("2:01"), 
                String::from("2:02"), 
                String::from("2:04"), 
                String::from("2:08"), 
                String::from("2:16"), 
                String::from("2:32"), 
                String::from("3:00"), 
                String::from("4:01"), 
                String::from("4:02"), 
                String::from("4:04"), 
                String::from("4:08"), 
                String::from("4:16"), 
                String::from("4:32"), 
                String::from("5:00"), 
                String::from("6:00"), 
                String::from("8:01"), 
                String::from("8:02"), 
                String::from("8:04"), 
                String::from("8:08"), 
                String::from("8:16"), 
                String::from("8:32"), 
                String::from("9:00"), 
                String::from("10:00")
            ]},
        ];
        for case in cases {
            let result = Solution::read_binary_watch(case.num);
            assert_eq!(result, case.result);
        }
    }
}