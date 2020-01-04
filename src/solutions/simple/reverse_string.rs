struct Solution;


impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 { return; }
        let (mut start, mut end) = (0, s.len()-1);
        while start <= end {
            if start == end {
                break;
            } else {
                s.swap(start, end);
            }
            start += 1;
            end -= 1;
        }
    }
}