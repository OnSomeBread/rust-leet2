#![allow(clippy::needless_pass_by_value, clippy::needless_range_loop)]
#![feature(fn_traits)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

//mod first_4k_lines;
mod tests;

use itertools::Itertools;
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

pub fn max_profit_top_down(prices: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn dp(nums: &Vec<i32>, i: usize, has: bool, cache: &mut HashMap<(usize, bool), i32>) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        if let Some(&ans) = cache.get(&(i, has)) {
            return ans;
        }

        let mut best = dp(nums, i + 1, has, cache);
        if has {
            best = best.max(dp(nums, i + 2, !has, cache) + nums[i]);
        } else {
            best = best.max(dp(nums, i + 1, !has, cache) - nums[i]);
        }

        cache.insert((i, has), best);
        best
    }

    dp(&prices, 0, false, &mut HashMap::new())
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![vec![0; 2]; n + 2];

    for i in (0..n).rev() {
        for j in (0..2).rev() {
            let has = j != 0;
            let mut best = dp[i + 1][has as usize];
            if has {
                best = best.max(dp[i + 2][!has as usize] + prices[i]);
            } else {
                best = best.max(dp[i + 1][!has as usize] - prices[i]);
            }

            dp[i][j] = best;
        }
    }

    dp[0][0]
}

pub fn max_profit2(prices: Vec<i32>, fee: i32) -> i32 {
    let n = prices.len();
    let mut dp = [0; 2];

    for i in (0..n).rev() {
        for j in (0..2).rev() {
            if j == 1 {
                dp[j] = dp[j].max(dp[0] + prices[i] - fee);
            } else {
                dp[j] = dp[j].max(dp[1] - prices[i]);
            }
        }
    }

    dp[0]
}

pub fn max_profit3_top_down(prices: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    fn dp(
        prices: &Vec<i32>,
        i: usize,
        has: bool,
        count: i32,
        cache: &mut HashMap<(usize, bool, i32), i32>,
    ) -> i32 {
        if i >= prices.len() {
            return 0;
        }

        if count > 2 {
            return -100_000;
        }

        if let Some(&ans) = cache.get(&(i, has, count)) {
            return ans;
        }

        let mut best = dp(prices, i + 1, has, count, cache);
        if has {
            best = best.max(dp(prices, i + 1, !has, count, cache) + prices[i]);
        } else {
            best = best.max(dp(prices, i + 1, !has, count + 1, cache) - prices[i]);
        }

        cache.insert((i, has, count), best);
        best
    }

    dp(&prices, 0, false, 0, &mut HashMap::new())
}

pub fn max_profit3(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let k = 2;
    let mut dp = vec![vec![0; 2]; k + 2];

    for i in (0..n).rev() {
        for l in (0..=k).rev() {
            for j in 0..2 {
                if j == 1 {
                    dp[l][j] = dp[l][j].max(dp[l][0] + prices[i]);
                } else {
                    dp[l][j] = dp[l][j].max(dp[l + 1][1] - prices[i]);
                }
            }
        }
    }

    dp[0][0]
}

pub fn max_profit4(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let k = k as usize;
    let mut dp = vec![vec![0; 2]; k + 2];

    for i in (0..n).rev() {
        for l in (0..=k).rev() {
            for j in 0..2 {
                if j == 1 {
                    dp[l][j] = dp[l][j].max(dp[l][0] + prices[i]);
                } else {
                    dp[l][j] = dp[l][j].max(dp[l + 1][1] - prices[i]);
                }
            }
        }
    }

    dp[0][0]
}

pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut presums = vec![nums[0]];
    for &num in nums.iter().skip(1) {
        presums.push(num + presums[presums.len() - 1]);
    }

    let mut ans = 0;
    let n = nums.len();
    for i in 0..n - 1 {
        let l = presums[i];
        let r = presums[n - 1] - l;
        if (r - l).abs() % 2 == 0 {
            ans += 1;
        }
    }
    ans
}

pub fn count_triples(n: i32) -> i32 {
    let mut ans = 0;
    for a in 1..=n {
        for b in a + 1..=n {
            let c2 = a * a + b * b;
            let t = (c2 as f64).sqrt();
            if (t - t.round()).abs() <= 1e-6 && c2 <= n * n {
                ans += 2;
            }
        }
    }
    ans
}

