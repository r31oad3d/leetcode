struct Solution;


impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let v_secret: Vec<char> = secret.chars().collect();
        let v_guess: Vec<char> = guess.chars().collect();
        let mut i = 0_usize;
        let (mut bulls, mut cows) = (0_i32, 0_i32);
        let mut vv_s = vec![];
        let mut vv_g = vec![];
        while let (Some(ss), Some(gg)) = (v_secret.get(i), v_guess.get(i)) {
            //println!("{:?}:{:?}", ss, gg);
            if ss == gg {
                bulls += 1;
            } else {
                vv_s.push(ss);
                vv_g.push(gg);
                if vv_s.contains(&gg) {
                    cows += 1;
                    let pos = vv_s.iter().position(|x| *x == gg).unwrap();
                    vv_s.remove(pos);
                    let pos = vv_g.iter().position(|x| *x == gg).unwrap();
                    vv_g.remove(pos);
                }
                if vv_g.contains(&ss) {
                    cows += 1;
                    let pos = vv_s.iter().position(|x| *x == ss).unwrap();
                    vv_s.remove(pos);
                    let pos = vv_g.iter().position(|x| *x == ss).unwrap();
                    vv_g.remove(pos);
                }
            }
            i += 1;
        }
        format!("{}A{}B", bulls, cows)
    }
}