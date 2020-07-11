struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let (mut one_num, mut unavailable_num) = (0_i32, 0_i32);
        let checker = |i, j| -> i32 {
            match grid.get(i) {
                Some(inner) => {
                    let inner: &Vec<i32> = inner;
                    match inner.get(j) {
                        Some(point) => {
                            if point == &1 {
                                1
                            } else {
                                0
                            }
                        }
                        None => -2,
                    }
                }
                None => -1,
            }
        };
        let mut i_index = 0_usize;
        'outer: loop {
            let mut j_index = 0_usize;
            'inner: loop {
                let status = checker(i_index, j_index);
                //println!(
                //    "checking:({},{}),status:{},one_num:{},unavailable_num:{}",
                //    i_index, j_index, status, one_num, unavailable_num
                //);
                if status == -1 {
                    break 'outer;
                } else if status == -2 {
                    break 'inner;
                } else if status == 1 {
                    let mut down = checker(i_index + 1, j_index);
                    let mut right = checker(i_index, j_index + 1);
                    if down < 0 {
                        down = 0
                    }
                    if right < 0 {
                        right = 0
                    }
                    unavailable_num += down + right;
                    one_num += 1;
                }
                j_index += 1;
            }
            i_index += 1;
        }
        one_num * 4 - unavailable_num * 2
    }
}
