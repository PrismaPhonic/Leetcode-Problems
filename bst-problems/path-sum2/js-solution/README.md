## JS Solution

Very similar to pathSum #1 solution except that we don't bother returning
anything from our rescursive function - we simply check when we get to a leaf if
the sums match and if so push to the paths array which was defined outside of
the recursive function - and then return the results
