#![allow(clippy::needless_pass_by_value)]

use core::f64;
use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;
use rand::Rng;
#[allow(unused_imports)]
use rayon::prelude::*;
use tracing::info;

#[allow(unused)]
macro_rules! vecvec {
    () => {
        Vec::new()
    };
    ( $( [ $( $x:expr ),* ] ),* $(,)? ) => {
        vec![ $( vec![ $( $x ),* ] ),* ]
    };
    ( $( [ $elem:expr; $n:expr ] ),* $(,)? ) => {
        vec![ $( vec![$elem; $n] ),* ]
    };
}

#[allow(unused)]
macro_rules! vecstrs {
    ($($x:expr),* $(,)?) => (
        vec![$($x.to_string()),*]
    );

    ($elem:expr; $n:expr) => (
        vec![$elem.to_string(); $n]
    );
}

pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    potions.sort_unstable();
    let m = potions.len() as i32;
    for spell in &mut spells {
        *spell = m - potions.partition_point(|x| (*spell as i64 * *x as i64) < success) as i32;
    }

    spells
}

pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let mut best = i32::MIN;
    let n = energy.len();
    let mut prefix = vec![0; n];
    let k = k as usize;
    for i in (0..n).rev() {
        if i + k < n {
            prefix[i] = energy[i] + prefix[i + k];
        } else {
            prefix[i] = energy[i];
        }
        best = best.max(prefix[i]);
    }
    best
}

pub const fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    num_bottles + (num_bottles - 1) / (num_exchange - 1)
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut cost = prices[0];
    let mut best = prices[0] - cost;

    for price in prices.iter().skip(1) {
        cost = cost.min(*price);
        best = best.max(*price - cost);
    }

    best
}

pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut curr = 1;
    let mut prev = 0;
    let mut ans = 0;

    for i in 1..n {
        if nums[i] > nums[i - 1] {
            curr += 1;
        } else {
            prev = curr;
            curr = 1;
        }
        ans = ans.max(prev.min(curr));
        ans = ans.max(curr / 2);
    }
    ans
}

pub fn coin_change_top_down(coins: Vec<i32>, amount: i32) -> i32 {
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    fn dp(coins: &[i32], cache: &mut HashMap<i32, i32>, amount: i32, curr: i32) -> i32 {
        if curr > amount {
            return i32::MAX;
        }
        if curr == amount {
            return 0;
        }

        if let Some(ans) = cache.get(&curr) {
            return *ans;
        }

        let mut best = i32::MAX;
        for coin in coins {
            best = best.min(dp(coins, cache, amount, coin.saturating_add(curr)).saturating_add(1));
        }

        cache.insert(curr, best);
        best
    }

    let ans = dp(&coins, &mut cache, amount, 0);
    if ans == i32::MAX {
        return -1;
    }
    ans
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut dp = vec![i32::MAX; amount + 1];
    dp[0] = 0;
    for coin in coins {
        let coin = coin as usize;
        for i in coin..=amount {
            dp[i] = dp[i].min(dp[i - coin].saturating_add(1));
        }
    }

    let ans = dp[amount];
    if ans == i32::MAX {
        return -1;
    }
    ans
}

pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    fn dp(
        coins: &[i32],
        cache: &mut HashMap<(usize, i32), i32>,
        amount: i32,
        i: usize,
        curr: i32,
    ) -> i32 {
        if i >= coins.len() || curr > amount {
            return 0;
        }
        if curr == amount {
            return 1;
        }
        if let Some(ans) = cache.get(&(i, curr)) {
            return *ans;
        }

        let v = if coins[i] > amount {
            dp(coins, cache, amount, i + 1, curr)
        } else {
            dp(coins, cache, amount, i, curr + coins[i]) + dp(coins, cache, amount, i + 1, curr)
        };
        cache.insert((i, curr), v);
        v
    }
    dp(&coins, &mut cache, amount, 0, 0)
}

pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    let mut vals = vec![0; value as usize];
    for num in nums {
        vals[(((num % value) + value) % value) as usize] += 1;
    }

    // POTENTIAL OPTIMIZATION POINT if any value is 0 exit early
    //let smallest_val = vals.iter().min().map_or(0, |x| *x);
    let mut smallest_val = i32::MAX;
    for val in &vals {
        smallest_val = smallest_val.min(*val);
        if *val == 0 {
            break;
        }
    }

    let mut ans = smallest_val * value;
    for _ in 0..value {
        if vals[(ans % value) as usize] == smallest_val {
            return ans;
        }
        vals[(ans % value) as usize] -= 1;
        ans += 1;
    }
    ans
}

pub const fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut ans = 0;
    let mut empty = 0;
    while num_bottles > 0 {
        ans += num_bottles;
        empty += num_bottles;
        num_bottles = 0;
        while empty - num_exchange >= 0 {
            empty -= num_exchange;
            num_bottles += 1;
            num_exchange += 1;
        }
    }

    ans
}

pub fn next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    let mut piv = n - 1;
    while piv > 0 && nums[piv] <= nums[piv - 1] {
        piv -= 1;
    }

    if piv > 0 {
        let mut piv2 = n - 1;
        while piv2 >= piv && nums[piv2] <= nums[piv - 1] {
            piv2 -= 1;
        }

        nums.swap(piv - 1, piv2);
    }

    nums[piv..n].reverse();
}

pub fn two_sum_slow(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();
    for (idx, num) in nums.iter().enumerate() {
        hm.entry(num).or_insert(idx);
    }

    for (idx, num) in nums.iter().enumerate() {
        let entry = target - num;
        if hm.contains_key(&entry) && *hm.get(&entry).unwrap() != idx {
            return vec![idx as i32, *hm.get(&entry).unwrap() as i32];
        }
    }
    vec![0, 0]
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        if let Some(ans) = hm.get(&(target - num)) {
            return vec![idx as i32, *ans as i32];
        }
        hm.insert(num, idx);
    }

    vec![0, 0]
}

pub const fn broken_calc(start_value: i32, mut target: i32) -> i32 {
    let mut ans = 0;
    while target > start_value {
        if target % 2 == 1 {
            target += 1;
        } else {
            target /= 2;
        }
        ans += 1;
    }
    ans + start_value - target
}

pub fn check_if_pangram(sentence: String) -> bool {
    let mut all_lets = [false; 26];
    for c in sentence.chars() {
        all_lets[(c as u8 - b'a') as usize] = true;
    }

    all_lets.iter().all(|x| *x)
}

pub fn convert_to_title(mut column_number: i32) -> String {
    let mut ans = vec![];
    while column_number > 0 {
        column_number -= 1;
        ans.push(((column_number % 26) as u8 + b'A') as char);
        column_number /= 26;
    }
    ans.iter().rev().collect()
}

pub const fn arrange_coins(n: i32) -> i32 {
    let (mut l, mut r) = (0_i64, n as i64);
    let n = n as i64;

    while l <= r {
        let mid = l + (r - l) / 2;
        let c = (mid * (mid + 1)) / 2;
        if c == n {
            return mid as i32;
        } else if c < n {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    r as i32
}

pub const fn min_operations_slow(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        if (n & 3) == 3 {
            ans += 1;
            n += 1;
        } else {
            ans += n & 1;
            n >>= 1;
        }
    }
    ans
}

pub const fn min_operations_fast(n: i32) -> i32 {
    (n ^ (n * 3)).count_ones() as i32
}

pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    let mut n = nums.len();
    while n > 1 {
        for i in 0..n - 1 {
            nums[i] = (nums[i] + nums[i + 1]) % 10;
        }
        n -= 1;
    }
    nums[0]
}

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();

    for num in &nums {
        *hm.entry(num).or_insert(0) += 1;
    }

    hm.values()
        .max()
        .map_or(0, |frq| hm.values().filter(|x| **x == *frq).sum())
}

pub fn sort_vowels(s: String) -> String {
    let vowels = |x: &char| matches!(x, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U');
    let mut found_vowels: Vec<char> = s.chars().filter(vowels).collect();
    found_vowels.sort_unstable();
    let mut found_iter = found_vowels.into_iter();

    s.chars()
        .map(|x| {
            if vowels(&x) {
                found_iter.next().unwrap()
            } else {
                x
            }
        })
        .collect()
}

pub fn rotate(matrix: &mut [Vec<i32>]) {
    fn swap(matrix: &mut [Vec<i32>], i1: usize, j1: usize, i2: usize, j2: usize) {
        let temp = matrix[i1][j1];
        matrix[i1][j1] = matrix[i2][j2];
        matrix[i2][j2] = temp;
    }

    let n = matrix.len();

    for i in 0..n / 2 {
        for j in 0..n - 2 * i - 1 {
            swap(matrix, i, i + j, i + j, n - 1 - i);
            swap(matrix, i, i + j, n - 1 - i, n - 1 - j - i);
            swap(matrix, i, i + j, n - 1 - j - i, i);
        }
    }
}

pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut pq = std::collections::BinaryHeap::new();
    let mut visited = std::collections::HashSet::new();
    pq.push((-(nums1[0] + nums2[0]), (0_usize, 0_usize)));

    let n1 = nums1.len();
    let n2 = nums2.len();

    while (ans.len() as i32) < k
        && let Some((_, (i, j))) = pq.pop()
    {
        if visited.contains(&(i, j)) {
            continue;
        }
        ans.push(vec![nums1[i], nums2[j]]);
        visited.insert((i, j));

        if i + 1 < n1 && !visited.contains(&(i + 1, j)) {
            pq.push((-(nums1[i + 1] + nums2[j]), (i + 1, j)));
        }

        if j + 1 < n2 && !visited.contains(&(i, j + 1)) {
            pq.push((-(nums1[i] + nums2[j + 1]), (i, j + 1)));
        }
    }

    ans
}

