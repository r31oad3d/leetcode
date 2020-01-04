struct Solution;

impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 {
            return vec![];
        }
        let (M, N) = (matrix.len() - 1, matrix.get(0).unwrap().len() - 1);
        let mut direction = true;
        let mut res = Vec::<i32>::new();
        let (mut i, mut j) = (0_usize, 0_usize);
        loop {
            //println!("(i,j)=({:?},{:?}),direction={:?}", i, j, direction);
            res.push(matrix.get(i).unwrap().get(j).unwrap().clone());
            if i == M && j == N {
                break;
            }
            if direction {
                if i == 0 {
                    if j == N {
                        i += 1;
                    } else {
                        j += 1;
                    }
                    direction = !direction;
                } else if j == N {
                    i += 1;
                    direction = !direction;
                } else {
                    i -= 1;
                    j += 1;
                }
            } else {
                if j == 0 {
                    if i == M {
                        j += 1;
                    } else {
                        i += 1;
                    }
                    direction = !direction;
                } else if i == M {
                    j += 1;
                    direction = !direction;
                } else {
                    i += 1;
                    j -= 1;
                }
            }
        }
        res
    }
}