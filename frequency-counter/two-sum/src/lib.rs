use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_positions: HashMap<i32, usize> = HashMap::new();
    for (idx, num) in nums.into_iter().enumerate() {
        let complement = target - num;
        if let Some(position) = num_positions.get(&complement) {
            return vec!(idx as i32, *position as i32);
        }
        num_positions.insert(num, idx);
    }
    vec!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twosum() {
        assert_eq!(two_sum(vec!(2, 7, 11, 15), 9), vec!(1,0))
    }
}
