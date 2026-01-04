#![allow(clippy::needless_pass_by_value)]
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut start_num: i32 = nums[0];
    let mut num: i32 = nums[0];
    let mut ans = vec![];

    let mut nums_iter = nums.into_iter();
    nums_iter.next();

    #[allow(clippy::while_let_on_iterator)]
    while let Some(val) = nums_iter.next() {
        if val != num + 1 {
            if start_num == num {
                ans.push(num.to_string());
            } else {
                let mut text = String::new();
                text += &start_num.to_string();
                text += "->";
                text += &num.to_string();

                ans.push(text);
            }
            start_num = val;
        }

        num = val;
    }
    if start_num == num {
        ans.push(num.to_string());
    } else {
        let mut text = String::new();
        text += &start_num.to_string();
        text += "->";
        text += &num.to_string();

        ans.push(text);
    }

    ans
}

pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    let mut slow: usize = 0;
    let mut fast: usize = 1;
    while fast < nums.len() {
        if nums[slow] != nums[fast] {
            slow += 1;
            nums[slow] = nums[fast];
        }
        fast += 1;
    }
    slow as i32
}

//274. H-Index
pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut v = citations;
    v.sort_unstable();

    let s = v.len();

    for i in (0..s).rev() {
        if v[s - i - 1] > i as i32 {
            return i as i32 + 1;
        }
    }
    0
}

// 51. N-Queens
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut col: HashSet<i32> = HashSet::new();
    let mut pos: HashSet<i32> = HashSet::new();
    let mut neg: HashSet<i32> = HashSet::new();
    let mut board = vec![".".to_string().repeat(n as usize); n as usize];
    let mut ans = vec![];

    pub fn queens_backtrack(
        col: &mut HashSet<i32>,
        pos: &mut HashSet<i32>,
        neg: &mut HashSet<i32>,
        board: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
        row: i32,
        n: i32,
    ) {
        if row == n {
            ans.push(board.clone());
            return;
        }

        for i in 0..n {
            if col.contains(&i) || pos.contains(&(row - i)) || neg.contains(&(row + i)) {
                continue;
            }

            col.insert(i);
            pos.insert(row - i);
            neg.insert(row + i);
            board[row as usize].replace_range((i as usize)..=(i as usize), "Q");

            queens_backtrack(col, pos, neg, board, ans, row + 1, n);

            col.remove(&i);
            pos.remove(&(row - i));
            neg.remove(&(row + i));
            board[row as usize].replace_range((i as usize)..=(i as usize), ".");
        }
    }

    queens_backtrack(&mut col, &mut pos, &mut neg, &mut board, &mut ans, 0, n);

    ans
}

// 52. N-Queens II
pub fn total_n_queens(n: i32) -> i32 {
    let mut col: HashSet<i32> = HashSet::new();
    let mut pos: HashSet<i32> = HashSet::new();
    let mut neg: HashSet<i32> = HashSet::new();
    let mut board = vec![".".to_string().repeat(n as usize); n as usize];
    let mut ans = 0;

    pub fn queens_backtrack(
        col: &mut HashSet<i32>,
        pos: &mut HashSet<i32>,
        neg: &mut HashSet<i32>,
        board: &mut Vec<String>,
        ans: &mut i32,
        row: i32,
        n: i32,
    ) {
        if row == n {
            *ans += 1;
            return;
        }

        for i in 0..n {
            if col.contains(&i) || pos.contains(&(row - i)) || neg.contains(&(row + i)) {
                continue;
            }

            col.insert(i);
            pos.insert(row - i);
            neg.insert(row + i);
            board[row as usize].replace_range((i as usize)..=(i as usize), "Q");

            queens_backtrack(col, pos, neg, board, ans, row + 1, n);

            col.remove(&i);
            pos.remove(&(row - i));
            neg.remove(&(row + i));
            board[row as usize].replace_range((i as usize)..=(i as usize), ".");
        }
    }

    queens_backtrack(&mut col, &mut pos, &mut neg, &mut board, &mut ans, 0, n);
    ans
}

// 150. Evaluate Reverse Polish Notation
// FINISH
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    let hs = HashSet::from([
        "+".to_string(),
        "-".to_string(),
        "/".to_string(),
        "*".to_string(),
    ]);

    for s in tokens {
        if hs.contains(&s) {
            let second = stack.pop().unwrap();
            let first = stack.pop().unwrap();

            if &s == "+" {
                stack.push(first + second);
            } else if &s == "-" {
                stack.push(first - second);
            } else if &s == "*" {
                stack.push(first * second);
            } else if &s == "/" {
                stack.push(first / second);
            }
        } else {
            stack.push(s.parse::<i32>().unwrap());
        }
    }
    stack.pop().unwrap()
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut v = nums;
    v.sort_unstable();
    let mut curr: i32 = v[0];
    let mut slow: usize = 0;
    let mut fast: usize = 0;
    let s = v.len();

    while fast < s {
        if curr == target {
            if slow == fast {
                return fast as i32;
            }
            return (fast - slow) as i32;
        }
        while curr < target {
            curr += v[fast];
            fast += 1;
        }
        while curr > target && slow < s {
            curr -= v[slow];
            slow += 1;
        }
    }
    0
}

pub const fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    let mut pigs: i32 = 0;
    while (minutes_to_test / minutes_to_die + 1).pow(pigs.cast_unsigned()) < buckets {
        pigs += 1;
    }
    pigs
}

// fails
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut n = nums;
    n.sort_unstable();

    fn k_sum(
        ans: &mut Vec<Vec<i32>>,
        nums: &Vec<i32>,
        quad: &mut Vec<i32>,
        k: i32,
        idx: usize,
        target: i32,
    ) {
        if k != 2 {
            for i in idx..=(nums.len() - k as usize) {
                if i > idx && nums[i] == nums[i - 1] {
                    continue;
                }
                quad.push(nums[i]);
                k_sum(ans, nums, quad, k - 1, i + 1, target - nums[i]);
                quad.pop();
            }
            return;
        }

        let mut left = idx;
        let mut right = nums.len() - 1;
        while left < right {
            let tmp = nums[left] + nums[right];
            #[allow(clippy::comparison_chain)]
            if tmp == target {
                quad.push(nums[left]);
                quad.push(nums[right]);
                ans.push(quad.clone());
                quad.pop();
                quad.pop();
                left += 1;
                while left < right && nums[left] == nums[left - 1] {
                    left += 1;
                }
            } else if tmp < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    let mut ans = vec![];
    k_sum(&mut ans, &n, &mut vec![], 4, 0, target);
    ans
}

// fails final answer needs queue this answer is too slow
pub fn max_sliding_window(nums: &[i32], k: i32) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..=(nums.len() - k as usize) + 1 {
        let mut m = i32::MIN;
        let mut j = i;
        while j < nums.len() && j < i + k as usize {
            if nums[j] > m {
                m = nums[j];
            }
            j += 1;
        }
        ans.push(m);
    }
    ans
}

pub fn get_averages(nums: &[i32], k: i32) -> Vec<i32> {
    let divisor: i64 = k as i64 * 2 + 1;
    if divisor as usize > nums.len() {
        return vec![-1; nums.len()];
    }

    let mut ans = vec![-1; nums.len()];

    let mut sum: i64 = 0;
    for j in nums.iter().take(divisor as usize) {
        sum += *j as i64;
    }

    let s = k as usize;
    for (i, _num) in nums.iter().enumerate().take(nums.len() - s).skip(s) {
        ans[i] = (sum / divisor) as i32;
        sum -= i64::from(nums[i - s]);
        if i + s + 1 < nums.len() {
            sum += i64::from(nums[i + s + 1]);
        }
    }

    ans
}

pub fn length_of_lis(nums: &[i32]) -> i32 {
    let mut ans = 0;
    let mut count;
    for (i, num) in nums.iter().enumerate() {
        count = 1;
        let mut tmp = num;
        for num2 in nums.iter().skip(i) {
            if tmp < num2 {
                tmp = num2;
                count += 1;
            }
        }
        ans = std::cmp::max(count, ans);
    }
    ans
}

// O(N)
// pub fn add_digits(num: i32) -> i32 {
//     let mut number = num.to_string();
//     while number.len() > 1 {
//         number = number.chars().map(|x| x as i32 - 0x30).sum::<i32>().to_string();
//     }

