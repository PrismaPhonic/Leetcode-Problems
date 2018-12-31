# 11. Container With Most Water

# JS Solution
Same approach as Rust solution. Solved this using a two pointer approach. Since the two dimensional box that holds our water is constrained by the smaller of the two heights (elements in the array represent height) we can simply store the area in a battle for the max, and then move the pointer in that currently occupies the smaller of the two heights.  We do this until our pointers meet one another and then return max. 


```javascript
/**
 * @param {number[]} height
 * @return {number}
 */
var maxArea = function(height) {
    let l = 0, max = 0, r = height.length - 1;
    
    while (l < r) {
        let left = height[l];
        let right = height[r];
        let area;
        
        if (left < right) {
            area = left * (r-l);
            l++;
        } else {
            area = right * (r-l);
            r--;
        }
        
        if (area > max) max = area;
    }
    
    return max;
};
```
