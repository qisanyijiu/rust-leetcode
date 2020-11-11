pub struct Solution {}

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        for (row_index,row) in a.iter().enumerate(){
            let mut new_row:Vec<i32> = vec![];
            for (col_index, col) in row.iter().enumerate(){
                let value = 1 - row[row.len()-col_index-1];
                new_row.push(value)
            }
            result.push(new_row);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        img: Vec<Vec<i32>>,
        result: Vec<Vec<i32>>
    }

    #[test]
    fn test() {
        let cases = vec![
            Case{
                img:vec![
                    vec![1,1,0],
                    vec![1,0,1],
                    vec![0,0,0]
                ],
                result: vec![
                    vec![1,0,0],
                    vec![0,1,0],
                    vec![1,1,1]
                ]
            },
            Case{
                img: vec![
                    vec![1,1,0,0],
                    vec![1,0,0,1],
                    vec![0,1,1,1],
                    vec![1,0,1,0]
                ],
                result: vec![
                    vec![1,1,0,0],
                    vec![0,1,1,0],
                    vec![0,0,0,1],
                    vec![1,0,1,0]
                ]
            }
        ];
        for case_item in cases.iter(){
            let result = Solution::flip_and_invert_image(case_item.img.clone());
            assert_eq!(result, case_item.result)
        }
    }
}