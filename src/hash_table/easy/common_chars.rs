pub struct Solution {}

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut answer = Self::count(&a[0]);
        for s in a[1..].iter() {
            let c = Self::count(s);
            for (idx, cnt) in answer.iter_mut().enumerate() {
                *cnt = std::cmp::min(*cnt, c[idx]);
            }
        }
        answer.iter().enumerate()
            .flat_map(|(idx, cnt)| std::iter::repeat((idx + 97) as u8 as char)
                .take(*cnt as usize))
            .map(|ch| ch.to_string()).collect()
    }

    #[inline]
    fn count(s: &str) -> [u8; 26] {
        s.chars()
            .map(|ch| (ch as usize) - 97)
            .fold([0u8; 26], |mut counts, ch| { counts[ch] += 1; counts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        a: Vec<String>,
        result: Vec<String>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                a: vec!["bella".to_string(),"label".to_string(),"roller".to_string()],
                result: vec!["e".to_string(),"l".to_string(),"l".to_string()]
            },
            Case{
                a: vec!["cool".to_string(),"lock".to_string(),"cook".to_string()],
                result: vec!["c".to_string(),"o".to_string()]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::common_chars(case_item.a.clone());
            assert_eq!(result, case_item.result)
        }
    }
}