struct Node {
    key: i32,
    value: i32,
    next: Option<Rc<RefCell<Self>>>,
    prev: Option<Rc<RefCell<Self>>>,
}

struct LRUCache {
    cache: std::collections::HashMap<i32, Rc<RefCell<Node>>>,
    max_size: i32,
    lru: Rc<RefCell<Node>>,
    mru: Rc<RefCell<Node>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let lru = Rc::new(RefCell::new(Node {
            key: 0,
            value: 0,
            next: None,
            prev: None,
        }));
        let mru = Rc::new(RefCell::new(Node {
            key: 0,
            value: 0,
            next: None,
            prev: None,
        }));

        lru.borrow_mut().next = Some(Rc::clone(&mru));
        mru.borrow_mut().prev = Some(Rc::clone(&lru));

        Self {
            cache: std::collections::HashMap::new(),
            max_size: capacity,
            lru,
            mru,
        }
    }

    fn insert(&self, node: Rc<RefCell<Node>>) {
        let old_mru = Rc::clone(self.mru.borrow_mut().prev.as_ref().unwrap());
        old_mru.borrow_mut().next = Some(Rc::clone(&node));
        self.mru.borrow_mut().prev = Some(Rc::clone(&node));
        node.borrow_mut().prev = Some(Rc::clone(&old_mru));
        node.borrow_mut().next = Some(Rc::clone(&self.mru));
    }

    #[allow(clippy::unused_self)]
    fn remove(&self, node: Rc<RefCell<Node>>) {
        let prev = Rc::clone(node.borrow_mut().prev.as_ref().unwrap());
        let next = Rc::clone(node.borrow_mut().next.as_ref().unwrap());
        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(&prev));
    }

    fn get(&self, key: i32) -> i32 {
        self.cache.get(&key).map_or(-1, |node| {
            self.remove(Rc::clone(node));
            self.insert(Rc::clone(node));
            node.as_ref().borrow().value
        })
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key) {
            self.remove(Rc::clone(node));
            self.insert(Rc::clone(node));
            node.borrow_mut().value = value;
        } else {
            let node = Rc::new(RefCell::new(Node {
                key,
                value,
                prev: None,
                next: None,
            }));

            self.cache.insert(key, Rc::clone(&node));
            self.insert(node);
        }

        if self.cache.len() > self.max_size as usize {
            let node_to_remove = Rc::clone(self.lru.borrow_mut().next.as_ref().unwrap());
            self.cache.remove(&node_to_remove.borrow().key);
            self.remove(node_to_remove);
        }
    }
}

pub fn test_lru_cache() {
    let mut lru = LRUCache::new(2);
    lru.put(1, 1);
    lru.put(2, 2);
    assert!(lru.get(1) == 1);
    lru.put(3, 3);
    assert!(lru.get(2) == -1);
    lru.put(4, 4);
    assert!(lru.get(1) == -1);
    assert!(lru.get(3) == 3);
    assert!(lru.get(4) == 4);
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl TreeNode {
    #[inline]
    pub const fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    let mut st = vec![(root, 0)];

    while let Some((node, val)) = st.pop()
        && let Some(node) = node
    {
        let node = node.borrow();
        let node_value = node.val + val;
        if node.left.is_none() && node.right.is_none() && node_value == target_sum {
            return true;
        }

        if let Some(left_node) = &node.left {
            st.push((Some(Rc::clone(left_node)), node_value));
        }

        if let Some(right_node) = &node.right {
            st.push((Some(Rc::clone(right_node)), node_value));
        }
    }
    false
}

pub fn max_distinct_elements(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut prev = i32::MIN;
    let mut ans = 0;
    for num in nums {
        let curr = std::cmp::min(std::cmp::max(num - k, prev + 1), num + k);
        if curr > prev {
            ans += 1;
            prev = curr;
        }
    }

    ans
}

pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    #[allow(non_snake_case)]
    let MOD = 1_000_000_007;
    let mut adj_list = vec![vec![]; n as usize];
    for road in roads {
        adj_list[road[0] as usize].push((road[1] as usize, road[2] as i64));
        adj_list[road[1] as usize].push((road[0] as usize, road[2] as i64));
    }

    let mut pq = std::collections::BinaryHeap::new();

    let mut best_times = vec![i64::MAX; n as usize];
    best_times[0] = 0;

    let mut best_time_counts = vec![0; n as usize];
    best_time_counts[0] = 1;

    pq.push((0, 0));

    while let Some((curr_time, curr_road)) = pq.pop() {
        let curr_time = -curr_time;
        if curr_time > best_times[curr_road] {
            continue;
        }

        for (adj_road, time) in &adj_list[curr_road] {
            let t = time + curr_time;
            if t < best_times[*adj_road] {
                best_times[*adj_road] = t;
                best_time_counts[*adj_road] = best_time_counts[curr_road];
                pq.push((-t, *adj_road));
            } else if t == best_times[*adj_road] {
                best_time_counts[*adj_road] =
                    (best_time_counts[*adj_road] + best_time_counts[curr_road]) % MOD;
            }
        }
    }

    best_time_counts[n as usize - 1]
}

pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for num in nums {
        *hm.entry(num).or_insert(0) += 1;
    }

    if let Some((key, _)) = hm.into_iter().filter(|(_, value)| *value == 1).max() {
        key
    } else {
        -1
    }
}

pub fn longest_consecutive_hashset(nums: Vec<i32>) -> i32 {
    let vals: std::collections::HashSet<i32> = nums.into_iter().collect();

    let mut ans = 0;
    for num in &vals {
        if !vals.contains(&(num - 1)) {
            let mut streak = 1;
            let mut v = num + 1;

            while vals.contains(&v) {
                streak += 1;
                v += 1;
            }
            ans = ans.max(streak);
        }
    }
    ans
}

pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    nums.sort_unstable();

    let mut ans = 1;
    let mut prev = nums[0];
    let mut streak = 1;
    for num in nums.iter().skip(1) {
        if *num == prev {
            continue;
        }
        if *num == prev + 1 {
            streak += 1;
        } else {
            streak = 1;
        }
        ans = ans.max(streak);
        prev = *num;
    }
    ans
}

pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations
        .iter()
        .fold(0, |acc, x| if x.contains('+') { acc + 1 } else { acc - 1 })
}

pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;
    let mut presums = vec![0];
    for num in &nums {
        presums.push(presums[presums.len() - 1] + *num);
    }

    let mut cache = HashMap::new();

    fn dp(
        nums: &[i32],
        presums: &[i32],
        cache: &mut HashMap<(usize, i32), i32>,
        i: usize,
        k: i32,
    ) -> i32 {
        if k == 1 {
            return presums[presums.len() - 1] - presums[i];
        }

        if let Some(ans) = cache.get(&(i, k)) {
            return *ans;
        }

        let mut best = i32::MAX;
        for j in i..=nums.len() - k as usize {
            let curr = nums[j] + presums[j] - presums[i];
            if curr > best {
                break;
            }
            best = best.min(std::cmp::max(curr, dp(nums, presums, cache, j + 1, k - 1)));
        }

        cache.insert((i, k), best);
        best
    }

    dp(&nums, &presums, &mut cache, 0, k)
}

struct SparseVector {
    sparse_vec: Vec<i32>,
}

impl SparseVector {
    const fn new(nums: Vec<i32>) -> Self {
        Self { sparse_vec: nums }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: Self) -> i32 {
        vec.sparse_vec
            .iter()
            .zip(self.sparse_vec.iter())
            .fold(0, |acc, x| acc + (x.0 * x.1))
    }
}

pub fn test_sparse_vec(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let v1 = SparseVector::new(nums1);
    let v2 = SparseVector::new(nums2);
    v1.dot_product(v2)
}

pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    nums.sort_unstable();
    let mut counter = std::collections::HashMap::new();
    let mut targets = std::collections::HashSet::new();
    for num in &nums {
        targets.insert(*num);
        targets.insert(*num - k);

        *counter.entry(*num).or_insert(0) += 1;
    }

    let mut targets: Vec<i32> = targets.into_iter().collect();
    targets.sort_unstable();

    let mut ans = 0;
    for target in targets {
        let l = nums.partition_point(|x| *x < target - k) as i32;
        let r = nums.partition_point(|x| *x <= target + k) as i32 - 1;

        if let Some(val) = counter.get(&target) {
            ans = ans.max((num_operations + val).min(r - l + 1));
        } else {
            ans = ans.max(num_operations.min(r - l + 1));
        }
    }
    ans
}

pub fn has_same_digits(s: String) -> bool {
    let mut nums: Vec<u8> = s.as_bytes().iter().map(|x| x - b'0').collect();
    let n = nums.len();
    for i in 0..n - 2 {
        for i in 0..n - i - 1 {
            nums[i] = (nums[i] + nums[i + 1]) % 10;
        }
    }

    nums[0] == nums[1]
}

pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut l = 0;
    let mut m;
    let mut odd = 0;

    for r in 0..nums.len() {
        odd += nums[r] % 2;
        while odd > k {
            odd -= nums[l] % 2;
            l += 1;
        }

        m = l;
        if odd == k {
            while nums[m] % 2 == 0 {
                m += 1;
            }

            ans += m - l + 1;
        }
    }

    ans as i32
}

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut hm = std::collections::HashMap::new();
    hm.insert(0, 1);
    let mut ans = 0;
    let mut curr = 0;
    for num in nums {
        curr += num;
        if hm.contains_key(&(curr - k)) {
            ans += hm.get(&(curr - k)).unwrap();
        }
        *hm.entry(curr).or_insert(0) += 1;
    }
    ans
}

