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

pub fn max_points2(points: Vec<Vec<i32>>) -> i32 {
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

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().sum::<i32>() % k
}

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    if n <= 2 {
        return n as i32;
    }

    let mut ans = 2;
    for i in 0..n {
        for j in i + 1..n {
            let mut curr = 2;
            for k in 0..n {
                if k != i
                    && k != j
                    && (points[j][1] - points[i][1]) * (points[i][0] - points[k][0])
                        == (points[i][1] - points[k][1]) * (points[j][0] - points[i][0])
                {
                    curr += 1;
                }
                ans = ans.max(curr);
            }
        }
    }
    ans
}

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let target = (nums.iter().map(|x| *x as i64).sum::<i64>() % p as i64) as i32;
    if target == 0 {
        return 0;
    }

    let mut hm = std::collections::HashMap::new();
    hm.insert(0, -1);
    let mut total = 0;
    let mut ans = i32::MAX;
    for (i, &num) in nums.iter().enumerate() {
        total = (total + num) % p;
        if num % p == target {
            return 1;
        }
        if let Some(&l) = hm.get(&((total - target + p) % p)) {
            ans = ans.min(i as i32 - l);
        }

        hm.insert(total, i as i32);
    }

    if ans == i32::MAX {
        return -1;
    }

    if ans == nums.len() as i32 { -1 } else { ans }
}

pub fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> bool {
    let dx = x1.max(x2.min(x_center)) - x_center;
    let dy = y1.max(y2.min(y_center)) - y_center;
    dx * dx + dy * dy <= radius * radius
}

pub const fn hamming_weight(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        ans += n & 1;
        n >>= 1;
    }
    ans
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for num in nums {
        ans ^= num;
    }

    ans
}

pub const fn get_sum(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let c = a & b;
        a ^= b;
        b = c << 1;
    }
    a
}

pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![pref[0]];
    for i in 1..pref.len() {
        ans.push(pref[i] ^ pref[i - 1]);
    }
    ans
}

pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let n = n as i64;
    let mut l = 1;
    let mut r = batteries.iter().map(|x| *x as i64).sum::<i64>() / n;
    while l < r {
        let t = r - (r - l) / 2;
        let mut extra = 0;
        for &p in &batteries {
            extra += (p as i64).min(t);
        }

        if extra / n >= t {
            l = t;
        } else {
            r = t - 1;
        }
    }
    l
}

pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![i32::MAX / 2; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=nums[i] as usize {
            if i + j < n {
                dp[i + j] = dp[i + j].min(dp[i] + 1);
            }
        }
    }

    dp[n - 1]
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return nums[0];
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = nums[0];
    for i in 1..n {
        dp[i + 1] = dp[i].max(dp[i - 1] + nums[i]);
    }
    dp[n]
}

// pub fn max_profit(prices: Vec<i32>) -> i32 {
//     let n = prices.len();
//     let mut dp = vec![vec![i32::MIN; 2]; n + 1];

//     for i in 1..n {
//         dp[i + 1][0] = dp[i - 1][1] + prices[i];
//         dp[i + 1][1] = dp[i][0] - prices[i];
//     }

//     dp[n][0]
// }

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .without_time()
        .init();

    //t1a(max_profit);
}
