Solutions for Advent of Code 2023 written in Rust. No particular paradigm or approach is targeted, beyond striving to have solutions sufficiently general to be able to read input data from any user and not be overfitted to my own.

## Day 1
An annoyingly fiddly start compared with prior years. My solution is inefficient in that it is performing a forwards and backwards search for each digit name string and then checking the returned match indexes (if any). Obviously this is doing more work then a direct parse might, but it still only takes 3ms to execute (with debug symbols) so not that bothered.

## Day 2
At first glance, this looked like it might be a pain but the way string splits in Rust convert directly to loopable iterators actually made it fairly easy to work with. A key idea which made this whole thing easier was to associate the colours to index values so they could be referenced numerically. This day's solution also forced me to modify the time code to display the readout in microseconds rather than milliseconds when the time is short...

## Day 3
The classic return of the "search a 2d array looking for things and reasoning about their neighbours". This brings with it the ugly case analysis of handling not reading out of bounds on the array and such like. As such the solution isn't pretty, but it at least works and I think is reasonably efficient. The gears aren't handled during the initial iteration, but their locations are marked a second map with the actual numbers marked on it is built during the part 1 iteration to make calculating part 2 fairly quick.

## Day 4
An easier puzzle than the previous days, although my initial iterator for loop for part 1 wasn't as compatible with part 2 so had to fall back with a indexed for loop over a vector. This was one of the ones where understanding the puzzle took longer than actually building the code to solve it.

## Day 5
Part 2 of this day contained a classic AoC brute force trap, which my initial solution fell right into. This was optimised by implementing a lookahead which assessed the level of "headroom" on each range to determine how many seeds could be skipped for having a known same result. The end result was very fast, running in roughly half a millisecond with debug symbols.

## Day 6
Looked at this on a train and was able to solve by hand on paper using only a regular calculator due to the small size of the input data and the existence of an analytical solution. The PDF of these workings has been included in the repo (with no apologies for terrible handwriting, it was half 7 in the morning...)
Added a code version of the same solution. As expected it runs incredibly fast - 54 μs with debug symbols and 13 μs without.

## Day 7
Today was my first foray into traits for custom types which made this fairly approachable. The main thrust of the solution was to create a struct to hold the parse of a hand, including a list of the cards and the strength type: the evaluation of which was made much simpler by the omission of suits or straights, all that was needed was to count how many each sized tuple were present in the hand. I then implemented the ordering trait on the hand struct which ordered first by hand strength and then resolved ties by doing card by card comparison. I translated the cards themselves into an integer enum from 0 to 12 to facilitate ease of comparison.
The joker twist in part 2 was prima facie quite the the curveball for my approach but I was able to make it work. I altered the parser so that, when part 2 was being done, the joker was considered card 0 and the cards from 2 up to T were then shuffled up one so that the comparison logic in the struct order still worked as-was. Since there were no straights to think about, for hand strength type it sufficed to simply add on the jokers to one of the largest tuples already present, at which point the same hand strength evaluation logic then worked.

## Day 9
Very straight forward - just initialised a square 2d array of size line length and filled with the differences going down. No real trick or gotcha here. Arguably part 2 was even easier than part 1.

## Day 10
This was a challenging one. Part 1 wasn't too bad once I had sorteds out the data structure, but part 2 required serious thought. The approach I took was to read from left to right across the array, changing from "outside" to "inside" every time the pipe crossed over the line being scanned. The nuance here was to realise that if the pipe came into the scanned row, went horizontally along it, and then left the scanned row in the same direction it entered, the inside/outside state wouldn't flip.