//     number.parse::<i32>().unwrap()
// }

// O(1)
pub const fn add_digits(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }
    let n = num % 9;
    if n == 0 {
        return 9;
    }
    n
}

// 2462. Total Cost to Hire K Workers
// pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
//     use priority_queue::PriorityQueue;
//     let mut ans = 0;
//     let mut pq: PriorityQueue<usize, i32> = PriorityQueue::new();

//     for (i, num) in costs.into_iter().enumerate() {
//         pq.push(i, num);
//     }

//     let mut count = 0;
//     while count < k {
//         let (idx, num) = pq.peek().unwrap();
//         println!("{}", num);
//         ans += *num as i64;

//         pq.pop();
//         count += 1;
//     }

//     ans
// }

// struct LRUCache {
//     map: std::collections::HashMap<i32,i32>,
//     cap:i32,
// }

// impl LRUCache {
//     fn new(capacity: i32) -> Self {
//         let hm: std::collections::HashMap<i32, i32> = std::collections::HashMap::with_capacity(capacity as usize);
//         LRUCache {map:hm, cap:capacity}
//     }

//     fn get(&self, key: i32) -> i32 {
//         match self.map.get(&key) {
//             Some(v) => *v,
//             None => -1,
//         }
//     }

//     fn put(&mut self, key: i32, value: i32) {
//         self.map.insert(key, value);
//     }
// }

// pub fn dfs(island:&Vec<Vec<i32>>, row:usize, col:usize) -> bool{
//     if island[row][col] == 0 && row == island.len() - 1 {
//         return false;
//     }

//     dfs(island, row + 1, col);
//     dfs(island, row - 1, col);
//     dfs(island, row, col + 1);
//     dfs(island, row, col - 1);

//     false
// }

// // 1970. Last Day Where You Can Still Cross
// pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
//     let mut island: Vec<Vec<i32>> = vec![vec![0; col as usize]; row as usize];
//     let mut ans = 0;

//     ans
// }

// slow solution - correct solution involves bit manipulation
pub fn single_number(nums: &[i32]) -> i32 {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *hm.entry(*num).or_insert(0) += 1;
    }

    for (k, v) in hm {
        if v == 1 {
            return k;
        }
    }

    -1
}

pub fn longest_subarray(nums: &[i32]) -> i32 {
    let mut ans = 0;
    //let mut i = 0;
    let mut flag;

    for (i, num) in nums.iter().enumerate() {
        let mut tmp = 0;
        flag = false;
        let mut itr = i;

        if nums[itr] == 1 {
            while itr < nums.len() && (nums[itr] == 1 || !flag) {
                if *num == 0 {
                    flag = true;
                }
                tmp += 1;
                itr += 1;
            }
        }

        ans = std::cmp::max(ans, tmp);
    }
    ans
}

// pub fn can_finish_dfs(n:i32, prereq: &Vec<Vec<i32>>, idx: usize) -> bool {
//     if idx > prereq.len() && n >= 0 {
//         return true;
//     }
//     else if n < 0 {
//         return false;
//     }

//     can_finish_dfs(n - 1, prereq, prereq.iter().position(|x| x[0] == prereq[idx][1]).unwrap_or(prereq.len()))
// }
// // create a hashset that will not be checked in the dfs but keeps track of the dfs path so that future paths don't go down the same paths
// pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
//     let mut hs: HashSet<i32> = HashSet::new();
//     let mut count = 0;

//     for v in prerequisites.iter() {
//         let n1 = v[0];
//         let n2 = v[1];

//         if hs.contains(&n1) {
//             return false;
//         }

//         hs.insert(n1);
//         count += 1;

//         if count > num_courses {
//             return false;
//         }
//     }

//     true
// }

// pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
//     let mut ans = 0;

//     for (idx, num) in arr.iter().enumerate() {
//         let mut prev = num;
//         let mut temp = 1;
//         for n in arr.iter().skip(idx) {
//             if n - prev == difference {
//                 temp += 1;
//                 prev = n;
//                 println!("{}", n);
//             }
//         }

//         ans = i32::max(temp, ans);
//     }

//     ans
// }

// unfinished
// pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
//     let mut hs: HashMap<i32, i32> = HashMap::new();
//     let mut ans = 1;
//     for num in arr.iter() {
//         ans = i32::max(ans, *hs.get(num).unwrap_or(&0));
//         hs.insert(*num, 1 + *hs.get(&(num - difference)).unwrap_or(&0));
//     }

//     ans
// }

// using 2 stacks keep track of all of the pos and neg asteroids that come by
// if there are any negative asteroids without any positive stopping them they get added to the answer
// otherwise they collide which ever is heaviest or they both get removed
pub fn asteroid_collision(asteroids: &[i32]) -> Vec<i32> {
    let mut ans = vec![];
    let mut pos = vec![];
    let mut neg = vec![];

    for num in asteroids {
        if num < &0 {
            neg.push(*num);
        } else {
            pos.push(*num);
        }
        while !neg.is_empty() && !pos.is_empty() {
            let v1 = -neg[neg.len() - 1];
            let v2 = pos[pos.len() - 1];
            match v1.cmp(&v2) {
                std::cmp::Ordering::Greater => {
                    pos.pop();
                }
                std::cmp::Ordering::Less => {
                    neg.pop();
                }
                std::cmp::Ordering::Equal => {
                    pos.pop();
                    neg.pop();
                }
            }
        }

        while !neg.is_empty() && pos.is_empty() {
            ans.push(neg.pop().unwrap());
        }
    }

    if !neg.is_empty() {
        ans.extend(neg);
        return ans;
    }
    ans.extend(pos);
    ans
}

pub fn max_rotate_function(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut ans;

    let mut tmp = 0;
    for (j, num) in nums.iter().enumerate().skip(1) {
        tmp += j as i32 * num;
    }
    ans = tmp;

    for i in (1..n).rev() {
        tmp = 0;
        for j in 1..n {
            tmp += j as i32 * nums[(i + j) % n];
        }
        ans = i32::max(ans, tmp);
    }

    ans
}

// pub fn rand_nums(amount: i32, minimum: i32, maximum: i32) -> Vec<i32> {
//     use rand::Rng;
//     let mut rng = rand::thread_rng();

//     let mut ans = vec![];

//     for _c in 0..amount {
//         ans.push(rng.gen_range(minimum..maximum))
//     }

//     ans
// }

// stalin sort
pub fn stalin(nums: Vec<i32>) -> Vec<i32> {
    let mut tmp = i32::MIN;
    nums.into_iter()
        .filter(|&x| {
            if x > tmp {
                tmp = x;
                return true;
            }
            false
        })
        .collect()
}

// stalin sort variant
// pub fn stalin_var(nums:Vec<i32>) -> Vec<i32> {
//     let mut ans = vec![];
//     for i in 0..nums.len() {
//         for j in 0..nums.len() {

//         }
//     }
//     ans
// }

// pub fn compress(chars: &mut Vec<char>) -> i32 {
//     let mut ans = vec![];
//     let mut prev:char = chars[0];
//     let mut count: u32 = 0;

//     for char in chars.iter() {
//         if *char != prev {
//             ans.push(*char);
//             if count != 1 {
//                 ans.push(char::from_u32(count).unwrap());
//             }

//             count = 0;
//             prev = *char;
//         }
//         count += 1;
//     }
//     println!("{:?}", ans);
//     1
// }

// 1870. Minimum Speed to Arrive on Time
// should use binary search l < r
pub fn min_speed_on_time(dist: &[i32], hour: f64) -> i32 {
    let m = match dist.iter().max() {
        Some(v) => *v as f64,
        None => return 0,
    };
    let mut ans = 1.0;
    let mut tmp: f64;
    while ans <= m {
        tmp = 0.0;
        for num in dist.iter().take(dist.len() - 1) {
            tmp += (*num as f64) / ans;

            if tmp > hour {
                break;
            }

            tmp = tmp.ceil();
        }

        tmp += (dist[dist.len() - 1] as f64) / ans;
        if hour >= tmp && hour - tmp < 1.0 {
            return ans as i32;
        }

        ans += 1.0;
    }

    if ans > m {
        return -1;
    }
    ans as i32
}

