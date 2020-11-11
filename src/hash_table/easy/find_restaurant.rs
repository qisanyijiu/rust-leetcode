pub struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut hash = HashMap::new();
        let mut re = <Vec<String>>::new();
        let mut mini = list1.len() + list2.len();
        for (i, s) in list1.iter().enumerate() {
            hash.insert(s, i);
        }
        for (i, s) in list2.iter().enumerate() {
            match hash.get(&s.to_string()) {
                Some(v) => {
                    if i + *v < mini {
                        mini = i + *v;
                        re = vec![s.to_string()];
                    } else if v + i == mini {
                        re.push(s.to_string());
                    }
                }
                None => {}
            }
        }
        re

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        list1: Vec<String>,
        list2: Vec<String>,
        result: Vec<String>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                list1: vec![String::from("Shogun"), String::from("Tapioca Express"), String::from("Burger King"), String::from("KFC")],
                list2: vec![String::from("Piatti"), String::from("The Grill at Torrey Pines"), String::from("Hungry Hunter Steakhouse"), String::from("Shogun")],
                result: vec![String::from("Shogun")]
            },
            Case{
                list1: vec![String::from("Shogun"), String::from("Tapioca Express"), String::from("Burger King"), String::from("KFC")],
                list2: vec![String::from("KFC"), String::from("Shogun"), String::from("Burger King")],
                result: vec![String::from("Shogun")]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::find_restaurant(case_item.list1.clone(), case_item.list2.clone());
            assert_eq!(result, case_item.result)
        }
    }
}