pub fn judge_square_sum_hash_set(c: i32) -> bool {
    let mut s = std::collections::HashSet::new();
    for v in 0..=((c as f64).sqrt() as i32) {
        s.insert(v * v);
    }

    for v in 0..=((c as f64).sqrt() as i32) {
        if s.contains(&(c - v * v)) {
            return true;
        }
    }
    false
}

pub fn judge_square_sum(c: i32) -> bool {
    let mut l = 0;
    let mut r = (c as f64).sqrt() as i64;
    let c = c as i64;

    while l <= r {
        match (l * l + r * r).cmp(&c) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => r -= 1,
            std::cmp::Ordering::Less => l += 1,
        }
    }

    false
}

pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    nums.iter()
        .enumerate()
        .filter_map(|(idx, val)| n.is_multiple_of(idx + 1).then_some(val * val))
        .sum()
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(curr: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(curr) = curr {
            let l = dfs(curr.borrow().left.clone(), ans);
            let r = dfs(curr.borrow().right.clone(), ans);

            *ans = (*ans).max(l + r);

            return l.max(r) + 1;
        }
        0
    }

    let mut ans = 0;
    let _ = dfs(root, &mut ans);
    ans
}

pub fn next_beautiful_number(n: i64) -> i64 {
    for i in n + 1..1e10 as i64 {
        let mut counter = std::collections::HashMap::new();
        for digit in i.to_string().chars() {
            *counter.entry(digit).or_insert(0) += 1;
        }

        if i.to_string().chars().all(|digit| {
            counter
                .get(&digit)
                .is_some_and(|val| *val == digit as u8 - b'0')
        }) {
            return i;
        }
    }

    -1
}

pub fn total_money(n: i32) -> i32 {
    (0..n).fold(0, |acc, x| acc + x % 7 + x / 7 + 1)
}

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let lasers: Vec<usize> = bank
        .iter()
        .map(|x| x.chars().filter(|x| *x == '1').count())
        .collect();

    let mut prev = 0;
    let mut ans = 0;

    for r in lasers {
        if r == 0 {
            continue;
        }

        ans += prev * r;
        prev = r;
    }

    ans as i32
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<Self>),
}

pub fn depth_nested_int(nested_list: &[NestedInteger], depth: i32) -> i32 {
    let mut s = 0;
    for value in nested_list {
        if let NestedInteger::Int(num) = value {
            s += num * depth;
        } else if let NestedInteger::List(nums) = value {
            s += depth_nested_int(nums, depth + 1);
        }
    }
    s
}

pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
    let mut s = 0;
    for value in nested_list {
        if let NestedInteger::Int(num) = value {
            s += num;
        } else if let NestedInteger::List(nums) = value {
            s += depth_nested_int(&nums, 2);
        }
    }
    s
}

pub fn reordered_power_of2(n: i32) -> bool {
    let mut digits = vec![0; 10];
    for digit in n.to_string().chars() {
        digits[(digit as u8 - b'0') as usize] += 1;
    }

    let mut ans = 1;
    while ans < 1e9 as i32 {
        let mut sub_digits = vec![0; 10];
        for digit in ans.to_string().chars() {
            sub_digits[(digit as u8 - b'0') as usize] += 1;
        }

        if digits == sub_digits {
            return true;
        }

        ans <<= 1;
    }

    false
}

pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![];
    let m = mat.len();
    let n = mat[0].len();
    for i in 0..m {
        for j in 0..n {
            if i % 2 == 0 {
                ans.push(mat[i - j][j]);
            }
            // else {
            //     ans.push(mat[i][j]);
            // }
        }
    }
    ans
}

pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let mut hm = HashMap::new();
    let mut smallest = i32::MAX / 2;
    for item in basket1 {
        *hm.entry(item).or_insert(0) += 1;
        smallest = smallest.min(item);
    }
    for item in basket2 {
        *hm.entry(item).or_insert(0) -= 1;
        smallest = smallest.min(item);
    }

    let mut combine = vec![];
    for (key, value) in hm {
        if value % 2 == 1 {
            return -1;
        }

        combine.extend(vec![key; (i32::abs(value) / 2) as usize]);
    }
    combine.sort_unstable();

    let mut ans = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..combine.len() / 2 {
        ans += std::cmp::min(smallest * 2, combine[i]) as i64;
    }

    ans
}

pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let s: i32 = nums.iter().sum();
    let mut ans = 0;
    let mut left = 0;
    for num in nums {
        if num != 0 {
            left += num;
            continue;
        }

        let right = s - left;
        if left == right || left + 1 == right {
            ans += 1;
        }
        if left == right || left == right + 1 {
            ans += 1;
        }
    }

    ans
}

pub const fn smallest_number(n: i32) -> i32 {
    let mut ans = 1;
    let mut i = 1;

    while ans < n {
        ans |= 1 << i;
        i += 1;
    }

    ans
}

pub fn smallest_number_not_const(n: i32) -> i32 {
    let mut ans = 1;
    for i in 1..32 {
        if ans >= n {
            break;
        }
        ans |= 1 << i;
    }

    ans
}

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let hs: std::collections::HashSet<char> = jewels.chars().collect();
    stones.chars().filter(|x| hs.contains(x)).count() as i32
}

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut bh: std::collections::BinaryHeap<i32> = stones.into_iter().collect();

    while bh.len() > 1 {
        let s1 = bh.pop().unwrap();
        let s2 = bh.pop().unwrap();
        if s1 > s2 {
            bh.push(s1 - s2);
        }
    }

    bh.pop().unwrap_or(0)
}

pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
    let mut hm = std::collections::HashMap::new();
    for num in nums {
        *hm.entry(num).or_insert(0) += 1;
    }

    hm.values().all(|x| *x <= 2)
}

pub fn dice_rolls(dice: i32, total: i32) -> i32 {
    let (n, m) = (dice as usize, total as usize);
    let mut dp = vec![vec![0; m]; n];
    #[allow(clippy::needless_range_loop)]
    for i in 0..6 {
        dp[0][i] = 1;
    }

    for i in 1..n {
        for j in 0..m {
            dp[i][j] = (0.max(j as i32 - 6) as usize..j)
                .map(|x| dp[i - 1][x])
                .sum();
        }
    }

    dp[n - 1][m - 1]
}

pub fn min_number_operations(target: Vec<i32>) -> i32 {
    let mut ans = target[0];
    for i in 1..target.len() {
        ans += 0.max(target[i] - target[i - 1]);
    }

    ans
}

pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    dp[0] = 1;
    dp[1] = 2;

    for i in 2..dp.len() {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    dp[(n + 1) as usize]
}

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut adj_list = vec![vec![]; rooms.len()];
    for (i, room) in rooms.iter().enumerate() {
        for key in room {
            adj_list[i].push(*key);
        }
    }

    let mut st = vec![0];
    let mut seen = vec![false; rooms.len()];
    while let Some(room) = st.pop() {
        if seen[room] {
            continue;
        }
        seen[room] = true;

        for next_room in &adj_list[room] {
            st.push(*next_room as usize);
        }
    }

    seen.iter().all(|x| *x)
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![1; nums.len()];

    for i in 0..nums.len() {
        let mut best = 0;
        for j in 0..i {
            if nums[i] > nums[j] {
                best = best.max(dp[j]);
            }
        }

        dp[i] = best + 1;
    }
    *dp.iter().max().unwrap_or(&0)
}

pub fn box_stacking_max_height(mut cuboids: Vec<Vec<i32>>) -> i32 {
    cuboids.sort_unstable_by_key(|x| x[0]);
    let mut dp: Vec<i32> = cuboids.iter().map(|x| x[2]).collect();

    for i in 0..cuboids.len() {
        let mut best = 0;
        for j in 0..i {
            if cuboids[i][1] >= cuboids[j][1] {
                best = best.max(dp[j]);
            }
        }
        dp[i] += best;
    }

    *dp.iter().max().unwrap_or(&0)
}

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let t1: Vec<u8> = text1.chars().map(|x| x as u8).collect();
    let t2: Vec<u8> = text2.chars().map(|x| x as u8).collect();
    let m = t1.len();
    let n = t2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if t1[i - 1] == t2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
            }
        }
    }

    dp[m][n]
}

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut found = std::collections::HashSet::new();
    let mut ans = vec![];
    for num in nums {
        if found.contains(&num) {
            ans.push(num);
            if ans.len() == 2 {
                break;
            }
        }
        found.insert(num);
    }
    ans
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Self>>,
}