// 2141. Maximum Running Time of N Computers
pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let mut s = batteries.iter().fold(0_i64, |acc, x| acc + *x as i64);
    let mut v = batteries;
    v.sort_unstable();
    let mut k: usize = 0;
    let l = v.len() - 1;
    while v[l - k] as i64 > s / (n - k as i32) as i64 {
        s -= v[l - k] as i64;
        k += 1;
    }

    s / (n - k as i32) as i64
}

// fast simple solution
// combinations mean if [1,2] exists then [2,1] would be invalid
fn conbine_helper(ans: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, n: i32, k: i32, start: i32) {
    if k == 0 {
        ans.push(tmp.clone());
        return;
    }

    for i in start..n {
        tmp.push(i);
        conbine_helper(ans, tmp, n, k - 1, i + 1);
        tmp.pop();
    }
}

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    conbine_helper(&mut ans, &mut vec![], n + 1, k, 1);
    ans
}

// permutations mean [1,2] and [2,1] are valid
//use std::collections::HashSet; // there is a way of doing this without hashset but fast anyway
pub fn helper_permute(
    ans: &mut Vec<Vec<i32>>,
    tmp: &mut Vec<i32>,
    nums: &[i32],
    hs: &mut HashSet<usize>,
    s: usize,
) {
    if s == 0 {
        ans.push(tmp.clone());
    }

    for i in 0..nums.len() {
        // stops repeat nums
        if hs.contains(&i) {
            continue;
        }

        hs.insert(i);
        tmp.push(nums[i]);
        helper_permute(ans, tmp, nums, hs, s - 1);
        hs.remove(&i);
        tmp.pop();
    }
}

pub fn permute(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut hs = HashSet::<usize>::new();
    helper_permute(&mut ans, &mut vec![], nums, &mut hs, nums.len());
    ans
}

pub fn two_city_sched_cost(costs: &[Vec<i32>]) -> i32 {
    let mut bha: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    let mut bhb: BinaryHeap<(i32, usize)> = BinaryHeap::new();

    let mut ans = 0;
    for (idx, pair) in costs.iter().enumerate() {
        bha.push((pair[0], idx));
        bhb.push((pair[1], idx));
    }

    let mut hs: HashSet<usize> = HashSet::new();
    let mut f = bha.peek() < bhb.peek();

    // god i hate this
    for _i in 0..costs.len() {
        if f {
            let (mut _priority, mut idx) = bha.pop().unwrap();
            while !hs.contains(&idx) {
                (_priority, idx) = bha.pop().unwrap();
            }
            ans += costs[idx][0];
            hs.insert(idx);
        } else {
            let (mut _priority, mut idx) = bhb.pop().unwrap();
            while !hs.contains(&idx) {
                (_priority, idx) = bhb.pop().unwrap();
            }
            ans += costs[idx][1];
            hs.insert(idx);
        }

        f = !f;
    }

    ans
}

fn helper_letter_combination(
    ans: &mut Vec<String>,
    tmp: &mut String,
    digits: &str,
    digit_codes: &Vec<&str>,
    idx: usize,
) {
    if idx == digits.len() {
        ans.push(tmp.clone());
        return;
    }

    for letter in digit_codes[digits.chars().nth(idx).unwrap() as usize - '2' as usize].chars() {
        *tmp += &letter.to_string();
        helper_letter_combination(ans, tmp, digits, digit_codes, idx + 1);
        tmp.pop();
    }
}

pub fn letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    let mut ans = vec![];
    let digit_codes = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

    helper_letter_combination(&mut ans, &mut String::new(), digits, &digit_codes, 0);
    ans
}

// pub fn predict_the_winner(nums: Vec<i32>) -> bool {
//     let mut l = 0;
//     let mut r = nums.len() - 1;
//     let mut p1 = 0;
//     let mut p2 = 0;
//     let mut turn = true;
//     while l < r {
//         if turn {
//             if nums[l] > nums[r] {
//                 p1 += nums[l];
//                 l += 1;
//             }
//             else {
//                 p1 += nums[r];
//                 r -= 1;
//             }
//         }
//         else {
//             if nums[l] > nums[r] {
//                 p2 += nums[l];
//                 l += 1;
//             }
//             else {
//                 p2 += nums[r];
//                 r -= 1;
//             }
//         }
//         turn = !turn;
//     }
//     p1 > p2
// }

// 0, 1, 1, 2, 3, 5, 8
pub fn memo_fib(n: f64) -> f64 {
    if n == 0.0 {
        return 0.0;
    }

    let mut v1 = 1.0;
    let mut v2 = 0.0;
    let mut c = n - 1.0;
    while c > 0.0 {
        let tmp = v1;
        v1 = tmp + v2;
        v2 = tmp;

        c -= 1.0;
    }
    v1
}

pub fn di_string_match(s: &str) -> Vec<i32> {
    let mut ans = vec![];

    let mut low = 0;
    let mut high = s.len() as i32;
    for letter in s.chars() {
        if letter == 'I' {
            ans.push(low);
            low += 1;
        } else {
            ans.push(high);
            high -= 1;
        }
    }
    if s.ends_with('I') {
        ans.push(low);
    } else {
        ans.push(high);
    }
    ans
}

// https://leetcode.com/problems/coin-change-ii/
// pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
//     let mut v = coins;
//     v.sort();
//     v.reverse();

//     let mut ans = 0;
//     let mut c = vec![];
//     let mut tmp = 0;
//     for coin in v.iter() {
//         tmp += *coin;

//         while tmp < amount {

//         }

//         while tmp > amount {

//         }

//     }
//     ans
// }

// https://leetcode.com/problems/unique-paths-ii/description/
// working slow solution
pub fn dfs_unique_paths_with_obstacles(grid: &[Vec<i32>], x: usize, y: usize, ans: &mut i32) {
    if x >= grid.len() || y >= grid[0].len() {
        return;
    }

    if grid[x][y] == 1 {
        return;
    }

    if x == grid.len() - 1 && y == grid[0].len() - 1 {
        *ans += 1;
        return;
    }

    dfs_unique_paths_with_obstacles(grid, x + 1, y, ans);
    dfs_unique_paths_with_obstacles(grid, x, y + 1, ans);
}

pub fn unique_paths_with_obstacles(obstacle_grid: &[Vec<i32>]) -> i32 {
    let mut ans = 0;
    dfs_unique_paths_with_obstacles(obstacle_grid, 0, 0, &mut ans);
    ans
}

pub fn decode_at_index(s: &str, k: i32) -> String {
    let mut ans = String::new();

    for letter in s.chars() {
        if letter.is_ascii_alphabetic() {
            ans += &letter.to_string();
        } else if letter.is_ascii_digit() {
            let tmp = ans.clone();
            for _c in 0..letter.to_digit(10).unwrap() - 1 {
                ans += &tmp;
                if ans.len() >= k as usize {
                    return ans.chars().nth(k as usize - 1).unwrap().to_string();
                }
            }
        }

        if ans.len() >= k as usize {
            return ans.chars().nth(k as usize - 1).unwrap().to_string();
        }
    }

    ans
}

// pub fn find_the_difference(s: String, t: String) -> char {
//     let mut v: Vec<char> = s.chars().collect();
//     v.sort_by(|a,b| b.cmp(a));
//     let mut c: Vec<char> = t.chars().collect();
//     c.sort_by(|a,b| b.cmp(a));

//     for idx in 0..v.len() {
//         if v[idx] != c[idx] {
//             return c[idx];
//         }
//     }

//     c[c.len()-1]
// }

// fastest solution
pub fn find_the_difference(s: &str, t: &str) -> char {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for c in s.chars() {
        sum1 += c as i32;
    }

    for c in t.chars() {
        sum2 += c as i32;
    }

    (sum2 - sum1) as u8 as char
}

// VERY SLOW
// pub fn find_the_difference(s: String, t: String) -> char {
//     let mut sum1 = 0;
//     let mut sum2 = 0;

//     for c in 0..s.len() {
//         sum1 += s.chars().nth(c).unwrap() as i32;
//         sum2 += t.chars().nth(c).unwrap() as i32;
//     }

//     sum2 += t.chars().last().unwrap() as i32;

//     (sum2-sum1) as u8 as char
// }

pub fn num_identical_pairs(nums: &[i32]) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                ans += 1;
            }
        }
    }

    ans
}

