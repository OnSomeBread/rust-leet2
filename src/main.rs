#![allow(clippy::needless_pass_by_value)]

use std::{cell::RefCell, rc::Rc};

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

pub const fn min_operations(n: i32) -> i32 {
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

pub fn add_binary(a: String, b: String) -> String {
    let n1 = u128::from_str_radix(&a, 2).unwrap();
    let n2 = u128::from_str_radix(&b, 2).unwrap();
    format!("{:b}", n1 + n2)
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
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
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
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
    List(Vec<NestedInteger>),
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

fn main() {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt().with_writer(non_blocking).init();

    info!(
        "{:?}",
        box_stacking_max_height(vecvec![[50, 45, 20], [95, 37, 53], [45, 23, 12]])
    );
}
