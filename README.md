# Metrum

## Syntax

### Durations
Durations can be specified with letters or ratios

Letters
Using w, h, q, e, s, t for whole, half, quarter, eighth, sixteenth and thirtysecond respectively.

Ratios
Using any combination of integers e.g. 1/4, 3/4, 5/7, 12/3

Additionally, durations can be dotted by specifying the number of dots after
e.g. w. is a dotted whole note
1/4. is a dotted quarter note

### Specifying Tempo
A tempo can be specified by defining what a duration is equal to. For example
`q = 140` means 140 quarter notes in a minute. `3/4 = 60` means 60 dotted half
notes in a minute. If no tempo is specified in the beginning, a default of
`q=120` is used.

### Writing rhythms
Rhythms are contained in bars. When writing Rhythms, it is not necessary to
specify the time signature. Each bar must start and end with the pipe character
`|`, which denotes the barline, and contains any amount of durations. For
example, a bar with four quarter notes looks like this `| q q q q |`. The first
beat of every bar will be given a strong click.

Units (durations and barlines) must be separated by space or new lines.

### Short hands
Repetitive sequences can be shortened with a couple of short hand notations. 

#### Repeating notes
To repeat a note, simply write `x2` (times 2) directly behind it. For example
`qx2` would mean 2 quarter notes. You can repeat a note any number of times.

#### Repeating bars
To repeat a bar, write `%2` in behind the closing barline of the bar that is to
be repeated. For example, `| w 1/6. |%2` means to repeat the whole note and the
dotted triplet quarter note twice.

### Changing tempo
You can change the tempo any where in the score. For example: `| q q
q=180 q q |`. Here, the first two quarter notes have `q=120` as the tempo and
the last two quarter notes have tempo `q=180` as the tempo. Note that there are
only 4 quarter notes in this example

#### Relative tempo 
A new tempo can also be set like this `q = q.`, where the right side is as long
as the left side. In ths previous example, it means that dotted quarters are
now as long as quarters from before the tempo change.
