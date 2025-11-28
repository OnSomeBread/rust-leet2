#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_range_loop)]

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use rayon::prelude::*;
#[allow(unused_imports)]
use tracing::info;

mod first_4k_lines;

mod tests;

#[allow(unused)]
#[allow(clippy::wildcard_imports)]
use tests::*;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut remainder = std::collections::HashMap::new();
    remainder.insert(0, 1);
    let mut ans = 0;
    let mut total = 0;
    for num in nums {
        total += num;
        if let Some(val) = remainder.get(&(total - k)) {
            ans += *val;
        }

        *remainder.entry(total).or_insert(0) += 1;
    }
    ans
}

pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut remainder = std::collections::HashMap::new();
    remainder.insert(0, 1);
    let mut ans = 0;
    let mut total = 0;
    for num in nums {
        total += num;
        let r = (total % k + k) % k;

        if let Some(val) = remainder.get(&r) {
            ans += *val;
        }

        *remainder.entry(r).or_insert(0) += 1;
    }
    ans
}

pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut xy = 0;
    let mut xz = 0;
    let mut yz = 0;
    for (i, row) in grid.iter().enumerate() {
        let mut bxz = i32::MIN;
        let mut byz = i32::MIN;
        for (j, val) in row.iter().enumerate() {
            if *val != 0 {
                xy += 1;
            }
            bxz = bxz.max(*val);
            byz = byz.max(grid[j][i]);
        }
        xz += bxz;
        yz += byz;
    }

    xy + xz + yz
}

pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let area = |p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>| -> f64 {
        0.5 * ((p2[0] - p1[0]) * (p3[1] - p1[1]) - (p2[1] - p1[1]) * (p3[0] - p1[0])).abs() as f64
    };
    let n = points.len();
    let mut ans = 0f64;
    for i in 0..n - 2 {
        for j in i + 1..n - 1 {
            for k in j + 1..n {
                ans = ans.max(area(&points[i], &points[j], &points[k]));
            }
        }
    }
    ans
}

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let mut slopes = std::collections::HashMap::new();
    for point in points {
        let (x, y) = (point[0], point[1]);

        *slopes
            .entry((x as f64 / y as f64).round() as i32)
            .or_insert(0) += 1;
    }

    *slopes.values().max().unwrap()
}

pub fn max_k_divisible_components_dfs(
    n: i32,
    edges: Vec<Vec<i32>>,
    values: Vec<i32>,
    k: i32,
) -> i32 {
    let n = n as usize;
    let mut adj_list = vec![vec![]; n];
    for edge in edges {
        let (start, end) = (edge[0] as usize, edge[1] as usize);
        adj_list[start].push(end);
        adj_list[end].push(start);
    }

    fn dfs(
        ans: &mut i32,
        adj_list: &Vec<Vec<usize>>,
        values: &Vec<i32>,
        k: i32,
        curr: usize,
        parent: usize,
    ) -> i64 {
        let mut total = values[curr] as i64;
        for &child in &adj_list[curr] {
            if child != parent {
                total += dfs(ans, adj_list, values, k, child, curr);
            }
        }

        if total % k as i64 == 0 {
            *ans += 1;
        }

        total
    }
    let mut ans = 0;
    dfs(&mut ans, &adj_list, &values, k, 0, usize::MAX);
    ans
}

pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    if n == 1 {
        return i32::from(values[0] % k == 0);
    }
    let n = n as usize;
    let mut adj_list = vec![vec![]; n];
    let mut in_degree = vec![0; n];
    for edge in edges {
        let (start, end) = (edge[0] as usize, edge[1] as usize);
        adj_list[start].push(end);
        adj_list[end].push(start);
        in_degree[start] += 1;
        in_degree[end] += 1;
    }

    let mut q: std::collections::VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (*x == 1).then_some(i))
        .collect();
    let mut values: Vec<i64> = values.into_iter().map(|x| x as i64).collect();

    let mut ans = 0;
    while let Some(front) = q.pop_front() {
        let total = values[front];
        if values[front] % k as i64 == 0 {
            ans += 1;
        }

        for &adj in &adj_list[front] {
            if in_degree[adj] == 0 {
                continue;
            }
            in_degree[adj] -= 1;
            values[adj] += total;
            if in_degree[adj] == 1 {
                q.push_back(adj);
            }
        }
    }
    ans
}

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .without_time()
        .init();

    t4a(max_k_divisible_components);
}
