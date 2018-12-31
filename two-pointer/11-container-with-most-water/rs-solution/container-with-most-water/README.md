# 11. Container With Most Water

## Rust Solution

I solved this using a two pointer approach. Since the two dimensional box that
holds our water is constrained by the smaller of the two heights (elements in
the array represent height) we can simply store the area in a battle for the
max, and then move the pointer in that currently occupies the smaller of the two
heights.  We do this until our pointers meet one another and then return max.
Inlining this solution produces a four fold increase in performance.  

```Rust
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
```
