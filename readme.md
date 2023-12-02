Solutions for Advent of Code 2023 written in Rust. No particular paradigm or approach is targeted, beyond striving to have solutions sufficiently general to be able to read input data from any user and not be overfitted to my own.

## Day 1
An annoyingly fiddly start compared with preivous years. My solution is inefficient in that it is performing a forwards and backwards search for each digit name string and then checking the returned match indexes (if any). Obviously this is doing more work then a direct parse might, but it still only takes 3ms to execute so not that bothered.

## Day 2
At first glance, this looked like it might be a pain but the way string splits in Rust convert directly to loopable iterators actually made it fairly easy to work with. A key idea which made this whole thing easier was to associate the colours to index values so they could be referenced numerically. This day's solution also forced me to modify the time code to display the readout in microseconds rather than milliseconds when the time is short...