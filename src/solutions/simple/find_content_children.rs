struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>,mut s: Vec<i32>) -> i32 {

        let (mut cnt, mut g_index, mut s_index) = (0_i32, 0_usize, 0_usize);
        g.sort();
        s.sort();
        while g_index< g.len() && s_index< s.len() {
            if s[s_index] >= g[g_index] {
                cnt += 1;
                g_index += 1;
                s_index += 1;
            } else if s[s_index] < g[g_index] {
                s_index += 1;
            }
        }
        cnt
    }
}