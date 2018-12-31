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
