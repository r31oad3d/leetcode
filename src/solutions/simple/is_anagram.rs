struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut table: Vec<i32> = vec![0; 26];
        let v_s: Vec<char> = s.chars().collect();
        let v_t: Vec<char> = t.chars().collect();
        let mut i = 0_usize;
        while let Some(c) = v_s.get(i) {
            let tmp_index: usize = *c as usize - 97;
            let mut tmp = table[tmp_index];
            tmp += 1;
            table[tmp_index] = tmp;
            i += 1;
        }
        i = 0_usize;
        while let Some(c) = v_t.get(i) {
            let tmp_index: usize = *c as usize - 97;
            let mut tmp = table[tmp_index];
            tmp -= 1;
            table[tmp_index] = tmp;
            if tmp < 0 {
                return false;
            }
            i += 1;
        }
        return true;
    }
}
