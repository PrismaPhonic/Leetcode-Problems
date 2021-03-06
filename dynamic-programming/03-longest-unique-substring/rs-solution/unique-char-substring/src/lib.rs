use std::collections::HashMap;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let string = String::from("abcabcbb");
        assert_eq!(length_of_longest_substring(string), 3);
    }

    #[test]
    fn test_1() {
        let string = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(string), 3);
    }
}