impl ListNode {
    #[inline]
    const fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

// pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let hs: std::collections::HashSet<i32> = nums.into_iter().collect();
//     let mut dummy = Box::new(ListNode::new(-1));
//     let mut dummy_ptr = dummy.as_mut();

//     let mut prev: Option<Box<ListNode>> = None;
//     let mut ptr = head.as_mut().unwrap().as_mut();
//     while let Some(mut next) = ptr.next {
//         if !hs.contains(&next.val) {
//             ptr = next.next.as_mut().unwrap().as_mut();
//         } else {
//             ptr = &mut next;
//         }
//     }

//     dummy.next
// }

pub fn modified_list(nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let set: std::collections::HashSet<_> = nums.into_iter().collect();

    while matches!(head.as_ref(), Some(node) if set.contains(&node.val)) {
        head = head.take().unwrap().next;
    }

    let mut current = head.as_mut();
    while let Some(node) = current {
        while matches!(node.next.as_ref(), Some(next) if set.contains(&next.val)) {
            node.next = node.next.take().unwrap().next;
        }
        current = node.next.as_mut();
    }

    head
}

pub fn modified_list_copy(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let hs: std::collections::HashSet<_> = nums.into_iter().collect();
    let mut dummy = Box::new(ListNode::new(-1));
    let mut dummy_ptr = dummy.as_mut();

    let mut ptr = head;
    while let Some(next) = ptr {
        if !hs.contains(&next.val) {
            dummy_ptr.next = Some(Box::new(ListNode::new(next.val)));
            dummy_ptr = dummy_ptr.next.as_mut().unwrap().as_mut();
        }
        ptr = next.next;
    }

    dummy.next
}

pub fn count_key_changes(s: String) -> i32 {
    let letters: Vec<char> = s.chars().collect();
    let mut prev = letters[0].to_ascii_lowercase();
    let mut ans = 0;
    for letter in letters.iter().skip(1) {
        if letter.to_ascii_lowercase() != prev {
            prev = letter.to_ascii_lowercase();
            ans += 1;
        }
    }
    ans
}

pub fn differ_by_one(dict: Vec<String>) -> bool {
    let dict = dict
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let n = dict.len();
    for i in 0..n {
        for j in i + 1..n {
            let mut diff = 0;
            #[allow(clippy::needless_range_loop)]
            for k in 0..dict[i].len() {
                if dict[i][k] != dict[j][k] {
                    diff += 1;
                }
                if diff >= 2 {
                    break;
                }
            }
            if diff == 1 {
                return true;
            }
        }
    }
    false
}

pub fn input_binary_search() {
    info!("input low point");
    let mut l;
    loop {
        let mut low_point = String::new();
        if std::io::stdin().read_line(&mut low_point).is_ok()
            && let Ok(val) = low_point.trim().parse::<f32>()
        {
            l = val;
            break;
        }
    }

    info!("input high point");
    let mut r;
    loop {
        let mut high_point = String::new();
        if std::io::stdin().read_line(&mut high_point).is_ok()
            && let Ok(val) = high_point.trim().parse::<f32>()
        {
            r = val;
            break;
        }
    }

    while l < r && r - l > 1e-4 {
        let mid = l.midpoint(r);
        info!(
            "\nlow  {}\nmid  {}\nhigh {}\nenter 1 for lower or 2 for higher",
            l, mid, r
        );

        loop {
            let mut direction = String::new();
            if std::io::stdin().read_line(&mut direction).is_ok()
                && let Ok(val) = direction.trim().parse::<u8>()
            {
                if val == 2 {
                    l = mid;
                    break;
                } else if val == 1 {
                    r = mid;
                    break;
                }
            }
        }
    }

    info!("final value {}", l);
}

pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    let walls: HashSet<(i32, i32)> = walls.into_iter().map(|x| (x[0], x[1])).collect();
    let guards: HashSet<(i32, i32)> = guards.into_iter().map(|x| (x[0], x[1])).collect();

    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let bounds = |i: i32, j: i32| -> bool { i < m && j < n && i >= 0 && j >= 0 };

    let mut visited = HashSet::new();
    for (i, j) in &guards {
        visited.insert((*i, *j));

        for (di, dj) in &dirs {
            let (mut ni, mut nj) = (i + *di, j + *dj);
            while bounds(ni, nj) && !walls.contains(&(ni, nj)) && !guards.contains(&(ni, nj)) {
                visited.insert((ni, nj));
                ni += *di;
                nj += *dj;
            }
        }
    }

    m * n - walls.len() as i32 - visited.len() as i32
}

pub fn find_all_recipes(
    recipes: Vec<String>,
    ingredients: Vec<Vec<String>>,
    supplies: Vec<String>,
) -> Vec<String> {
    use std::collections::{HashSet, VecDeque};
    let mut supplies: HashSet<String> = supplies.into_iter().collect();
    let mut q: VecDeque<(usize, String)> = recipes.into_iter().enumerate().collect();

    let mut ans = vec![];
    let mut changed = i32::MAX;
    while changed != 0 {
        changed = 0;
        let mut next = VecDeque::new();
        while let Some((i, recipie)) = q.pop_front() {
            if ingredients[i].iter().all(|x| supplies.contains(x)) {
                changed += 1;
                supplies.insert(recipie.clone());
                ans.push(recipie);
            } else {
                next.push_back((i, recipie));
            }
        }
        q = next;
    }

    ans
}

pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let m = str1.len();
    let n = str2.len();
    if str1.clone() + &str2 != str2 + &str1 {
        return String::new();
    }

    const fn gcd(mut n: usize, mut m: usize) -> usize {
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }

    str1.chars().take(gcd(m, n)).collect()

    // let s1: Vec<char> = str1.chars().collect();
    // let s2: Vec<char> = str2.chars().collect();

    // let mut largest = String::new();
    // let mut curr = vec![];

    // for i in 0..s1.len().min(s2.len()) {
    //     if s1[i] != s2[i] {
    //         break;
    //     }

    //     curr.push(s1[i]);
    //     if s1.len().is_multiple_of(curr.len()) && s2.len().is_multiple_of(curr.len()) {
    //         let v1 = (0..s1.len()).all(|j| curr[j % curr.len()] == s1[j]);
    //         let v2 = (0..s2.len()).all(|j| curr[j % curr.len()] == s2[j]);
    //         if v1 && v2 {
    //             largest = curr.iter().copied().collect();
    //         }
    //     }
    // }

    // largest
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let most_candy = *candies.iter().max().unwrap();
    let mut result = vec![false; candies.len()];
    for (i, candy) in candies.iter().enumerate() {
        if *candy + extra_candies >= most_candy {
            result[i] = true;
        }
    }
    result
}

pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    for i in 0..flowerbed.len() {
        if *flowerbed.get(i - 1).unwrap_or(&0) == 0
            && flowerbed[i] == 0
            && *flowerbed.get(i + 1).unwrap_or(&0) == 0
        {
            flowerbed[i] = 1;
            n -= 1;
        }
    }
    n <= 0
}

pub fn reverse_vowels(s: String) -> String {
    let vowels: std::collections::HashSet<char> = String::from("aeiouAEIOU").chars().collect();
    let mut s: Vec<char> = s.chars().collect();
    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        while l < r && !vowels.contains(&s[l]) {
            l += 1;
        }
        while l < r && !vowels.contains(&s[r]) {
            r -= 1;
        }

        s.swap(l, r);
        l += 1;
        r = r.saturating_sub(1);
    }

    s.into_iter().collect()

    // let vowels: std::collections::HashSet<char> = String::from("aeiouAEIOU").chars().collect();
    // let mut found = vec![];

    // for letter in s.chars().rev() {
    //     if vowels.contains(&letter) {
    //         found.push(letter);
    //     }
    // }

    // let mut ans = String::new();
    // let mut found_idx = 0;
    // for letter in s.chars() {
    //     if vowels.contains(&letter) {
    //         ans += &found[found_idx].to_string();
    //         found_idx += 1;
    //     } else {
    //         ans += &letter.to_string();
    //     }
    // }
    // ans
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut first = i32::MAX;
    let mut second = i32::MAX;
    for num in nums {
        if first >= num {
            first = num;
        } else if second >= num {
            second = num;
        } else {
            return true;
        }
    }
    false
}

pub fn compress(chars: &mut [char]) -> i32 {
    let mut i = 0;
    let mut j = 0;
    while i < chars.len() {
        let mut count = 1;
        let mut k = i + 1;
        while k < chars.len() && chars[k] == chars[i] {
            count += 1;
            k += 1;
        }
        chars[j] = chars[i];
        j += 1;
        if count > 1 {
            for digit in count.to_string().chars() {
                chars[j] = digit;
                j += 1;
            }
        }
        i += count;
    }
    j as i32
}

pub fn max_operations1(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut ans = 0;
    while l < r {
        let val = nums[l] + nums[r];
        match val.cmp(&k) {
            std::cmp::Ordering::Equal => {
                ans += 1;
                l += 1;
                r -= 1;
            }
            std::cmp::Ordering::Less => l += 1,
            std::cmp::Ordering::Greater => r -= 1,
        }
    }

    ans
    // let mut hm = std::collections::HashMap::new();
    // for num in &nums {
    //     *hm.entry(*num).or_insert(0) += 1;
    // }

    // let mut ans = 0;
    // for num in &nums {
    //     if k - num == *num {
    //         if let Some(val) = hm.get_mut(num)
    //             && *val >= 2
    //         {
    //             *val -= 2;
    //             ans += 1;
    //         }
    //     } else if *hm.get(&(k - num)).unwrap_or(&0) > 0 && *hm.get(num).unwrap() > 0 {
    //         *hm.entry(k - *num).or_insert(0) -= 1;
    //         *hm.entry(*num).or_insert(0) -= 1;
    //         ans += 1;
    //     }
    // }

    // ans
}

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut total = nums.iter().take(k as usize).sum::<i32>();
    let mut ans: f64 = (total as f64) / (k as f64);
    let mut l = 0;
    #[allow(clippy::explicit_counter_loop)]
    for num in nums.iter().skip(k as usize) {
        total -= nums[l];
        l += 1;
        total += num;
        ans = ans.max((total as f64) / (k as f64));
    }

    ans
}

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut curr = 0;
    for g in gain {
        curr += g;
        ans = ans.max(curr);
    }
    ans
}

pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut presums = vec![nums[0]];
    for num in nums.iter().skip(1) {
        presums.push(*num + presums[presums.len() - 1]);
    }

    for i in 0..presums.len() {
        let left = *presums.get(i - 1).unwrap_or(&0);
        let right = presums[presums.len() - 1] - presums[i];
        if left == right {
            return i as i32;
        }
    }
    -1
}

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut p1 = cost[0];
    let mut p2 = cost[1];
    let mut curr;

    for c in cost.iter().skip(2) {
        curr = p1.min(p2) + c;
        p1 = p2;
        p2 = curr;
    }
    p1.min(p2)

    // let mut dp = vec![0; cost.len() + 1];
    // dp[0] = cost[0];
    // dp[1] = cost[1];

    // for i in 2..cost.len() {
    //     dp[i] = dp[i - 1].min(dp[i - 2]) + cost[i];
    // }

    // dp[cost.len() - 1].min(dp[cost.len() - 2])
}