pub fn dfs_min_cost_climbing_stairs(cost: &Vec<i32>, idx: usize) -> i32 {
    if idx >= cost.len() {
        return 0;
    }

    let curr = cost[idx];
    let step_1 = dfs_min_cost_climbing_stairs(cost, idx + 1);
    let step_2 = dfs_min_cost_climbing_stairs(cost, idx + 2);

    if step_1 > step_2 {
        return step_2 + curr;
    }

    step_1 + curr
}

pub fn greedy_min_cost_climbing_stairs(cost: &Vec<i32>, curr: i32, idx: usize) -> i32 {
    if idx + 2 >= cost.len() {
        return curr + cost[idx];
    }

    if cost[idx + 1] >= cost[idx + 2] {
        return greedy_min_cost_climbing_stairs(cost, curr + cost[idx], idx + 2);
    }

    greedy_min_cost_climbing_stairs(cost, curr + cost[idx], idx + 1)
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    std::cmp::min(
        greedy_min_cost_climbing_stairs(&cost, 0, 0),
        greedy_min_cost_climbing_stairs(&cost, 0, 1),
    )
}

pub fn find_different_binary_string_dfs(
    hs: &HashSet<&String>,
    built_str: &mut String,
    str_len: usize,
) -> String {
    if built_str.len() == str_len && !hs.contains(built_str) {
        return built_str.clone();
    }
    if built_str.len() >= str_len {
        return String::new();
    }

    built_str.push('0');
    let ans = find_different_binary_string_dfs(hs, &mut built_str.clone(), str_len);
    built_str.pop();
    if !ans.is_empty() {
        return ans;
    }

    built_str.push('1');
    let ans = find_different_binary_string_dfs(hs, &mut built_str.clone(), str_len);
    built_str.pop();
    if !ans.is_empty() {
        return ans;
    }

    String::new()
}

// working backtracking solution
pub fn find_different_binary_string(nums: &[String]) -> String {
    let hs: HashSet<&String> = nums.iter().clone().collect::<HashSet<_>>();
    find_different_binary_string_dfs(&hs, &mut String::new(), nums[0].len())
}

pub fn hamming_weight(n: u128) -> i32 {
    n.to_string()
        .chars()
        .filter(|x| {
            if *x == '1' {
                return true;
            }
            false
        })
        .count() as i32
}

pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    let mut str1 = String::new();
    let mut str2 = String::new();

    for x in &word1 {
        str1 += x;
    }
    for x in &word2 {
        str2 += x;
    }
    str1 == str2
}

pub fn can_cross_dfs(stones: &Vec<i32>, idx: usize, jump: i32, hm: &HashMap<i32, usize>) -> bool {
    if idx == stones.len() - 1 {
        return true;
    }

    for c in -1..1 {
        if hm.contains_key(&(stones[idx] + jump + c)) {
            let entry = hm[&(stones[idx] + jump + c)];
            if entry != 0 && entry > idx && can_cross_dfs(stones, entry, jump + c, hm) {
                return true;
            }
        }
    }

    false
}

// works but time limit exceeded
pub fn can_cross(stones: Vec<i32>) -> bool {
    let start = std::time::Instant::now();
    let mut hm = HashMap::new();
    for (i, num) in stones.iter().copied().enumerate() {
        *hm.entry(num).or_insert(0) = i;
    }
    println!("{}", start.elapsed().as_micros());
    can_cross_dfs(&stones, 0, 0, &hm)
}

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut prev = 0;
    let mut count = 0;
    for num in arr.iter() {
        if *num != prev {
            prev = *num;
            count = 0;
        }
        if count >= arr.len() / 4 || arr.len() == 1 {
            ans = *num;
        }
        count += 1;
    }
    ans
}

pub fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut p = paths[0][1].clone();
    while let Some(val) = paths.iter().skip(1).find(|x| x[0] == p) {
        p = val[1].clone();
    }
    p
}

// finding 2 smallest values in int vec
pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut min1 = prices[0];
    let mut min2 = prices[1];

    for price in prices.into_iter().skip(1) {
        if price < min1 {
            min2 = min1;
            min1 = price;
        } else if price < min2 {
            min2 = price;
        }
    }

    if min1 + min2 > money {
        return money;
    }

    money - min1 - min2
}

pub fn max_score(s: String) -> i32 {
    let mut max = 0;
    for split_idx in 1..s.len() {
        let num1 = s[0..split_idx].to_string();
        let num2 = s[split_idx..s.len()].to_string();
        max = std::cmp::max(
            num1.chars().filter(|x| *x == '0').count() + num2.chars().filter(|x| *x == '1').count(),
            max,
        );
    }
    max as i32
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut ptr = head;
    let mut prev = None;

    while let Some(mut val) = ptr {
        let n = val.clone().next;
        val.next = prev;
        prev = Some(val.clone());
        ptr = n;
    }

    prev
}

pub fn is_ugly(n: i32) -> bool {
    let mut val = n;
    for num in 2..6 {
        while val % num == 0 {
            val /= num;
        }
    }
    val == 1
}

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut hs = HashSet::new();
    for num in nums {
        if hs.contains(&num) {
            return num;
        }
        hs.insert(num);
    }
    0
}

pub fn dp_count_battleships(
    board: &Vec<Vec<char>>,
    checked: &mut Vec<Vec<bool>>,
    row: usize,
    col: usize,
    direction: i32,
) {
    if row >= board.len() || col >= board[0].len() || board[row][col] != 'X' {
        return;
    }

    checked[row][col] = true;
    match direction {
        1 => {
            dp_count_battleships(board, checked, row + 1, col, 1);
        }
        2 => {
            dp_count_battleships(board, checked, row, col + 1, 2);
        }
        _ => {
            dp_count_battleships(board, checked, row + 1, col, 1);
            dp_count_battleships(board, checked, row, col + 1, 2);
        }
    }
}

// 419. Battleships in a Board SOLVED
pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    let mut checked = vec![vec![false; board[0].len()]; board.len()];
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if !checked[i][j] && board[i][j] == 'X' {
                dp_count_battleships(&board, &mut checked, i, j, 0);
                ans += 1;
            }
        }
    }
    ans
}

// SLOWWW
// pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
//     let mut nums = nums.clone();
//     nums.sort();
//     let mut prev = nums[0];
//     let mut missing = nums[0];
//     let mut dup = nums[0];

//     for num in nums.iter().skip(1) {
//         if *num == prev {
//             dup = *num;
//             break;
//         }
//         prev = *num;
//     }

//     for i in 1..nums.len() + 1 {
//         if !nums.iter().any(|x| *x == i as i32) {
//             missing = i as i32;
//             break;
//         }
//     }
//     vec![dup,missing]
// }

// 645. Set Mismatch
pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut hs = HashSet::new();
    let mut ans = vec![0; 2];

    for num in nums.iter() {
        if hs.contains(num) {
            ans[0] = *num;
        }
        hs.insert(*num);
    }

    for i in 1..nums.len() + 1 {
        if !hs.contains(&(i as i32)) {
            ans[1] = i as i32;
            break;
        }
    }

    ans
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

// inorder traversal of a bin tree
pub fn inorder_traversal_recurs(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    if let Some(node) = root {
        let n = (*node).borrow_mut();
        inorder_traversal_recurs(n.left.clone(), vals);
        vals.push(n.val);
        inorder_traversal_recurs(n.right.clone(), vals);
    }
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    inorder_traversal_recurs(root, &mut ans);
    ans
}

pub fn preorder_traversal_recurs(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    if let Some(node) = root {
        let n = (*node).borrow_mut();
        vals.push(n.val);
        preorder_traversal_recurs(n.left.clone(), vals);
        preorder_traversal_recurs(n.right.clone(), vals);
    }
}

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    preorder_traversal_recurs(root, &mut ans);
    ans
}

pub fn postorder_traversal_recurs(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
    if let Some(node) = root {
        let n = (*node).borrow_mut();
        postorder_traversal_recurs(n.left.clone(), vals);
        postorder_traversal_recurs(n.right.clone(), vals);
        vals.push(n.val);
    }
}

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    postorder_traversal_recurs(root, &mut ans);
    ans
}

