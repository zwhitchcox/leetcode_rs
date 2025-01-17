struct Solution;

impl Solution {
    fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let n = costs.len();
        let mut diffs: Vec<i32> = costs.iter().map(|v| v[0] - v[1]).collect();
        diffs.sort_unstable();
        let sum_of_b = costs.iter().fold(0, |sum, v| sum + v[1]);
        let sum_of_diff: i32 = diffs.iter().take(n / 2).sum();
        sum_of_b + sum_of_diff
    }
}

#[test]
fn test() {
    let costs: Vec<Vec<i32>> = [[10, 20], [30, 200], [400, 50], [30, 20]]
        .iter()
        .map(|v| v.to_vec())
        .collect();
    assert_eq!(Solution::two_city_sched_cost(costs), 110);
}
