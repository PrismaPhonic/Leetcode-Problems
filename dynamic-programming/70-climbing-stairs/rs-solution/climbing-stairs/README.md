# 70. Climbing Stairs

## Rust Solution

Here's my rust solution:

```rust
pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 { return 1 };

    let mut ways_each = vec![0, 1, 2];

    for i in 3..=n as usize {
        let result = ways_each[i-1] + ways_each[i-2];
        ways_each.push(result);
    }

    ways_each[n as usize]
}
```

Essentially for any stair the possible combinations to get to that stair will be
a summation of all the possibilites for the two stairs that came before it. So
for stair 3 it will be the total sum of possible combinations to stair 1 and to
stair 2 (this is due to the limit of 1 or 2 stair jumps). We can then store our
results as we go summing the previous two - we just have to first pre-fill steps
1 and 2 and start on step 3. At the end we return the last index item.
