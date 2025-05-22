tool for setting clock frequency of TT cards.

building:
`cargo build`

this will show a build error, but it works nonetheless.

now you have an executable at `target/debug/tt-set-freq`

usage:
```
# auto detect card and set it to min clock speed
tt-set-freq min
# max clock speed
tt-set-freq max
# given clock speed in MHz. don't worry, there are protections in the firmware that prevent you from doing something stupid.
tt-set-freq 500

# set card with ID 3 to min clock speed
tt-set-freq min 3
```
