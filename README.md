Tool for setting clock frequency of TT cards.

## tested on
- Blackhole
- Grayskull


## building
`cargo build`

this will show a build error, but it works nonetheless.

now you have an executable at `target/debug/tt-set-freq`


## information
The device firmware currently has three clock modes:
- forced manual clock speed. on Blackhole, can be from 200 MHz to 1400 MHz
- "long idle" mode. on Blackhole, currently 800 MHz
- "busy" mode. on Blackhole, currently 1350 MHz.

Tenstorrent's software currently uses the "busy" mode when the software starts,
and switches to the "long idle" mode when it stops.

Because of how the firmware works,
it currently can NOT automatically go from forced manual mode to "long idle" or "busy".

If in forced manual mode, a program first has to reset the clock speed (on Blackhole, currently sets to 800 MHz),
and then it can go into "long idle" or "busy".

This tool automatically goes out of forced manual mode when called with, `min`, `max`, or `reset`


## usage
```
# auto detect card and set it to min clock speed
tt-set-freq min
# max clock speed
tt-set-freq max

# force the given clock speed in MHz.
# don't worry, there are protections in the firmware that prevent you from doing something stupid.
tt-set-freq 500

# set card with ID 3 to min clock speed
tt-set-freq min 3

# un-force clock speed of card with ID 3
tt-set-freq reset 3
```
