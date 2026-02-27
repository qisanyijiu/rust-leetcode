pub struct Solution {}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cnt = [0; 26]; 
        for c in p.bytes() {
            cnt[(c - b'a') as usize] += 1; // 统计 p 的字母
        }

        let s = s.as_bytes();
        let mut ans = vec![];
        let mut left = 0;
        for (right, &c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            cnt[c] -= 1; // 右端点字母进入窗口
            while cnt[c] < 0 { // 字母 c 太多了
                cnt[(s[left] - b'a') as usize] += 1; // 左端点字母离开窗口
                left += 1;
            }
            if right - left + 1 == p.len() { // t 和 p 的每种字母的出现次数都相同（证明见上）
                ans.push(left as i32); // t 左端点下标加入答案
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        s: String,
        p: String,
        result: Vec<i32>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case {
                s: String::from("cbaebabacd"),
                p: String::from("abc"),
                result: vec![0, 6],
            }
        ];

        for case in cases {
            let result = Solution::find_anagrams(case.s, case.p);
            assert_eq!(result, case.result);
        }
    }
}