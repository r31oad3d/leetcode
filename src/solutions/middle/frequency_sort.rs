struct Solution;


impl Solution {
    pub fn frequency_sort(s: String) -> String {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        for (i,c) in s.chars().enumerate() {
            let ref mut value = map.entry(c).or_insert((c,0));
            value.1 += 1;
        }
        //println!("{:?}", map);
        let mut res = String::new();
        let mut vec: Vec<(char,i32)> = map.values().cloned().collect();
        //println!("1:  {:?}", vec);
        vec.sort_by(|item1, item2| item1.1.partial_cmp(&item2.1).unwrap());
        //println!("2:  {:?}", vec);
        vec.reverse();
        //println!("3:  {:?}", vec);
        for item in vec.iter() {
            for i in 0..item.1 {
                res.push(item.0);
            }
        }
        res
    }
}