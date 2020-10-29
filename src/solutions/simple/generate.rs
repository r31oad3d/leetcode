struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        Solution::sub_generate(num_rows, &mut res);
        res
    }

    fn sub_generate(num_rows: i32, sub_res: &mut Vec<Vec<i32>>) {
        match num_rows {
            0 => {
                return;
            }
            1 => {
                sub_res.push(vec![1]);
            }
            2 => {
                sub_res.push(vec![1]);
                sub_res.push(vec![1, 1]);
            }
            _ => {
                let next_sub_rows = num_rows - 1;
                Solution::sub_generate(next_sub_rows, sub_res);
                let temp = &mut sub_res[next_sub_rows as usize - 1];
                let mut current_res = vec![1];
                for i in 0..temp.len() {
                    let x = temp[i];
                    if i == temp.len() - 1 {
                        break;
                    } else {
                        let y = temp[i + 1];
                        current_res.push(x + y);
                    }
                }
                current_res.push(1);
                sub_res.push(current_res);
            }
        }
    }
}
