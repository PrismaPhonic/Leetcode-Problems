#[inline]
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len()-1; 
    let mut max = 0;
    
    while l < r {
        let left = height[l];
        let right = height[r];
        
        let area = if left < right {
            l += 1;
            left * (r-l+1) as i32
        } else {
            r -= 1;
            right * (r-l+1) as i32
        };
        if area > max { max = area };
    }
    
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testcase_0() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}