pub fn dp_jump(nums: &Vec<i32>, idx: usize, jump: i32) -> i32 {
    if idx >= nums.len() {
        return i32::MAX;
    }
    if idx == nums.len() - 1 {
        return jump;
    }

    let mut m = i32::MAX;
    for i in (1..nums[idx] + 1).rev() {
        m = std::cmp::min(dp_jump(nums, idx + i as usize, jump + 1), m);
    }
    m
}

// works but time limit exceeded SLOW
pub fn jump(nums: Vec<i32>) -> i32 {
    dp_jump(&nums, 0, 0)
}

pub fn pre_order_traversal_dp(
    root: Option<Rc<RefCell<TreeNode>>>,
    ans: &mut Vec<Vec<i32>>,
    current_path: &mut Vec<i32>,
    target_sum: i32,
    current: i32,
) {
    if let Some(node) = root {
        let n = node.borrow();

        current_path.push(n.val);
        if n.left.is_none() && n.right.is_none() && target_sum == current + n.val {
            ans.push(current_path.clone());
            return;
        }

        pre_order_traversal_dp(
            n.left.clone(),
            ans,
            &mut current_path.clone(),
            target_sum,
            current + n.val,
        );
        pre_order_traversal_dp(
            n.right.clone(),
            ans,
            &mut current_path.clone(),
            target_sum,
            current + n.val,
        );
    }
}

// 113. Path Sum II // PASSES backtracking with preorder traversal through binary tree
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    pre_order_traversal_dp(root, &mut ans, &mut vec![], target_sum, 0);
    ans
}

pub fn create_bin_tree_from_vec(nodes: Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
    if nodes.is_empty() {
        return None;
    }

    let mut deq = VecDeque::new();
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        nodes[0].parse::<i32>().unwrap(),
    ))));

    deq.push_back(root.clone());
    let mut i = 1;
    while !deq.is_empty() {
        let front_node = deq.pop_front().unwrap();
        if let Some(node) = front_node {
            let mut n = (*node).borrow_mut();
            if i < nodes.len() {
                if let Ok(val) = nodes[i].parse::<i32>() {
                    n.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    deq.push_back(n.left.clone());
                } else {
                    n.left = None;
                }
            }

            i += 1;
            if i < nodes.len() {
                if let Ok(val) = nodes[i].parse::<i32>() {
                    n.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    deq.push_back(n.right.clone());
                } else {
                    n.right = None;
                }
            }

            i += 1;
        }
    }

    root
}

pub fn level_order_recurs(
    root: Option<Rc<RefCell<TreeNode>>>,
    ans: &mut Vec<Vec<i32>>,
    depth: usize,
) {
    if let Some(node) = root {
        if ans.len() == depth {
            ans.push(vec![]);
        }
        let n = node.borrow();

        ans[depth].push(n.val);
        level_order_recurs(n.left.clone(), ans, depth + 1);
        level_order_recurs(n.right.clone(), ans, depth + 1);
    }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    level_order_recurs(root, &mut ans, 0);
    ans
}

pub fn pre_order_traversal_flatten(
    root: &mut Option<Rc<RefCell<TreeNode>>>,
    ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) {
    if let Some(node) = root {
        let n = node.borrow();
        ans.push(Some((*node).clone()));
        pre_order_traversal_flatten(&mut n.left.clone(), ans);
        pre_order_traversal_flatten(&mut n.right.clone(), ans);
    }
}

// 114. Flatten Binary Tree to Linked List
// pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
//     let mut ans = vec![];
//     pre_order_traversal_flatten(root, &mut ans);
//     for (i, node) in ans.iter().flatten().enumerate().take(ans.len() - 1) {
//         let mut n = (*node).borrow_mut();
//         n.left = None;
//         n.right = ans[i + 1].clone();
//     }
// }

// 82. Remove Duplicates from Sorted List II
// pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut prev = head.clone();

//     head
// }

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];

    for i in 0..temperatures.len() {
        let mut found = false;
        for j in i + 1..temperatures.len() {
            if temperatures[i] < temperatures[j] {
                ans.push((j - i) as i32);
                found = true;
                break;
            }
        }
        if !found {
            ans.push(0);
        }
    }

    ans
}

// 49. Group Anagrams PASSED
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs.iter() {
        let mut v = s.chars().collect::<Vec<char>>();
        v.sort_unstable();
        let ss = v.iter().collect::<String>();
        hm.entry(ss.clone())
            .and_modify(|x| x.push(s.clone()))
            .or_insert_with(|| vec![s.clone()]);
    }

    hm.iter().map(|x| x.1.clone()).collect()
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for num in nums.iter() {
        *hm.entry(num).or_insert(1) += 1;
    }

    let mut count = 0;
    let mut ans = 0;
    for x in hm.iter() {
        if *x.1 > count {
            ans = **x.0;
            count = *x.1;
        }
    }
    ans
}

pub fn first_palindrome(words: Vec<String>) -> String {
    for word in words.iter() {
        let mut right = match word.len() % 2 {
            0 => word.len() / 2,
            1 => word.len() / 2 + 1,
            _ => 0,
        };
        let left = match word.len() % 2 {
            0 | 1 => word.len() / 2 - 1,
            _ => 0,
        };

        let mut left = left as i32;
        let mut lets = word.chars();
        let mut flag = true;
        while left >= 0 && right < word.len() {
            if lets.nth(left as usize).unwrap() != lets.nth(right).unwrap() {
                flag = false;
                break;
            }
            left -= 1;
            right += 1;
        }
        if flag {
            return word.clone();
        }
    }
    String::new()
}

pub fn maximum_odd_binary_number(s: String) -> String {
    let num1 = s.chars().filter(|x| *x == '1').count();

    let ans = if num1 != 0 {
        "1".repeat(num1 - 1)
    } else {
        String::new()
    };

    ans + &"0".repeat(s.len() - num1) + "1"
}

pub fn take_course(
    taken_courses: &mut HashSet<i32>,
    all_courses: &HashMap<i32, i32>,
    num_courses: i32,
    course_num: i32,
) -> i32 {
    if num_courses <= 0 {
        return -1;
    }

    let mut cc = num_courses;
    if taken_courses.contains(&course_num) {
        return num_courses;
    }
    taken_courses.insert(course_num);
    if let Some(key_value) = all_courses.get_key_value(&course_num) {
        cc = take_course(taken_courses, all_courses, num_courses, *key_value.1);
    }

    cc - 1
}

// 207. Course Schedule
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let mut taken_courses: HashSet<i32> = HashSet::new();
    let mut all_courses: HashMap<i32, i32> = HashMap::new();

    for course in prerequisites.iter() {
        all_courses.entry(course[0]).or_insert(course[1]);
    }

    let mut num_courses = num_courses;

    for course in prerequisites.iter() {
        num_courses = take_course(&mut taken_courses, &all_courses, num_courses, course[0]);
        if num_courses == -1 {
            return false;
        }
    }

    true
}

// passes
pub fn find_duplicates1(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    let mut check = vec![false; nums.len() + 1];
    for num in nums.iter() {
        if check[*num as usize] {
            ans.push(*num);
        } else {
            check[*num as usize] = true;
        }
    }
    ans
}

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut check = vec![false; nums.len() + 1];
    nums.into_iter()
        .filter(|x| {
            if !check[*x as usize] {
                check[*x as usize] = true;
                return false;
            }
            true
        })
        .collect()
}

pub fn length_of_last_word(s: String) -> i32 {
    let words: Vec<&str> = s.split_whitespace().collect();
    words[words.len() - 1].len() as i32
}

pub fn max_depth_iter(s: String) -> i32 {
    let mut num = 0;
    let mut ans = 0;
    for letter in s.chars().filter(|x| *x == '(' || *x == ')') {
        if letter == '(' {
            num += 1;
            ans = std::cmp::max(ans, num);
        } else if letter == ')' {
            num -= 1;
        }
    }
    ans
}

pub fn max_depth(s: String) -> i32 {
    let mut num = 0;
    let mut ans = 0;
    for letter in s.chars() {
        if letter == '(' {
            num += 1;
            ans = std::cmp::max(ans, num);
        } else if letter == ')' {
            num -= 1;
        }
    }
    ans
}

