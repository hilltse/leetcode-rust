pub fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    use std::collections::HashMap;
    let mut seen = HashMap::new();

    for (i, &x) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - x)) {
            return (j, i);
        }
        seen.insert(x, i);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc1() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), (0, 1));
    }
}
