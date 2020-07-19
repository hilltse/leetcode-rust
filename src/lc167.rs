pub fn two_sum_sorted(nums: &[i32], target: i32) -> (usize, usize) {
    let mut i = 0;
    let mut j = nums.len() as i32 - 1;

    while i < j {
        let sum = nums[i as usize] + nums[j as usize];
        match sum.cmp(&target) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j -= 1,
            std::cmp::Ordering::Equal => return (i as usize, j as usize),
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc167() {
        assert_eq!(two_sum_sorted(&[2, 7, 11, 15], 9), (0, 1));
    }
}
