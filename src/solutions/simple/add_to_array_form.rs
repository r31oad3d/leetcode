struct Solution;

impl Solution {
    pub fn add_to_array_form(mut a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut b: Vec<i32> = vec![];
        let mut remainder = k;
        while remainder >= 1 {
            b.push(remainder % 10);
            remainder /= 10;
        }
        a.reverse();
        let size = if a.len() >= b.len() { a.len() } else { b.len() };
        let compute = |n1: i32, n2: i32, flag| -> (i32, i32) {
            let tmp = n1 + n2 + flag;
            if tmp >= 10 {
                (tmp - 10, 1)
            } else {
                (tmp, 0)
            }
        };
        let mut i = 0_usize;
        let mut flag = 0;
        while i < size {
            match (a.get(i), b.get(i)) {
                (Some(n1), Some(n2)) => {
                    let (_res, _flag) = compute(*n1, *n2, flag);
                    flag = _flag;
                    res.insert(0, _res);
                }
                (None, Some(n)) | (Some(n), None) => {
                    let (_res, _flag) = compute(*n, 0, flag);
                    flag = _flag;
                    res.insert(0, _res);
                }
                _ => panic!("wtf?"),
            }
            i += 1;
        }
        if flag == 1 {
            res.insert(0, 1);
        }
        res
    }
}
