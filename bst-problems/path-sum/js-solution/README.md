## JS Solution

I solved this using a simple recursive DFS appproach.  The solution is very
readable so I won't talk about it in too much depth but notice the logic of the
returns.  We only return true if a child recursion has returned true (this gates
off false returns from a recursive child).  If we get back to the root node that
kicked off the recursion in the first place and we have not gotten back a `true`
from any root-to-leaf path then we return false.  

Had to add a check that the root itself is not null because leetcode decided it
would be a great idea to give an example of some junk input which is
unbelievably stupid to have to special case for, but here we are.  