pub fn open_file(file_name: &str) -> String {
    use std::{fs, io::prelude::*};

    let mut file = fs::File::open(file_name).expect("file was not found");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("file did not read correctly to a string the file should be encoded into utf8");

    text
}

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut students = students.iter().collect::<std::collections::VecDeque<_>>();
    let mut sandwiches = sandwiches.iter().rev().collect::<Vec<_>>();
    let mut not_eaten = 0;

    while !students.is_empty() && not_eaten < students.len() {
        let s = students.front().unwrap();
        let sandwich = sandwiches.last().unwrap();

        if **s == **sandwich {
            sandwiches.pop();
            students.pop_front();
            not_eaten = 0;
        } else {
            students.push_back(*s);
            students.pop_front();
            not_eaten += 1;
        }
    }

    students.len() as i32
}

pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut ans = vec![String::new(); score.len()];
    let mut prioity = BinaryHeap::new();

    for (idx, s) in score.iter().enumerate() {
        prioity.push((*s, idx));
    }

    if let Some(val) = prioity.pop() {
        ans[val.1] = "Gold Medal".to_string();
    }
    if let Some(val) = prioity.pop() {
        ans[val.1] = "Silver Medal".to_string();
    }
    if let Some(val) = prioity.pop() {
        ans[val.1] = "Bronze Medal".to_string();
    }

    let mut placement = 4;
    while !prioity.is_empty() {
        ans[prioity.pop().unwrap().1] = placement.to_string();
        placement += 1;
    }

    ans
}

// works but slow
pub fn last_remaining_slow(n: i32) -> i32 {
    let mut v: Vec<i32> = (1..=n).collect();
    while v.len() > 1 {
        let mut itr = 0;
        while itr < v.len() {
            v.remove(itr);
            itr += 1;
        }

        if v.len() <= 1 {
            break;
        }

        let mut itr: i32 = (v.len() - 1) as i32;
        while itr >= 0 {
            v.remove(itr as usize);
            itr -= 2;
        }
    }
    v[0]
}

pub const fn last_remaining(n: i32) -> i32 {
    let mut head = 1;
    let mut remaining = n;
    let mut left = true;
    let mut step = 1;

    while remaining > 1 {
        if left || remaining % 2 == 1 {
            head += step;
        }

        remaining /= 2;
        step *= 2;
        left = !left;
    }

    head
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    while i < j {
        ans = std::cmp::max(ans, (j - i) as i32 * std::cmp::min(height[i], height[j]));
        if height[i] < height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans
}

// does not time limit exceed easy pass
pub fn find_min1(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums[0]
}

// faster solution anyway
pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let mid = usize::midpoint(l, r);
        if nums[mid] > nums[r] {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    nums[l]
}

// passes
pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
    let mut ans = 0;
    for house in &houses {
        let mut m = i32::MAX;
        for heater in &heaters {
            m = std::cmp::min(m, i32::abs(*heater - *house));
        }
        ans = std::cmp::max(ans, m);
    }

    ans
}

// SOO FUCKING CLOSE 1255. Maximum Score Words Formed by Letters
pub fn dfs_max_score_words(
    words: &Vec<String>,
    counts: &mut Vec<i32>,
    score: &Vec<i32>,
    idx: usize,
) -> i32 {
    let mut m = 0;
    for i in idx..words.len() {
        let mut valid = true;
        let mut r = 0;
        for letter in words[i].chars() {
            let s = letter as usize - 'a' as usize;
            counts[s] -= 1;
            r += score[s];

            if counts[s] < 0 {
                valid = false;
            }
        }
        if valid {
            r += dfs_max_score_words(words, counts, score, idx + 1);
            m = std::cmp::max(m, r);
        }

        for letter in words[i].chars() {
            counts[letter as usize - 'a' as usize] += 1;
        }
    }
    m
}

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut counts: Vec<i32> = vec![0; score.len()];

    for letter in &letters {
        counts[*letter as usize - 'a' as usize] += 1;
    }

    dfs_max_score_words(&words, &mut counts, &score, 0)
}

pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut hs = std::collections::HashSet::new();
    let mut n = vec![];
    for num in &nums {
        if hs.contains(num) {
            n.push(*num);
        } else {
            hs.insert(num);
        }
    }
    if n.is_empty() {
        return 0;
    }

    let mut ans = n[0];
    for num in n.iter().skip(1) {
        ans ^= *num;
    }

    ans
}

pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
    let mut hs = std::collections::HashMap::new();
    let mut occurance = 1;
    for (idx, num) in nums.iter().enumerate() {
        if *num == x {
            hs.insert(occurance, idx);
            occurance += 1;
        }
    }

    let mut ans = vec![];
    for query in &queries {
        if hs.contains_key(query) {
            ans.push(*hs.entry(*query).or_insert(0) as i32);
        } else {
            ans.push(-1);
        }
    }

    ans
}

// works but too slow didnt make the contest :(
pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut colors = vec![0; limit as usize + 1];
    let mut ans = vec![];
    for query in &queries {
        colors[query[0] as usize] = query[1];
        println!("{:?}", colors);
        let mut hs = std::collections::HashSet::new();
        hs.insert(0);
        let mut c = 0;
        for color in &colors {
            if hs.insert(*color) {
                c += 1;
            }
        }
        ans.push(c);
    }
    ans
}

// fails not sure why infinite looping
pub fn num_steps(s: &str) -> i32 {
    let mut ans = 0;
    let mut curr = s.to_string();
    while curr != "1" {
        let mut n = u32::from_str_radix(s, 2).unwrap();

        println!("{}", n);
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n += 1;
        }

        println!("{}", n);

        curr = format!("{n:b}");
        ans += 1;
    }
    ans
}

pub fn single_number3(nums: Vec<i32>) -> Vec<i32> {
    let mut ht = HashMap::new();
    let mut ans = vec![];
    for num in &nums {
        *ht.entry(*num).or_insert(0) += 1;
    }

    for entry in &ht {
        if *entry.1 == 1 {
            ans.push(*entry.0);
        }
    }

    ans
}

pub fn single_number3method2(nums: Vec<i32>) -> Vec<i32> {
    let mut hs = HashSet::new();
    for num in &nums {
        if hs.contains(num) {
            hs.remove(num);
        } else {
            hs.insert(*num);
        }
    }
    hs.into_iter().collect::<Vec<_>>()
}

pub fn single_number3method3(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans = vec![];
    let mut i = 0;
    while i < nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            i += 2;
        } else {
            ans.push(nums[i]);
            i += 1;
        }
    }

    if ans.len() == 1 {
        ans.push(nums[nums.len() - 1]);
    }

    ans
}

pub fn score_of_string(s: String) -> i32 {
    let mut score = 0;
    let s: Vec<char> = s.chars().collect();
    for i in 0..s.len() - 1 {
        score += (s[i] as i32 - s[i + 1] as i32).abs();
    }
    score
}

pub const fn reverse_string(s: &mut [char]) {
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
}

pub fn longest_palindrome(s: String) -> i32 {
    let mut ht = HashMap::new();
    let mut ans = 0;
    for letter in s.chars() {
        *ht.entry(letter).or_insert(0) += 1;
    }
    let mut odd = false;
    for entry in ht.iter() {
        if *entry.1 % 2 == 0 {
            ans += *entry.1;
        } else if !odd {
            ans += *entry.1;
            odd = true;
        } else {
            ans += *entry.1 - 1;
        }
    }
    ans
}

pub fn longest_palindrome2(s: String) -> i32 {
    let mut hs = HashSet::new();
    let mut ans = 0;
    for letter in s.chars() {
        if hs.contains(&letter) {
            hs.remove(&letter);
            ans += 1;
        } else {
            hs.insert(letter);
        }
    }

    if !hs.is_empty() {
        return ans * 2 + 1;
    }
    ans * 2
}

pub fn clear_digits(s: String) -> String {
    let mut v = vec![];

    for letter in s.chars() {
        if letter.is_ascii_digit() {
            v.pop();
        } else {
            v.push(letter);
        }
    }
    v.into_iter().collect()
}

