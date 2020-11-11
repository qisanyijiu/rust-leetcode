pub struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        use std::collections::HashMap;
        let mut count_map:HashMap<char, i32> = HashMap::new();
        let useful_chars:[char; 5] = ['b','a','l','o','n'];
        for ch in text.chars(){
            if !useful_chars.contains(&ch){
                continue
            }
            match count_map.get(&ch) {
                Some(cnt) => {
                    count_map.insert(ch, *cnt+1);
                },
                None=>{
                    count_map.insert(ch, 1);
                }
            }
        }
        if count_map.len() < 5 {
            return 0
        }
        let mut result:i32 = std::i32::MAX;
        for ch in text.chars(){
            match count_map.get(&ch) {
                Some(cnt)=>{
                    if ch == 'l' || ch == 'o'{
                        result = result.min(*cnt/2)
                    }else{
                        result = result.min(*cnt)
                    }
                }
                None=>{}
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        text: String,
        result: i32
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                text: "nlaebolko".to_string(),
                result: 1
            },
            Case{
                text: "loonbalxballpoon".to_string(),
                result: 2
            },
            Case{
                text: "leetcode".to_string(),
                result: 0
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::max_number_of_balloons(case_item.text.clone());
            assert_eq!(result, case_item.result)
        }
    }
}