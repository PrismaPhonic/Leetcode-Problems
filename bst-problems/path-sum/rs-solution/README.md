## RS Solution

I solved this with the same recursive DFS approach I used for my JS solution.
The tricky thing is that they decided to store nodes as `Option<Rc<RefCell<TreeNode>>>`.  The implementation of using `Option` was obviously smart here as we need some way to know if left or right are `None`.  It seemed overly convoluted to me that they used `RefCell`.  For these problems we aren't re-arranging node relationships at all, so there's no need for interior mutability - and it causes a few potential issues:

1. We might be missing logical issues in our algorithm design that won't be
   checked at compile time - presumably there might be some given input by which
   algorithm could fail (this is true for most programming languages, but it's
   extremely nice to leverage the borrow checker in rust).
2. We encure a cost for checking the borrow rules at runtime.

Regardless that's what they picked, so we just had to use the `borrow` method of
`RefCell` to get an immutable borrow for comparison when digging in.

This solution was **very** fast, at 0ms for a large input and faster than all
other Rust solutions (at time of submission) submitted on leetcode.
