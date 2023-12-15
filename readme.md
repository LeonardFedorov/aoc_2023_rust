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

## Day 8
Part 1 was fine. My idea here was to speed up array access and path tracing by converting each 3 letter address into an integer as if it were base 26. While perfectly fine in principle, this introduced a step of indirection to reading the values out due step-through that made debugging part 2 considerably harder. Maybe the lesson is to not over-optimise in the first pass?
I was not a fan of Part 2. Essentially, any reasonable solution here had to make many implicit assumptions about the nature of the data set beyond the general statement of the problem presented on the website. I think I was able to get away only with the assumption that there was only one Z value in each cycle and then ran the simulation brute force from there. This would scale incredibly poorly if there were more cycles to track in parallel, but it works well enough at this scale. Interestingly, in debug build this takes 160 seconds to execute, but just over 3 seconds in release build so there's some serious optimisation going on there. Something to look into when I am less tilted at this puzzle...

## Day 9
Very straight forward - just initialised a square 2d array of size line length and filled with the differences going down. No real trick or gotcha here. Arguably part 2 was even easier than part 1.

## Day 10
This was a challenging one. Part 1 wasn't too bad once I had sorteds out the data structure, but part 2 required serious thought. The approach I took was to read from left to right across the array, changing from "outside" to "inside" every time the pipe crossed over the line being scanned. The nuance here was to realise that if the pipe came into the scanned row, went horizontally along it, and then left the scanned row in the same direction it entered, the inside/outside state wouldn't flip.

## Day 11
Another easy day. Didn't fall into the trap that part 1 was laying (that of reconstructing the expanded cosmos) so part 2 was an easy addition. Played around with generic types for function signatures for the first time here, which was a nice addition (although strictly speaking unnecessary).

## Day 13
Easy in principle, but proved quite fiddly to get all the indexes working properly. Maybe an issue with a long day at work beforehand... Tried out using loop labels to do multi-level breaks and continues. Not 100% sure it was a good idea as it's moving toward a goto, but certainly saved a lot of space with incremental break and skip logic. Part 2 very straight forward to graft on in my construct - the approach was to count the number of mismatched cells occuring during the loop and count for p2 if there was exactly one error. The loop breaks then only occur when a given test scenario can't be valid for either part.

## Day 14
Not much to say about this one. I had a little bit of trepidation about storing full copies of the maps for the cycle analysis, but in the event the cycle came fairly quickly so this wasn't an issue. Intuitively, it seems like the rocks ought to clump up over time so the problem should generally tend to a steady state fairly quickly (barring a degenerative edge case), but I didn't prove this.

## Day 15
Part 1 very straiht forward - since my preference is to work with strings as byte arrays anyway this very straight forward. Part 2 took a bit of thinking on data structure. I settled on using a single vector for all the boxes initalised with empty vectors to hold a tuple (str, int) for the label and focus lengths. To save time, at the expense of some memory, I handled lens deletion by just zeroing out its entry. When the part 2 scoring occurs, the loop keeps an explicit count of how many initalised lenses are passed.