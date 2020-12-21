pub struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut order_map:[usize;26] = [0;26];
        order.chars().into_iter().enumerate().map(|(i, ch)|{
            order_map[Self::index(ch)] = i
        });
        for (i, word) in words.iter().enumerate(){
            if i > words.len() - 2{
                break;
            }
            let next = &words[i+1];
            let tmp_tuple = word.chars().into_iter().zip(next.chars().into_iter());
            for item in tmp_tuple{
                let f_index = Self::index(item.0);
                let s_index = Self::index(item.1);
                if order_map[f_index] > order_map[s_index]{
                    return false;
                }
            }
        }
        true
    }

    #[inline]
    fn index(ch: char) -> usize{
        ch as usize - 'a' as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        words: Vec<String>,
        order: String,
        result: bool
    }

    #[test]
    fn test() {
        let cases = vec![
//            Case{
//                words: vec!["hello".to_string(),"leetcode".to_string()],
//                order: "hlabcdefgijkmnopqrstuvwxyz".to_string(),
//                result: true
//            },
            Case{
                words: vec!["word".to_string(),"world".to_string(),"row".to_string()],
                order: "worldabcefghijkmnpqstuvxyz".to_string(),
                result: false
            },
//            Case{
//                words: vec!["apple".to_string(),"app".to_string()],
//                order: "abcdefghijklmnopqrstuvwxyz".to_string(),
//                result: false
//            }
        ];
        for case_item in cases.iter(){
            let result = Solution::is_alien_sorted(case_item.words.clone(), case_item.order.clone());
            assert_eq!(result, case_item.result)
        }
    }
}