pub fn min_cost_2(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut curr = 0;
    let mut largest = 0;
    let mut ans = 0;
    let mut prev = ' ';
    for (i, color) in colors.chars().enumerate() {
        if color == prev {
            largest = largest.max(needed_time[i]);
            curr += needed_time[i];
        } else {
            ans += curr - largest;
            largest = needed_time[i];
            curr = needed_time[i];
            prev = color;
        }
    }

    ans + curr - largest
}

pub fn maximum_number_of_ones(width: i32, height: i32, side_length: i32, max_ones: i32) -> i32 {
    let mut count = vec![];

    for r in 0..side_length {
        for c in 0..side_length {
            count.push((1 + (width - c - 1) / side_length) * (1 + (height - r - 1) / side_length));
        }
    }

    count.sort_unstable_by(|a, b| b.cmp(a));
    count.iter().take(max_ones as usize).sum()
}

pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let num_courses = num_courses as usize;
    let mut adj_list = vec![vec![]; num_courses];
    let mut in_degree = vec![0; num_courses];

    for preq in prerequisites {
        let (course, required) = (preq[0] as usize, preq[1] as usize);
        adj_list[required].push(course);
        in_degree[course] += 1;
    }

    let mut visited = vec![false; num_courses];
    let mut st: Vec<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == 0)
        .map(|(i, _)| i)
        .collect();

    let mut ans = vec![];
    while let Some(course) = st.pop() {
        if visited[course] {
            continue;
        }
        visited[course] = true;
        ans.push(course as i32);

        for next in &adj_list[course] {
            in_degree[*next] -= 1;
            if in_degree[*next] == 0 {
                st.push(*next);
            }
        }
    }

    if ans.len() != num_courses {
        return vec![];
    }

    ans
}

pub fn min_time(n: i32, edges: Vec<Vec<i32>>, mut has_apple: Vec<bool>) -> i32 {
    has_apple[0] = true;
    let mut adj_list = vec![vec![]; n as usize];
    let mut degrees = vec![0; n as usize];
    for edge in edges {
        let (curr, next) = (edge[0] as usize, edge[1] as usize);
        adj_list[curr].push(next);
        adj_list[next].push(curr);
        degrees[curr] += 1;
        degrees[next] += 1;
    }

    let mut st: std::collections::VecDeque<usize> = degrees
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == 1)
        .map(|(i, _)| i)
        .collect();

    while let Some(top) = st.pop_front() {
        if has_apple[top] {
            continue;
        }

        for next in &adj_list[top] {
            if degrees[*next] > 0 {
                degrees[top] -= 1;
                degrees[*next] -= 1;
                if degrees[*next] == 1 {
                    st.push_back(*next);
                }
            }
        }
    }

    degrees.iter().sum()
}

pub fn find_x_sum_easy(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::with_capacity(51);

    for num in nums.iter().take(k as usize) {
        *hm.entry(*num).or_insert(0) += 1;
    }

    let mut ans = vec![];
    for i in 0..=(nums.len() - k as usize) {
        let curr = hm
            .iter()
            .sorted_unstable_by(|a, b| b.1.cmp(a.1).then(b.0.cmp(a.0)))
            .take(x as usize)
            .fold(0, |acc, (key, value)| acc + *key * *value);

        ans.push(curr);
        *hm.entry(nums[i]).or_default() -= 1;
        *hm.entry(*nums.get(i + k as usize).unwrap_or(&0))
            .or_default() += 1;
    }

    ans
}

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut uf: Vec<usize> = (0..edges.len()).collect();
    let mut rank = vec![1; edges.len()];

    fn union_find(uf: &[usize], mut curr: usize) -> usize {
        while uf[curr] != curr {
            curr = uf[curr];
        }
        curr
    }

    fn union_parents(uf: &mut [usize], rank: &mut [i32], x: usize, y: usize) -> bool {
        let p1 = union_find(uf, x);
        let p2 = union_find(uf, y);
        if p1 == p2 {
            return false;
        }

        if rank[p1] > rank[p2] {
            rank[p2] += rank[p1];
            uf[p1] = p2;
        } else {
            rank[p1] += rank[p2];
            uf[p2] = p1;
        }

        true
    }

    for edge in edges {
        if !union_parents(
            &mut uf,
            &mut rank,
            edge[0] as usize - 1,
            edge[1] as usize - 1,
        ) {
            return edge;
        }
    }

    vec![]
}

pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut uf: Vec<_> = (0..accounts.len()).collect();
    let mut rank = vec![1; accounts.len()];
    fn find(uf: &[usize], mut i: usize) -> usize {
        while uf[i] != i {
            i = uf[i];
        }
        uf[i]
    }

    fn union_parents(uf: &mut [usize], rank: &mut [i32], a: usize, b: usize) -> bool {
        let p1 = find(uf, a);
        let p2 = find(uf, b);
        if p1 == p2 {
            return false;
        }

        if rank[p1] < rank[p2] {
            rank[p2] += rank[p1];
            uf[p1] = p2;
        } else {
            rank[p1] += rank[p2];
            uf[p2] = p1;
        }

        true
    }

    use std::collections::HashMap;
    let mut email_to_account = HashMap::new();
    for (i, account) in accounts.iter().enumerate() {
        for email in account.iter().skip(1) {
            if let Some(val) = email_to_account.get(email) {
                let _ = union_parents(&mut uf, &mut rank, i, *val);
            } else {
                email_to_account.insert(email, i);
            }
        }
    }

    let mut email_groups = HashMap::new();
    for (key, value) in email_to_account {
        (*email_groups
            .entry(find(&uf, value))
            .or_insert_with(Vec::new))
        .push(key.to_owned());
    }

    let mut ans: Vec<Vec<String>> = vec![];
    for (i, mut g) in email_groups {
        g.sort_unstable();
        ans.push(
            std::iter::once(accounts[i][0].clone())
                .chain(g.into_iter())
                .collect(),
        );
    }

    ans
}

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    use std::collections::{BTreeSet, HashMap};
    let x = x as usize;
    let k = k as usize;

    let mut hm = HashMap::new();
    for num in nums.iter().take(k) {
        *hm.entry(*num).or_insert(0) += 1;
    }

    let init_values: Vec<(i32, i32)> = hm
        .iter()
        .map(|(&key, &value)| (value, key))
        .sorted_unstable()
        .rev()
        .collect();

    let mut curr = init_values
        .iter()
        .take(x)
        .fold(0, |acc, (value, key)| acc + *value as i64 * *key as i64);

    let mut largest: BTreeSet<(i32, i32)> = init_values
        .iter()
        .take(x)
        .map(|(value, key)| (*value, *key))
        .collect();

    let mut smallest: BTreeSet<(i32, i32)> = init_values.into_iter().skip(x).collect();

    let mut ans = vec![];
    for i in k..nums.len() {
        ans.push(curr);

        let mut update = |key: i32, delta: i32| {
            if let Some(val) = hm.get_mut(&key) {
                let e = (*val, key);
                if largest.contains(&e) {
                    largest.remove(&e);
                    curr -= *val as i64 * key as i64;
                } else {
                    smallest.remove(&e);
                }
            }

            let e = hm.entry(key).or_default();
            *e += delta;
            if *e == 0 {
                let _ = e;
                hm.remove_entry(&key);
            } else {
                smallest.insert((*e, key));
            }
        };

        update(nums[i - k], -1);
        update(nums[i], 1);

        while largest.len() < x
            && let Some((value, key)) = smallest.pop_last()
        {
            largest.insert((value, key));
            curr += key as i64 * value as i64;
        }

        while !smallest.is_empty() && smallest.last().unwrap() > largest.first().unwrap() {
            let (value, key) = smallest.pop_last().unwrap();
            curr += key as i64 * value as i64;
            largest.insert((value, key));
            let (value, key) = largest.pop_first().unwrap();
            curr -= key as i64 * value as i64;
            smallest.insert((value, key));
        }
    }

    ans.push(curr);
    ans
}

pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    let mut count = 1;
    let mut j = 1;

    for i in 1..nums.len() {
        if nums[i] == nums[i - 1] {
            count += 1;
            if count > 2 {
                continue;
            }
        } else {
            count = 1;
        }
        nums[j] = nums[i];
        j += 1;
    }

    j as i32
}

pub fn add_binary(a: String, b: String) -> String {
    let s1: Vec<char> = a.chars().collect();
    let s2: Vec<char> = b.chars().collect();
    let mut ans = vec![];
    let m = s1.len();
    let n = s2.len();
    let mut carry = false;

    for i in 0..(n.min(m)) {
        if s1[m - 1 - i] == s2[n - 1 - i] {
            ans.push(if carry { '1' } else { '0' });
            carry = s1[m - 1 - i] == '1';
        } else {
            ans.push(if carry { '0' } else { '1' });
        }
    }

    let mut check_single = |c: char| {
        if c == '0' {
            ans.push(if carry { '1' } else { '0' });
            carry = false;
        } else {
            ans.push(if carry { '0' } else { '1' });
        }
    };

    for i in (n.min(m))..m {
        check_single(s1[m - 1 - i]);
    }

    for i in (n.min(m))..n {
        check_single(s2[n - 1 - i]);
    }

    if carry {
        ans.push('1');
    }

    ans.into_iter().rev().collect()
}

pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap, HashSet};

    let c = c as usize;
    let mut adj_list = vec![vec![]; c];
    for con in connections {
        let (c1, c2) = (con[0] as usize - 1, con[1] as usize - 1);
        adj_list[c1].push(c2);
        adj_list[c2].push(c1);
    }

    let mut group_to_heap = HashMap::new();
    let mut station_to_group = vec![0; c];

    let mut st: Vec<_> = (0..c).enumerate().collect();
    let mut visited = vec![false; c];
    while let Some((station, group)) = st.pop() {
        if visited[station] {
            continue;
        }
        visited[station] = true;

        group_to_heap
            .entry(group)
            .or_insert_with(BinaryHeap::new)
            .push(Reverse(station));

        station_to_group[station] = group;

        for adj_station in &adj_list[station] {
            st.push((*adj_station, group));
        }
    }

    let mut ans = vec![];
    let mut deleted = HashSet::new();
    for query in queries {
        let (op, station) = (query[0], (query[1] - 1) as usize);
        if op == 1 {
            let stations = group_to_heap.get_mut(&station_to_group[station]).unwrap();

            if !deleted.contains(&station) {
                ans.push(station as i32 + 1);
                continue;
            }

            let mut m_station = None::<usize>;
            while let Some(Reverse(t)) = stations.peek() {
                if !deleted.contains(t) {
                    m_station = Some(*t);
                    break;
                }
                stations.pop();
            }

            if let Some(m_station) = m_station {
                ans.push(m_station as i32 + 1);
            } else {
                ans.push(-1);
            }
        } else if op == 2 {
            deleted.insert(station);
        }
    }

    ans
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;
    let mut adj_list = vec![vec![]; num_courses];
    let mut in_degree = vec![0; num_courses];
    for preq in prerequisites {
        let (p1, p2) = (preq[0] as usize, preq[1] as usize);
        adj_list[p2].push(p1);
        in_degree[p1] += 1;
    }

    let mut st: Vec<_> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(i, x)| (*x == 0).then_some(i))
        .collect();

    let mut visited = vec![false; num_courses];
    while let Some(course) = st.pop() {
        if visited[course] {
            continue;
        }
        visited[course] = true;

        for req_course in &adj_list[course] {
            in_degree[*req_course] -= 1;
            if in_degree[*req_course] == 0 {
                st.push(*req_course);
            }
        }
    }

    visited.iter().all(|x| *x)
}

pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
    let r = r as usize;
    let n = stations.len();
    let mut diff = vec![0; n + 1];
    for (i, st) in stations.iter().enumerate() {
        diff[i.saturating_sub(r)] += *st as i64;
        diff[n.min(i + r + 1)] -= *st as i64;
    }

    let mut left = *stations.iter().min().unwrap() as i64;
    let mut right = stations.iter().map(|x| *x as i64).sum::<i64>() + k as i64;

    let check = |val: i64| -> bool {
        let mut c_diff = diff.clone();
        let mut curr = 0;
        let mut r_stations = k as i64;
        for i in 0..n {
            curr += c_diff[i];
            if curr < val {
                let n_stations = val - curr;
                if r_stations < n_stations {
                    return false;
                }
                r_stations -= n_stations;

                c_diff[n.min(i + 2 * r + 1)] -= n_stations;
                curr += n_stations;
            }
        }
        true
    };

    let mut ans = 0;
    while left <= right {
        let mid = left.midpoint(right);

        if check(mid) {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    ans
}

pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
    // not seeing any online solutions trying to sort the input array
    // so that the check fn hits false sooner if it was going to hit false anyway
    piles.sort_unstable_by(|a, b| b.cmp(a));
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();
    let mut ans = l;

    let check = |val: i32| -> bool {
        let mut remain = h;
        for pile in &piles {
            remain -= (*pile as f64 / val as f64).ceil() as i32;
            if remain < 0 {
                return false;
            }
        }
        true
    };

    while l <= r {
        let mid = l.midpoint(r);
        if check(mid) {
            r = mid - 1;
        } else {
            ans = mid + 1;
            l = mid + 1;
        }
    }

    ans
}

pub fn min_eating_speed_wo_sort(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();
    let mut ans = l;

    let check = |val: i32| -> bool {
        let mut remain = h;
        for pile in &piles {
            remain -= (*pile as f64 / val as f64).ceil() as i32;
            if remain < 0 {
                return false;
            }
        }
        true
    };

    while l <= r {
        let mid = l.midpoint(r);
        if check(mid) {
            r = mid - 1;
        } else {
            ans = mid + 1;
            l = mid + 1;
        }
    }

    ans
}

// spoilers no sorting tends to be better
pub fn test_diff_in_sorting() {
    let mut r = rand::rng();
    let mut sorting_count = 0;
    let mut no_sorting_count = 0;
    let mut total_sorting = 0f64;
    let mut total_no_sorting = 0f64;

    let total_tests = 1000;
    let t = std::time::Instant::now();
    for _ in 0..total_tests {
        let input_vec: Vec<_> = (0..100_000)
            .map(|_| r.random_range(0..1_000_000_000))
            .collect();
        let input_vec2: Vec<_> = input_vec.clone();
        let hours = r.random_range(input_vec.len() as i32..i32::MAX);

        let t1 = std::time::Instant::now();
        min_eating_speed(input_vec, hours);
        let e1 = t1.elapsed().as_micros();
        let _ = t1;

        let t2 = std::time::Instant::now();
        min_eating_speed_wo_sort(input_vec2, hours);
        let e2 = t2.elapsed().as_micros();
        let _ = t2;

        match e1.cmp(&e2) {
            std::cmp::Ordering::Less => sorting_count += 1,
            std::cmp::Ordering::Greater => no_sorting_count += 1,
            std::cmp::Ordering::Equal => {}
        }

        total_sorting += e1 as u32 as f64;
        total_no_sorting += e2 as u32 as f64;
        info!("{} {}", e1, e2);
    }

    info!("test time {}ms", t.elapsed().as_millis());
    info!(
        "sorting {:#} avg time {:#} micros",
        sorting_count,
        total_sorting / total_tests as f64
    );
    info!(
        "no sorting {:#} avg time {:#} micros",
        no_sorting_count,
        total_no_sorting / total_tests as f64
    );
}

pub fn num_rabbits(answers: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for answer in answers {
        *hm.entry(answer).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (key, value) in hm {
        let groups = (value / (key + 1)) * (key + 1);
        ans += groups;
        if groups != value {
            ans += key + 1;
        }
    }
    ans
}

struct TwoSum {
    hm: std::collections::HashMap<i32, i32>,
}

impl TwoSum {
    fn new() -> Self {
        Self {
            hm: std::collections::HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        *self.hm.entry(number).or_default() += 1;
    }

    fn find(&self, value: i32) -> bool {
        for (key, val) in &self.hm {
            if value - *key == *key {
                if *val > 1 {
                    return true;
                }
                continue;
            }
            if self.hm.contains_key(&(value - *key)) {
                return true;
            }
        }
        false
    }
}

pub fn test_two_sum_ds() {
    let mut ts = TwoSum::new();
    ts.add(1);
    ts.add(3);
    ts.add(5);
    assert!(ts.find(4));
    assert!(!ts.find(7));
}

pub fn minimum_one_bit_operations(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut k = 1u32;
    while 2i32.pow(k) <= n {
        k += 1;
    }
    k -= 1;

    2i32.pow(k + 1) - minimum_one_bit_operations(2i32.pow(k) ^ n)
}

pub fn min_moves(nums: Vec<i32>) -> i32 {
    let largest = *nums.iter().max().unwrap();
    let mut ans = 0;
    for num in nums {
        ans += largest - num;
    }
    ans
}

pub fn total_fruit(fruits: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    let mut l = 0;
    let mut ans = 0;
    for (r, f) in fruits.iter().enumerate() {
        *hm.entry(f).or_insert(0) += 1;
        while hm.len() > 2 {
            let e = hm.entry(&fruits[l]).or_default();
            *e -= 1;
            if *e == 0 {
                hm.remove(&fruits[l]);
            }
            l += 1;
        }
        ans = ans.max(r - l + 1);
    }
    ans as i32
}

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut curr = 0;
    let mut ans = 0;
    for (r, num) in nums.iter().enumerate() {
        while l < r && *num & curr != 0 {
            curr ^= nums[l];
            l += 1;
        }
        curr |= *num;
        ans = ans.max(r - l + 1);
    }
    ans as i32
}

pub const fn count_operations(mut num1: i32, mut num2: i32) -> i32 {
    let mut count = 0;
    while num1 > 0 && num2 > 0 {
        if num1 < num2 {
            num2 -= num1;
        } else {
            num1 -= num2;
        }
        count += 1;
    }
    count
}

#[derive(Default)]
struct TrieNode {
    pub hm: std::collections::HashMap<char, Self>,
    pub is_word_ending: bool,
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut traverse = &mut self.root.hm;
        for (i, letter) in word.chars().enumerate() {
            let e = traverse.entry(letter.to_ascii_lowercase()).or_default();
            if i == word.len() - 1 {
                e.is_word_ending = true;
            }
            traverse = &mut e.hm;
        }
    }

    fn search(&self, word: String) -> bool {
        let mut traverse = &self.root.hm;
        for (i, letter) in word.chars().enumerate() {
            if let Some(e) = traverse.get(&letter.to_ascii_lowercase()) {
                if i == word.len() - 1 && !e.is_word_ending {
                    return false;
                }
                traverse = &e.hm;
            } else {
                return false;
            }
        }
        true
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut traverse = &self.root.hm;
        for letter in prefix.chars() {
            if let Some(e) = traverse.get(&letter.to_ascii_lowercase()) {
                traverse = &e.hm;
            } else {
                return false;
            }
        }
        true
    }
}