// fucking hell this passes winner streak is required to pass
pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let mut vd = (0..skills.len()).collect::<std::collections::VecDeque<_>>();
    let mut winners = std::collections::HashMap::new();
    let mut winner_streak = (0, 0);
    while vd.len() > 1 {
        let fp = *vd.front().unwrap();
        vd.pop_front();
        let sp = *vd.front().unwrap();
        vd.pop_front();
        if skills[fp] > skills[sp] {
            if winner_streak.0 == fp {
                winner_streak.1 += 1;
                if winner_streak.1 as usize > skills.len() {
                    return winner_streak.0 as i32;
                }
            } else {
                winner_streak.0 = fp;
                winner_streak.1 = 1;
            }
            let e = winners.entry(fp).or_insert(0);
            *e += 1;
            if *e >= k {
                return fp as i32;
            }
            vd.push_front(fp);
            vd.push_back(sp);
        } else {
            if winner_streak.0 == sp {
                winner_streak.1 += 1;
                if winner_streak.1 as usize > skills.len() {
                    return winner_streak.0 as i32;
                }
            } else {
                winner_streak.0 = sp;
                winner_streak.1 = 1;
            }
            let e = winners.entry(sp).or_insert(0);
            *e += 1;
            if *e >= k {
                return sp as i32;
            }
            vd.push_front(sp);
            vd.push_back(fp);
        }
    }
    -1
}

// some changes to make the code look shorter
pub fn find_winning_player1(skills: Vec<i32>, k: i32) -> i32 {
    let mut vd = (0..skills.len()).collect::<std::collections::VecDeque<_>>();
    let mut winners = std::collections::HashMap::new();
    let mut winner_streak = (0, 0);
    while vd.len() > 1 {
        let fp = *vd.front().unwrap();
        vd.pop_front();
        let sp = *vd.front().unwrap();
        vd.pop_front();

        let winner;
        let loser;
        if skills[fp] > skills[sp] {
            winner = fp;
            loser = sp;
        } else {
            winner = sp;
            loser = fp;
        }

        if winner_streak.0 == winner {
            winner_streak.1 += 1;
            if winner_streak.1 as usize > skills.len() {
                return winner_streak.0 as i32;
            }
        } else {
            winner_streak.0 = winner;
            winner_streak.1 = 1;
        }
        let e = winners.entry(winner).or_insert(0);
        *e += 1;
        if *e >= k {
            return winner as i32;
        }
        vd.push_front(winner);
        vd.push_back(loser);
    }
    -1
}

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut v = heights.clone();
    v.sort_unstable();
    let mut ans = 0;
    for i in 0..heights.len() {
        if v[i] != heights[i] {
            ans += 1;
        }
    }
    ans
}

// sorting is faster at larger values than solution 2
pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
    let mut v = nums;
    v.sort_unstable();
    let mut moves = 0;
    let mut prev = v[0];
    #[allow(clippy::needless_range_loop)]
    for i in 1..v.len() {
        if prev >= v[i] {
            moves += prev - v[i] + 1;
            v[i] = prev + 1;
        }

        prev = v[i];
    }

    moves
}

pub fn min_increment_for_unique2(nums: Vec<i32>) -> i32 {
    // min heap
    let mut bh = std::collections::BinaryHeap::new();
    for num in &nums {
        bh.push(-num);
    }

    let mut moves = 0;
    let mut prev = -bh.pop().unwrap();
    while !bh.is_empty() {
        let mut n = -bh.pop().unwrap();

        if prev >= n {
            moves += prev - n + 1;
            n = prev + 1;
        }

        prev = n;
    }

    moves
}

// too slow 633. Sum of Square Numbers
// find 2 squared ints that add up to c
// this is a similar approach to two sum
pub fn judge_square_sum2(c: i32) -> bool {
    let mut hs = std::collections::HashSet::new();
    let mut h = 0;
    let mut i = 1;
    while h <= c {
        hs.insert(h);
        h = i * i;
        i += 1;
    }

    for num in &hs {
        if hs.contains(&(c - num)) {
            return true;
        }
    }
    false
}

// two pointer approach passes
pub fn judge_square_sum(c: i32) -> bool {
    let mut r = (c as f32).sqrt() as i64;
    let mut l = 0;
    while l <= r {
        let s: i64 = l * l + r * r;
        if s == c as i64 {
            return true;
        }
        if s < c as i64 {
            l += 1;
        } else {
            r -= 1;
        }
    }

    false
}

pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    let mut ans = 0;
    for (idx, hour) in hours.iter().enumerate() {
        hm.insert(*hour % 24, idx);
    }

    for (idx, hour) in hours.iter().enumerate() {
        if hm.contains_key(hour) && *hm.entry(*hour).or_insert(0) < idx {
            ans += 1;
        }
    }
    ans
}

// vec(units, grade on 4.0 scale)
pub fn calc_gpa(grades: Vec<(i32, i32)>) -> f32 {
    let mut total = 0;
    let mut total_units = 0;
    //let mut total_gpa = 0;
    for grade in grades.iter() {
        total += grade.0 * grade.1;
        total_units += grade.0;
        //total_gpa += grade.1;
    }

    total as f32 / (grades.len() as f32 * (total_units as f32 / grades.len() as f32))
}

pub fn grab_grades(file: &str) -> Vec<(i32, i32)> {
    let mut ans = vec![];
    let txt = open_file(file);
    let lines: Vec<&str> = txt.split('\n').collect();

    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        ans.push((
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        ));
    }

    ans
}

pub fn losing_player(x: i32, y: i32) -> String {
    let mut turn = false;
    let mut flag = true;

    let mut coins = (x, y);
    while flag {
        if coins.0 >= 1 && coins.1 >= 4 {
            coins.0 -= 1;
            coins.1 -= 4;
            turn = !turn;
        } else {
            flag = false;
        }
    }

    if turn {
        return "Bob".to_string();
    }
    "Alice".to_string()
}

pub fn minimum_length(s: String) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for letter in s.chars() {
        *hm.entry(letter).or_insert(0) += 1;
    }

    let mut ans = 0;
    for e in hm.iter() {
        if *e.1 % 2 == 1 {
            ans += 1;
        } else {
            ans += 2;
        }
    }
    ans
}

// first pass use heap to find closest abs(x) value
// second pass applies the process which will be 1 for each of the ones that are not equal to abs(x)
pub fn min_changes(nums: Vec<i32>, k: i32) -> i32 {
    let mut hm = std::collections::HashMap::new();
    let mut l = 0;
    let mut r = nums.len() - 1;
    while l < r {
        *hm.entry(i32::abs(nums[l] - nums[r])).or_insert(0) += 1;
        l += 1;
        r -= 1;
    }

    let mut pq: std::collections::BinaryHeap<_> = hm.values().collect();
    let mut m = i32::MAX;
    while !pq.is_empty() {
        let x = *pq.pop().unwrap();
        let mut t = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let v = i32::abs(nums[l] - nums[r]);
            if v == x {
                l += 1;
                r -= 1;
                continue;
            }
            if i32::abs(v - x) <= k {
                t += 1;
            } else if i32::abs(v - x) <= 2 * k {
                t += 2;
            } else {
                t = i32::MAX;
                break;
            }

            l += 1;
            r -= 1;
        }

        m = std::cmp::min(m, t);
    }
    m
}

pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    let mut v: Vec<Vec<i32>> = vec![vec![0; 11]; n as usize];
    for p in &pick {
        v[p[0] as usize][p[1] as usize] += 1;
    }

    let mut ans = 0;
    for e in v.iter().enumerate() {
        for color in e.1 {
            if *color > e.0 as i32 {
                ans += 1;
                break;
            }
        }
    }

    ans
}

pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
    let mut k = k as i64;
    let s = chalk.iter().fold(0, |acc: i64, x: &i32| acc + *x as i64);
    if k >= s {
        k %= chalk.iter().fold(0, |acc: i64, x: &i32| acc + *x as i64);
    }
    let mut k = k as i32;
    let mut idx = 0;
    while k >= chalk[idx % chalk.len()] {
        k -= chalk[idx % chalk.len()];
        idx += 1;
    }

    (idx % chalk.len()) as i32
}

pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut new_str = String::new();
    for letter in s.chars() {
        new_str += &(letter.to_ascii_lowercase() as u8 - 96).to_string();
    }

    let mut k = k;
    while k > 0 {
        let mut sum: u32 = 0;
        for letter in new_str.clone().chars() {
            sum += (letter as u8 - 48) as u32;
        }
        new_str = sum.to_string();
        k -= 1;
    }

    new_str.parse::<i32>().unwrap()
}

pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    let hs1: std::collections::HashSet<String> = s1
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();
    let mut hs2: std::collections::HashSet<String> = s2
        .split_whitespace()
        .map(std::string::ToString::to_string)
        .collect();
    let mut ans: Vec<String> = vec![];

    for item in &hs1 {
        if hs2.contains(item) {
            hs2.remove(item);
        } else {
            ans.push(item.clone());
        }
    }
    #[allow(clippy::useless_conversion)]
    ans.extend(hs2.into_iter());
    ans
}

// https://leetcode.com/problems/the-skyline-problem/
// pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

//     vec![]
// }

pub fn min_element(nums: Vec<i32>) -> i32 {
    let mut new_nums = vec![];
    for num in nums.iter() {
        let mut str_num = num.to_string();
        if str_num.len() > 1 {
            let mut n = 0;

            for letter in str_num.chars() {
                n += letter as i32 - 48;
            }
            str_num = n.to_string();
        }
        new_nums.push(str_num.parse::<i32>().unwrap());
    }
    *new_nums.iter().min().unwrap()
}

pub fn maximum_total_sum(maximum_height: Vec<i32>) -> i64 {
    let mut max_height = maximum_height;
    max_height.sort_unstable();
    max_height.reverse();

    let mut s: i64 = max_height[0] as i64;
    let mut last: i64 = max_height[0] as i64;
    for height in max_height.iter().skip(1) {
        println!("{}", last);
        if *height as i64 >= last {
            last -= 1;
        } else {
            last = *height as i64;
        }
        if last <= 0 {
            return -1;
        }
        s += last;
    }

    s
}

pub fn min_starting_index(s: String, pattern: String) -> i32 {
    for i in 0..s.len() {
        if i + pattern.len() - 1 > s.len() {
            break;
        }

        let mut change = false;
        let mut is_equal = true;
        let mut j = i;
        while j < s.len() && j - i < pattern.len() {
            if (s.chars().nth(j).unwrap() != pattern.chars().nth(j - i).unwrap()) && !change {
                change = true;
            } else if s.chars().nth(j).unwrap() != pattern.chars().nth(j - i).unwrap() {
                is_equal = false;
                break;
            }
            j += 1;
        }

        if j - i != pattern.len() {
            is_equal = false;
        }

        if is_equal {
            return i as i32;
        }
    }

    -1
}

// backtracking approach WAY TOO SLOW O(n!)
// pub fn bt_max_coins(nums: Vec<i32>, coins:i32) -> i32 {
//     if nums.is_empty() {
//         return coins;
//     }
//     let best = coins;
//     for (idx, num) in nums.iter().enumerate() {
//         let left = if idx == 0 {
//             1
//         }
//         else {
//             nums[idx - 1]
//         };

//         let right = if idx == nums.len() - 1 {
//             1
//         }
//         else {
//             nums[idx + 1]
//         };

//         let v =
//         bt_max_coins(nums.clone(), coins + (left * *num * right));

//     }
//     best
// }

// pub fn max_coins(nums: Vec<i32>) -> i32 {
//     bt_max_coins(nums, 0)
// }

//https://leetcode.com/problems/burst-balloons/solutions/892552/for-those-who-are-not-able-to-understand-any-solution-with-diagram/

pub fn solve(nums: &Vec<i32>, memo: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i > j {
        return 0;
    }

    if i == j {
        let mut temp = nums[i];
        if i > 0 {
            temp *= nums[i - 1];
        }
        if i + 1 < nums.len() {
            temp *= nums[i + 1];
        }
        return temp;
    }

    if memo[i][j] != -1 {
        return memo[i][j];
    }

    let mut ans = 0;

    for k in i..j + 1 {
        let mut temp = nums[k];
        if j + 1 < nums.len() {
            temp *= nums[j + 1];
        }
        if i > 0 {
            temp *= nums[i - 1];
        }

        temp += solve(nums, memo, i, k - 1) + solve(nums, memo, k + 1, j);

        ans = i32::max(ans, temp);
    }
    memo[i][j] = ans;
    memo[i][j]
}

pub fn max_coins(nums: Vec<i32>) -> i32 {
    let mut memo = vec![vec![-1; 500]; 500];

    let mut v = vec![1];
    for num in nums.iter() {
        v.push(*num);
    }
    v.push(1);

    solve(&v, &mut memo, 1, v.len() - 2)
}

// BITBURNER START
// correct function for bitburner
// 761t20261u2x1U659C2W1D9p4p4w2u2D8q1z9H2H2R2g1l1Y2726129m2P
pub fn count_chars(letters: String) -> String {
    let mut c = ' ';
    let mut count = 1;
    let mut ans = String::new();

    for i in 0..letters.len() {
        let letter = letters.chars().nth(i).unwrap();
        if letter == c {
            count += 1;
        } else {
            while count > 9 {
                ans += &9.to_string();
                if c.is_ascii_uppercase() {
                    ans += &c.to_ascii_uppercase().to_string();
                } else if c.is_ascii_lowercase() {
                    ans += &c.to_ascii_lowercase().to_string();
                } else {
                    ans += &c.to_string();
                }

                count -= 9;
            }
            ans += &count.to_string();
            if c.is_ascii_uppercase() {
                ans += &c.to_ascii_uppercase().to_string();
            } else if c.is_ascii_lowercase() {
                ans += &c.to_ascii_lowercase().to_string();
            } else {
                ans += &c.to_string();
            }
            c = letter;
            count = 1;
        }
    }
    ans += &count.to_string();
    if c.is_ascii_uppercase() {
        ans += &c.to_ascii_uppercase().to_string();
    } else if c.is_ascii_lowercase() {
        ans += &c.to_ascii_lowercase().to_string();
    } else {
        ans += &c.to_string();
    }

    let mut chars = ans.chars();
    chars.next();
    chars.next();

    chars.as_str().to_string()
}

pub fn recurse_unique_path(grid_size: &Vec<usize>, ans: &mut i32, i: usize, j: usize) {
    if i == grid_size[0] - 1 && j == grid_size[1] - 1 {
        *ans += 1;
    }
    if i < grid_size[0] {
        recurse_unique_path(grid_size, ans, i + 1, j);
    }
    if j < grid_size[1] {
        recurse_unique_path(grid_size, ans, i, j + 1);
    }
}

// correct function for bitburner
pub fn unipaths_in_grid(grid_size: Vec<usize>) -> i32 {
    let mut ans = 0;
    recurse_unique_path(&grid_size, &mut ans, 0, 0);
    ans
}

pub fn stock_trader(prices: Vec<i32>) -> i32 {
    let mut ans = 0;

    for i in 0..prices.len() {
        for j in i + 1..prices.len() {
            if ans < prices[j] - prices[i] {
                ans = prices[j] - prices[i];
            }
        }
    }
    ans
}

// pub fn valid_parenth(s:String) -> Vec<String> {
//     let mut ans = vec![];
//     let mut s = s.clone();
//     let mut chars = s.chars();
//     while chars.nth(0).unwrap() == ')' {
//         chars.next();
//     }
//     while chars.clone().last().unwrap() == '(' {
//         chars.next_back();
//     }
//     s = chars.as_str().to_string();

//     ans
// }

// pub fn merge_intervals(intervals:Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//     let mut ans = vec![];
//     for i in 0..intervals.len() {
//         for j in 0..intervals.len() {

//         }
//     }

//     ans
// }

// pub fn total_ways_to_sum(nums:Vec<i32>) -> i32 {
//     let mut ans = 0;
//     let mut s = nums.clone();
//     s.sort();

//     ans
// }

pub fn ceasar_cipher(word: String, shift: u8) -> String {
    let mut ans = String::new();
    for letter in word.chars() {
        if letter.is_whitespace() {
            ans.push(' ');
            continue;
        }
        let mut l = letter as u8;
        if l - shift < 65 {
            l = l - shift + 26;
        } else {
            l -= shift;
        }
        ans.push(l as char);
    }
    ans
}

pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut prev: Vec<char> = words[0].chars().collect();
    prev.sort_unstable();
    let mut ans = vec![words[0].clone()];
    for word in words.into_iter().skip(1) {
        let mut curr: Vec<char> = word.chars().collect();
        curr.sort_unstable();
        if curr != prev {
            prev = curr;
            ans.push(word);
        }
    }
    ans
}