pub fn chef(n: i32, p: i32, k: i32) -> i32 {
    let mut day = 1;
    for i in 0..k {
        let mut val = i;
        while val < n {
            if val == p {
                return day;
            }
            day += 1;
            val += k;
        }
    }
    -1
}

pub fn best_closing_time(customers: String) -> i32 {
    let mut cust = 0;
    for c in customers.chars() {
        if c == 'Y' {
            cust += 1;
        }
    }

    let mut smallest_penalty = i32::MAX;
    let mut best_closing_time = 0;
    let mut no_cust = 0;
    for (i, c) in customers.chars().enumerate() {
        let penalty = no_cust + cust;

        if penalty < smallest_penalty {
            smallest_penalty = penalty;
            best_closing_time = i;
        }

        if c == 'Y' {
            cust -= 1;
        } else {
            no_cust += 1;
        }
    }

    if no_cust < smallest_penalty {
        customers.len() as i32
    } else {
        best_closing_time as i32
    }
}

pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = 0;
    for (i, child) in happiness.iter().enumerate().take(k as usize) {
        ans += (*child as i64 - i as i64).max(0);
    }
    ans
}

pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
    capacity.sort_unstable_by(|a, b| b.cmp(a));
    let mut apples = apple.iter().sum::<i32>();

    for (i, c) in capacity.iter().enumerate() {
        apples -= *c;
        if apples <= 0 {
            return i as i32 + 1;
        }
    }
    -1
}

pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let strs: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();
    let mut ans = 0;
    for j in 0..strs[0].len() {
        let mut prev = strs[0][j];
        for i in 1..strs.len() {
            if prev > strs[i][j] {
                ans += 1;
                break;
            }
            prev = strs[i][j];
        }
    }
    ans
}

pub fn max_two_events(mut events: Vec<Vec<i32>>) -> i32 {
    events.sort_unstable();
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let mut bh: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    let mut ans = 0;
    let mut best = 0;
    for e in events {
        while let Some(Reverse((end_time, event_value))) = bh.peek()
            && *end_time < e[0]
        {
            best = best.max(*event_value);
            bh.pop();
        }

        ans = ans.max(best + e[2]);
        bh.push(Reverse((e[1], e[2])));
    }
    ans
}

// pub fn min_deletion_size2(strs: Vec<String>) -> i32 {
//     let strs: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();
//     let mut ans = 0;
//     for j in 0..strs[0].len() {
//         let mut prev = strs[0][j];
//         let mut sorted = true;
//         for i in 1..strs.len() {
//             if prev > strs[i][j] {
//                 ans += 1;
//                 sorted = false;
//                 break;
//             }
//             prev = strs[i][j];
//         }
//         if sorted {
//             return ans;
//         }
//     }
//     ans
// }

pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
    use std::cmp::Reverse as Rev;
    use std::collections::BinaryHeap;
    meetings.sort_unstable_by_key(|x| x[0]);
    let n = n as usize;

    let mut room_used_count = vec![0; n];
    let mut open_rooms: BinaryHeap<Rev<usize>> = (0..n).map(Rev).collect();
    let mut unopen_rooms: BinaryHeap<Rev<(u32, usize)>> = BinaryHeap::new();

    for meeting in meetings {
        let (start, end) = (meeting[0] as u32, meeting[1] as u32);

        while let Some(&Rev((meeting_time, room))) = unopen_rooms.peek()
            && start >= meeting_time
        {
            unopen_rooms.pop();
            open_rooms.push(Rev(room));
        }

        let (room, end_time) = if let Some(Rev(room)) = open_rooms.pop() {
            (room, end)
        } else {
            let Rev((meeting_time, room)) = unopen_rooms.pop().unwrap();
            (room, meeting_time + end - start)
        };

        room_used_count[room] += 1;
        unopen_rooms.push(Rev((end_time, room)));
    }

    let mut most_used_room = 0;
    let mut most_used_count = 0;
    for (room, &count) in room_used_count.iter().enumerate() {
        if count > most_used_count {
            most_used_count = count;
            most_used_room = room;
        }
    }

    most_used_room as i32
}

pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let n = grid[0].len();
    let mut prev = n;
    for i in 0..grid.len() {
        for j in (0..prev).rev() {
            if grid[i][j] >= 0 {
                ans += n - j - 1;
                prev = j + 1;
                break;
            }
        }
        if grid[i][0] < 0 {
            ans += n;
        }
    }
    ans as i32
}

pub fn max_profit5(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;

    let mut presums_prices = vec![0];
    let mut presums_pricesxstrat = vec![0];
    for (&price, &strat) in prices.iter().zip(strategy.iter()) {
        presums_prices.push(price as i64 + presums_prices.last().unwrap());
        presums_pricesxstrat
            .push(price as i64 * strat as i64 + presums_pricesxstrat.last().unwrap());
    }

    let s = *presums_pricesxstrat.last().unwrap();
    let mut ans = s;
    for i in 0..=(prices.len() - k) {
        let left = presums_pricesxstrat[i];
        let mid = presums_prices[i + k] - presums_prices[i + k / 2];
        let right = s - presums_pricesxstrat[i + k];

        ans = ans.max(left + right + mid);
    }
    ans
}

pub fn validate_coupons(
    code: Vec<String>,
    business_line: Vec<String>,
    is_active: Vec<bool>,
) -> Vec<String> {
    let mut ans = vec![];
    let valid_categories: std::collections::HashMap<&str, u8> = vec![
        ("electronics", 0),
        ("grocery", 1),
        ("pharmacy", 2),
        ("restaurant", 3),
    ]
    .into_iter()
    .collect();

    for (c, b, &a) in itertools::izip!(&code, &business_line, &is_active) {
        if a && let Some(&val) = valid_categories.get(b.as_str())
            && !c.is_empty()
            && c.chars().all(|x| x.is_alphanumeric() || x == '_')
        {
            ans.push((c.clone(), val));
        }
    }

    ans.into_iter()
        .sorted_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)))
        .map(|x| x.0)
        .collect()
}

pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize + 1;
    let mut max_row = vec![0; n];
    let mut min_row = vec![n; n];
    let mut max_col = vec![0; n];
    let mut min_col = vec![n; n];
    for b in &buildings {
        let x = b[0] as usize;
        let y = b[1] as usize;

        max_row[y] = max_row[y].max(x);
        min_row[y] = min_row[y].min(x);

        max_col[x] = max_col[x].max(y);
        min_col[x] = min_col[x].min(y);
    }

    let mut ans = 0;

    for b in &buildings {
        let x = b[0] as usize;
        let y = b[1] as usize;

        if x < max_row[y] && x > min_row[y] && y < max_col[x] && y > min_col[x] {
            ans += 1;
        }
    }

    ans
}

pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    fn check(grid: &[Vec<i32>], i: usize, j: usize) -> bool {
        let mut v = [false; 10];
        for ni in i..i + 3 {
            for nj in j..j + 3 {
                if grid[ni][nj] > 9 || grid[ni][nj] == 0 || v[grid[ni][nj] as usize] {
                    return false;
                }
                v[grid[ni][nj] as usize] = true;
            }
        }

        let s = grid[i][j] + grid[i + 1][j + 1] + grid[i + 2][j + 2];
        let s2 = grid[i][j + 2] + grid[i + 1][j + 1] + grid[i + 2][j];
        if s != s2 {
            return false;
        }

        for ni in i..i + 3 {
            let mut row_sum = 0;
            let mut col_sum = 0;
            for nj in j..j + 3 {
                row_sum += grid[ni][nj];
                col_sum += grid[i + nj - j][j + ni - i];
            }

            if s != row_sum || s != col_sum {
                return false;
            }
        }

        true
    }

    let m = grid.len();
    if m < 3 {
        return 0;
    }
    let n = grid[0].len();
    if n < 3 {
        return 0;
    }

    let mut ans = 0;
    for i in 0..grid.len() - 2 {
        for j in 0..grid[i].len() - 2 {
            if check(&grid, i, j) {
                ans += 1;
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
        .with_target(false)
        .init();

    ta(num_magic_squares_inside);
}