pub fn trie_test() {
    let mut obj = Trie::new();
    obj.insert("apple".into());
    assert!(obj.search("apple".into()));
    assert!(obj.starts_with("app".into()));
}

#[derive(Default)]
struct TrieNode2 {
    pub hm: std::collections::HashMap<char, Self>,
    pub words_end_here: i32,
    pub words_prefix_here: i32,
}

#[derive(Default)]
struct Trie2 {
    root: TrieNode2,
}

impl Trie2 {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, word: String) {
        let mut traverse = &mut self.root.hm;
        for (i, letter) in word.chars().enumerate() {
            let e = traverse.entry(letter.to_ascii_lowercase()).or_default();
            if i == word.len() - 1 {
                e.words_end_here += 1;
            }
            e.words_prefix_here += 1;
            traverse = &mut e.hm;
        }
    }

    fn count_words_equal_to(&self, word: String) -> i32 {
        let mut traverse = &self.root.hm;
        for (i, letter) in word.chars().enumerate() {
            if let Some(e) = traverse.get(&letter.to_ascii_lowercase()) {
                if i == word.len() - 1 {
                    return e.words_end_here;
                }
                traverse = &e.hm;
            } else {
                return 0;
            }
        }
        0
    }

    fn count_words_starting_with(&self, word: String) -> i32 {
        let mut traverse = &self.root.hm;
        for (i, letter) in word.chars().enumerate() {
            if let Some(e) = traverse.get(&letter.to_ascii_lowercase()) {
                if i == word.len() - 1 {
                    return e.words_prefix_here;
                }
                traverse = &e.hm;
            } else {
                return 0;
            }
        }
        0
    }

    fn erase(&mut self, prefix: String) {
        let mut traverse = &mut self.root.hm;
        for (i, letter) in prefix.chars().enumerate() {
            let e = traverse.entry(letter.to_ascii_lowercase()).or_default();
            if i == prefix.len() - 1 {
                e.words_end_here -= 1;
            }
            e.words_prefix_here -= 1;
            traverse = &mut e.hm;
        }
    }
}

pub fn trie2_test() {
    let mut obj = Trie2::new();
    obj.insert("apple".into());
    assert!(obj.count_words_equal_to("apple".into()) == 1);
    assert!(obj.count_words_starting_with("app".into()) == 1);
    obj.erase("apple".into());
    assert!(obj.count_words_equal_to("apple".into()) == 0);
    assert!(obj.count_words_starting_with("app".into()) == 0);
}

pub fn min_interval(mut intervals: Vec<Vec<i32>>, mut queries: Vec<i32>) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};
    intervals.sort_unstable();
    let mut q_to_idx = HashMap::new();
    for (i, q) in queries.iter().enumerate() {
        q_to_idx.entry(*q).or_insert_with(Vec::new).push(i);
    }
    let mut ans = vec![0; queries.len()];

    queries.sort_unstable();
    queries.dedup();
    let mut curr_intervals = BinaryHeap::new();

    let mut l = 0;
    for q in queries {
        while l < intervals.len() && q >= intervals[l][0] {
            curr_intervals.push(Reverse((
                intervals[l][1] - intervals[l][0] + 1,
                intervals[l][1],
            )));

            l += 1;
        }

        while let Some(Reverse((_, r))) = curr_intervals.peek()
            && q > *r
        {
            curr_intervals.pop();
        }

        if let Some(Reverse((dist, _))) = curr_intervals.peek() {
            for i in &q_to_idx[&q] {
                ans[*i] = *dist;
            }
        } else {
            for i in &q_to_idx[&q] {
                ans[*i] = -1;
            }
        }
    }

    ans
}

pub fn min_operations2(nums: Vec<i32>) -> i32 {
    let mut st = vec![];
    let mut ans = 0;
    for num in nums {
        while let Some(top) = st.last()
            && num < *top
        {
            st.pop();
        }
        if num == 0 {
            continue;
        }
        if st.last().map_or(0, |x| *x) != num {
            ans += 1;
        }
        st.push(num);
    }
    ans
}

pub fn find_max_form_top_down(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let mut counts = vec![];
    for s in strs {
        let (mut zeros, mut ones) = (0, 0);
        for letter in s.chars() {
            if letter == '0' {
                zeros += 1;
            } else if letter == '1' {
                ones += 1;
            }
        }
        counts.push((zeros, ones));
    }
    use std::collections::HashMap;

    fn dp(
        counts: &[(i32, i32)],
        cache: &mut HashMap<(usize, (i32, i32)), i32>,
        i: usize,
        total: (i32, i32),
        m: i32,
        n: i32,
    ) -> i32 {
        if total.0 > m || total.1 > n {
            return i32::MIN;
        }
        if i >= counts.len() {
            return 0;
        }

        if let Some(ans) = cache.get(&(i, total)) {
            return *ans;
        }

        let next_total = (total.0 + counts[i].0, total.1 + counts[i].1);
        let count_curr = dp(counts, cache, i + 1, next_total, m, n) + 1;
        let skip_curr = dp(counts, cache, i + 1, total, m, n);

        let ans = count_curr.max(skip_curr);
        cache.insert((i, total), ans);
        ans
    }

    let mut cache = HashMap::new();

    dp(&counts, &mut cache, 0, (0, 0), m, n)
}

pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for s in &strs {
        let zeros = s.matches('0').count();
        let ones = s.len() - zeros;

        for t1 in (zeros..=m).rev() {
            for t2 in (ones..=n).rev() {
                dp[t1][t2] = dp[t1][t2].max(dp[t1 - zeros][t2 - ones] + 1);
            }
        }
    }

    dp[m][n]
}

pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();
    let mut dp = vec![vec![0; n]; m];
    dp[0][0] = i32::from(obstacle_grid[0][0] != 1);
    for i in 1..m {
        dp[i][0] = i32::from(obstacle_grid[i][0] != 1 && dp[i - 1][0] == 1);
    }
    for j in 1..n {
        dp[0][j] = i32::from(obstacle_grid[0][j] != 1 && dp[0][j - 1] == 1);
    }
    for i in 1..m {
        for j in 1..n {
            if obstacle_grid[i][j] == 1 {
                dp[i][j] = 0;
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }

    dp[m - 1][n - 1]
}

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut ans = 0;

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if matrix[i][j] == '1' {
                dp[i][j] = dp[i + 1][j].min(dp[i + 1][j + 1]).min(dp[i][j + 1]) + 1;
                ans = ans.max(dp[i][j]);
            }
        }
    }

    ans * ans
}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let ones = nums.iter().filter(|x| **x == 1).count();
    if ones > 0 {
        return nums.len() as i32 - ones as i32;
    }
    let gcd = |mut a: i32, mut b: i32| -> i32 {
        if b > a {
            std::mem::swap(&mut a, &mut b);
        }

        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    };

    let mut ans = i32::MAX;
    for i in 0..nums.len() {
        let mut g = 0;
        for (j, nj) in nums.iter().enumerate().skip(i) {
            if j - i + 1 >= ans as usize {
                break;
            }
            g = gcd(g, *nj);
            if g == 1 {
                ans = j as i32 - i as i32 + 1;
                break;
            }
        }
    }

    if ans == i32::MAX {
        return -1;
    }

    ans - 1 + nums.len() as i32 - 1
}

#[derive(Default)]
struct MyNode {
    pub val: i32,
    pub next: Option<*mut Self>,
    pub prev: Option<*mut Self>,
}

impl MyNode {
    const fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}

struct MyLinkedList {
    front: Option<*mut MyNode>,
    tail: Option<*mut MyNode>,
    size: i32,
}

