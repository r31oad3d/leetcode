struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: i32, j: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
            let mut ret = 1;
            grid[i as usize][j as usize] = 0;
            let (dx, dy) = (vec![-1, 1, 0, 0], vec![0, 0, -1, 1]);
            for i in 0..4 {
                let new_x = i as i32 + dx[i];
                let new_y = j as i32 + dy[i];
                println!("inner: ({},{})", new_x, new_y);
                if 0 <= new_x
                    && (new_x as usize) < grid.len()
                    && 0 <= new_y
                    && (new_y as usize) < grid[0].len()
                    && grid[new_x as usize][new_y as usize] == 1
                {
                    println!("dfs: ({},{})", new_x, new_y);
                    ret += dfs(new_x, new_y, grid);
                }
            }
            ret
        }
        let mut area = 0_i32;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    println!("({},{})", i, j);
                    area =
                        std::cmp::max(area, dfs(i as i32, j as i32, &mut grid));
                }
            }
        }
        area
    }
}

pub fn test() {
    let test = vec![
        vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    ];

    //println!("{:?}", Solution::max_area_of_island(test));

    let test2 = vec![
        vec![1, 1, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1],
    ];
    println!("{:?}", Solution::max_area_of_island(test2));
}
