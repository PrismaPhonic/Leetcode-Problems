use std::collections::HashSet;

type Grid = Vec<Vec<char>>;

type Point = (usize, usize);

pub fn num_islands(grid: Grid) -> i32 {
   let height = grid.len();

    if height == 0 { return 0 };

    let width = grid[0].len();

    let mut seen: HashSet<Point> = HashSet::new();
    let mut stack: Vec<Point> = Vec::new();
    let mut count = 0;

    // dfs closure for use later when triggering dfs from island find
    let mut dfs = |stack: &mut Vec<Point>, seen: &mut HashSet<Point>| {
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let (x, y) = top;

            if x > 0 && grid[y][x-1] == '1' {
                let left = (x-1, y);
                if seen.insert(left) {
                    stack.push(left);
                }
            }
            if x < width-1 && grid[y][x+1] == '1' {
                let right = (x + 1, y);
                if seen.insert(right) {
                    stack.push(right);
                }
            }
            if y > 0 && grid[y-1][x] == '1' {
                let up = (x, y-1);
                if seen.insert(up) {
                    stack.push(up);
                }
            }
            if y < height-1 && grid[y+1][x] == '1' {
                let down = (x, y+1);
                if seen.insert(down) {
                    stack.push(down);
                }
            }
        }
    };

    // If we have already seen the point, continue looping, otherwise
    // (haven't seen it yet), see if it's a 1. if so, we found an island. increment count and
    // trigger depth first search, updating seen with each neighbor '1' until we exhaust search and pop
    // back out
    for y in 0..height {
        for x in 0..width {
            let point = (x, y);
            if !seen.insert(point) {
                continue;
            } else {
                if grid[y][x] == '1' {
                    count += 1;
                    // trigger search
                    stack.push(point);

                    dfs(&mut stack, &mut seen);
                }
            }
        }
    }

    count

    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
