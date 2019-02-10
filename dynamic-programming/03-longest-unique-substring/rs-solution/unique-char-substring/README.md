# 03. Longest Substring Without Repeating Characters

## Rust Solution

Here's my rust solution:

```rust
fn length_of_longest_substring(s: String) -> i32 {
    // special case empty string because leetcode wants it to be 1 which is stupid
    let mut occurances: HashMap<char, usize> = HashMap::new();
    let mut max = 0;
    let mut count = 0;
    let mut sub_start_idx = 0;
    
    for (i, c) in s.chars().enumerate() {
        
        // if it already exists in the hashmap, get the index it was previously found at
        if let Some(idx) = occurances.get(&c) {
            
            // if that index is within our newest substring bounds, then set max and reset
            // our newest substring start idx to be 1 index higher than the last found occurance
            // update char index with newest and adjust count relative to our position so we can
            // keep counting up without going back.
            if idx >= &sub_start_idx {            
                if count > max {max = count};
                
                sub_start_idx = idx + 1;
                count = (i - sub_start_idx) as i32;
                
                //set latest index for the leter
                occurances.entry(c).and_modify(|e| {*e = i});               
            } else {  
                // regardless we must update the latest idx for this char.
                occurances.insert(c, i);
            }
        } else {
            occurances.insert(c, i);
        }
        count += 1;
    }
    
    // check this in case our string had no repeats to hit the max setter
    if count > max { count } else { max }
}
```

The idea here is that we keep track of the maximum unique substring without
repeating ourselves.  Once we find a character we've seen before, we set our max
(if count is greater than max) and then we set our substring starting index to
be one index higher than our previously found version of the repeating character
we just hit.  We then adjust our count to be the difference between our current
position and our new substring start index position, and update the character we
found before with it's most recent index position (where we are now). This
allows us to keep carrying on and have an accurate count for our new substring
which started one index after the last occurance of the repeating character we
just found.
