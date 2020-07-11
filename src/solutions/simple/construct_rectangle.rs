struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut area_mut: f64 = area as f64;
        let mut root: i32 = area_mut.sqrt().round() as i32;
        let mut diff = area - 1_i32;
        let mut res = vec![area, 1];
        let mut j = root;
        'outer: for i in (2..=root).rev() {
            while j < area {
                println!("i:{}, j:{}, diff:{}, res:{:?}", i, j, diff, res);
                if i * j == area {
                    if j - i < diff {
                        diff = j - i;
                        res = vec![j, i];
                        break 'outer;
                    }
                } else if i * j > area {
                    continue 'outer;
                } else {
                    j += 1;
                }
            }
        }
        res
    }

    pub fn construct_rectangle_v2(area: i32) -> Vec<i32> {
        let mut area_mut: f64 = area as f64;
        let mut root: i32 = area_mut.sqrt().round() as i32;
        while area % root != 0 {
            root -= 1;
        }
        vec![area / root, root]
    }
}

pub fn test() {
    println!("{:?}", Solution::construct_rectangle(40));
}