impl MyLinkedList {
    const fn new() -> Self {
        Self {
            front: None,
            tail: None,
            size: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index >= self.size || index < 0 {
            return -1;
        }
        if index < self.size / 2 {
            let mut i = 0;
            let mut ptr = self.front;
            while let Some(curr) = ptr {
                if i == index {
                    return unsafe { (*curr).val };
                }
                ptr = unsafe { (*curr).next };
                i += 1;
            }
        } else {
            let mut i = self.size - 1;
            let mut ptr = self.tail;
            while let Some(curr) = ptr {
                if i == index {
                    return unsafe { (*curr).val };
                }
                ptr = unsafe { (*curr).prev };
                i -= 1;
            }
        }
        -1
    }

    fn add_at_head(&mut self, val: i32) {
        let new_front = Box::into_raw(Box::new(MyNode::new(val)));
        if let Some(front) = self.front {
            unsafe {
                (*front).prev = Some(new_front);
                (*new_front).next = Some(front);
            };
            self.front = Some(new_front);
        } else {
            self.front = Some(new_front);
            self.tail = Some(new_front);
        }

        self.size += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_tail = Box::into_raw(Box::new(MyNode::new(val)));
        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(new_tail);
                (*new_tail).prev = Some(tail);
            };
            self.tail = Some(new_tail);
        } else {
            self.tail = Some(new_tail);
            self.front = Some(new_tail);
        }
        self.size += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index > self.size {
            return;
        } else if index == 0 {
            self.add_at_head(val);
            return;
        } else if index == self.size {
            self.add_at_tail(val);
            return;
        }
        if index < self.size / 2 {
            let mut i = 0;
            let mut ptr = self.front;
            while let Some(curr) = ptr {
                if i == index {
                    let new_node = Box::into_raw(Box::new(MyNode::new(val)));
                    unsafe {
                        if let Some(prev) = (*curr).prev {
                            (*prev).next = Some(new_node);
                            (*new_node).prev = Some(prev);
                        }
                        (*new_node).next = Some(curr);
                        (*curr).prev = Some(new_node);
                    }

                    self.size += 1;
                    return;
                }
                ptr = unsafe { (*curr).next };
                i += 1;
            }
        } else {
            let mut i = self.size - 1;
            let mut ptr = self.tail;
            while let Some(curr) = ptr {
                if i == index {
                    let new_node = Box::into_raw(Box::new(MyNode::new(val)));
                    unsafe {
                        if let Some(prev) = (*curr).prev {
                            (*prev).next = Some(new_node);
                            (*new_node).prev = Some(prev);
                        }
                        (*new_node).next = Some(curr);
                        (*curr).prev = Some(new_node);
                    }

                    self.size += 1;
                    return;
                }
                ptr = unsafe { (*curr).prev };
                i -= 1;
            }
        }
    }

    fn pop_first(&mut self) -> i32 {
        let front = self.front.unwrap();
        unsafe {
            if let Some(next) = (*front).next {
                (*next).prev = None;
                self.front = Some(next);
            } else {
                self.front = None;
                self.tail = None;
            }
        }
        self.size -= 1;

        let val = unsafe { (*front).val };
        unsafe { drop(Box::from_raw(front)) }
        val
    }

    fn pop_last(&mut self) -> i32 {
        let tail = self.tail.unwrap();
        unsafe {
            if let Some(prev) = (*tail).prev {
                (*prev).next = None;
                self.tail = Some(prev);
            } else {
                self.front = None;
                self.tail = None;
            }
        }
        self.size -= 1;
        let val = unsafe { (*tail).val };
        unsafe { drop(Box::from_raw(tail)) }
        val
    }

    fn delete_at_index(&mut self, index: i32) {
        if index >= self.size {
            return;
        } else if index == 0 {
            self.pop_first();
            return;
        } else if index == self.size - 1 {
            self.pop_last();
            return;
        }

        if index < self.size / 2 {
            let mut i = 0;
            let mut ptr = self.front;
            while let Some(curr) = ptr {
                if i == index {
                    unsafe {
                        if let Some(prev) = (*curr).prev {
                            (*prev).next = (*curr).next;
                        }
                        if let Some(next) = (*curr).next {
                            (*next).prev = (*curr).prev;
                        }
                    }
                    unsafe { drop(Box::from_raw(curr)) };
                    self.size -= 1;
                    return;
                }
                ptr = unsafe { (*curr).next };
                i += 1;
            }
        } else {
            let mut i = self.size - 1;
            let mut ptr = self.tail;
            while let Some(curr) = ptr {
                if i == index {
                    unsafe {
                        if let Some(prev) = (*curr).prev {
                            (*prev).next = (*curr).next;
                        }
                        if let Some(next) = (*curr).next {
                            (*next).prev = (*curr).prev;
                        }
                    }
                    unsafe { drop(Box::from_raw(curr)) };
                    self.size -= 1;
                    return;
                }
                ptr = unsafe { (*curr).prev };
                i -= 1;
            }
        }
    }
}

pub fn test_linked_list() {
    let mut obj = MyLinkedList::new();
    assert!(obj.get(0) == -1);

    obj.add_at_head(1);
    assert!(obj.get(0) == 1);

    obj.pop_first();
    assert!(obj.get(0) == -1);

    obj.add_at_head(1);
    assert!(obj.get(0) == 1);

    obj.add_at_tail(3);
    assert!(obj.get(1) == 3);

    obj.pop_last();
    assert!(obj.get(1) == -1);

    obj.add_at_tail(3);
    assert!(obj.get(1) == 3);

    obj.add_at_index(1, 2);

    assert!(obj.get(1) == 2);

    obj.delete_at_index(1);
    assert!(obj.get(1) == 3);
}

pub fn num_kings(target: f64, chips: i32, base_mult: i32, retriggers: i32) -> i32 {
    ((target / chips as f64 / base_mult as f64).log(1.5) / (retriggers + 1) as f64).ceil() as i32
}

pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
    use std::collections::HashSet;
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                rows.insert(i);
                cols.insert(j);
            }
        }
    }

    for row in rows {
        for val in &mut matrix[row] {
            *val = 0;
        }
    }

    for col in cols {
        for row in matrix.iter_mut() {
            row[col] = 0;
        }
    }
}

pub fn game_of_life(board: &mut [Vec<i32>]) {
    let mut changes = std::collections::HashMap::new();
    let m = board.len();
    let n = board[0].len();
    let check_live_neighbors = |i: usize, j: usize| -> i32 {
        let dirs = [
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (0, 1),
            (0, -1),
        ];
        let mut count = 0;
        for (di, dj) in dirs {
            let (ni, nj) = (i as i32 + di, j as i32 + dj);
            if ni >= 0
                && ni < m as i32
                && nj >= 0
                && nj < n as i32
                && board[ni as usize][nj as usize] == 1
            {
                count += 1;
            }
        }
        count
    };

    for (i, row) in board.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let live = check_live_neighbors(i, j);
            if *val == 1 && !(2..=3).contains(&live) {
                changes.insert((i, j), 0);
            } else if *val == 0 && live == 3 {
                changes.insert((i, j), 1);
            }
        }
    }

    for ((i, j), value) in changes {
        board[i][j] = value;
    }
}

// pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
//     let mut ans = vec![];
//     let m = matrix.len();
//     let n = matrix[0].len();

//     ans
// }

pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort_unstable();
    let all_diff = (arr[0] - arr[1]).abs();
    for i in 1..arr.len() - 1 {
        let diff = (arr[i] - arr[i + 1]).abs();
        if diff != all_diff {
            return false;
        }
    }
    true
}

pub fn pivot_integer(n: i32) -> i32 {
    let val = ((n * n + n) as f64 / 2f64).sqrt();
    if val - ((val as i32) as f64) > 1e-6 {
        return -1;
    }
    val as i32
}

pub fn is_palindrome_using_str(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    x.to_string().chars().collect::<Vec<char>>()
        == x.to_string().chars().rev().collect::<Vec<char>>()
}

pub const fn is_palindrome(mut x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut ans = 0;
    while x > ans {
        ans = ans * 10 + x % 10;
        x /= 10;
    }
    ans == x || ans / 10 == x
}

pub fn max_operations_w_heap_alloc(s: &str) -> i64 {
    let letters = s.chars().collect::<Vec<char>>();
    let mut ans = 0;
    let mut count_ones = 0;
    for i in 0..letters.len() - 1 {
        if letters[i] == '1' {
            count_ones += 1;
        }
        if letters[i] == '1' && letters[i + 1] == '0' {
            ans += count_ones;
        }
    }
    ans
}

pub fn max_operations(s: &str) -> i64 {
    let mut ans = 0;
    let mut count_ones = 0;
    for (curr, next) in s.chars().zip(s.chars().skip(1)) {
        if curr == '1' {
            count_ones += 1;
        }
        if curr == '1' && next == '0' {
            ans += count_ones;
        }
    }
    ans
}

pub fn is_ugly(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    for div in [2, 3, 5] {
        while n % div == 0 {
            n /= div;
        }
    }

    n == 1
}

pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    if k % 2 == 0 {
        return -1;
    }

    let mut r = 0;
    for i in 1..=k {
        r = (r * 10 + 1) % k;
        if r == 0 {
            return i;
        }
    }
    -1
}

pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut ans = vec![];
    for n in left..=right {
        let mut m = n;
        while m > 0 {
            let t = m % 10;
            if t == 0 || n % t != 0 {
                break;
            }
            m /= 10;
        }
        if m == 0 {
            ans.push(n);
        }
    }
    ans
}

pub const fn reverse(x: i32) -> i32 {
    if x == i32::MIN {
        return 0;
    }
    let mut n = x.abs();
    let mut ans = 0;
    while n > 0 {
        if ans > i32::MAX / 10 {
            return 0;
        }
        ans = ans * 10 + (n % 10);
        n /= 10;
    }
    if x > 0 { ans } else { -ans }
}

pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut diffs = vec![vec![0; n + 1]; n + 1];
    for q in queries {
        let (r1, c1, r2, c2) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
        diffs[r1][c1] += 1;
        diffs[r1][c2 + 1] -= 1;
        diffs[r2 + 1][c1] -= 1;
        diffs[r2 + 1][c2 + 1] += 1;
    }

    let mut ans = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let v1 = if i == 0 { 0 } else { diffs[i - 1][j] };
            let v2 = if j == 0 { 0 } else { diffs[i][j - 1] };
            let v3 = if j == 0 || i == 0 {
                0
            } else {
                diffs[i - 1][j - 1]
            };
            ans[i][j] = diffs[i][j] + v1 + v2 - v3;
        }
    }
    ans
}

struct Bank {
    pub accounts: Vec<i64>,
}

impl Bank {
    const fn new(balance: Vec<i64>) -> Self {
        Self { accounts: balance }
    }

    const fn valid_acc(&self, account: i32) -> bool {
        account > 0 && account <= self.accounts.len() as i32
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.valid_acc(account1) || !self.valid_acc(account2) {
            return false;
        }

        if self.accounts[account1 as usize - 1] < money {
            return false;
        }

        self.accounts[account1 as usize - 1] -= money;
        self.accounts[account2 as usize - 1] += money;

        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.valid_acc(account) {
            return false;
        }
        self.accounts[account as usize - 1] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.valid_acc(account) || self.accounts[account as usize - 1] < money {
            return false;
        }
        self.accounts[account as usize - 1] -= money;
        true
    }
}

pub fn test_bank_system() {
    let mut obj = Bank::new(vec![10, 100, 20, 50, 30]);
    assert!(obj.withdraw(3, 10));
    assert!(obj.transfer(5, 1, 20));
    assert!(obj.deposit(5, 20));
    assert!(!obj.transfer(3, 4, 15));
    assert!(obj.withdraw(10, 50));
}

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    test_bank_system();
}
