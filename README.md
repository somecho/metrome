# Metrome

Metrome is a click track generator. It uses simple text-based rhythm notation to
generate any kind of rhythmic click track. Metrome currently can be used as a
CLI or as a Rust crate. A web interface is in the plans!

### Why would you use Metrome when you can use a metronome? 

Your rhythmic needs may differ. For most musicians, a simple device that beats
constant pulses may be enough. For a small portion of musicians, a
[polyrhythmic metronome](https://github.com/somecho/polymetro) may serve their
niche rhythmic needs. However, if you find yourself facing pieces with constant
tempo, meter and pulse changes, you might want to have dedicated click track
for practicing the piece. Heck, even pieces with the occasional odd time
signature will mess up practicing with a normal metronome.

## Usage
### CLI 
**System requirements**: you will need Rust to run the Metrome CLI. Follow [this
page](https://www.rust-lang.org/tools/install) to install Rust.

#### Quickstart
For a quick start, you can clone this repo and render the examples. These
instructions work for Mac and Linux users. If you know Powershell, feel free to
contribute a matching script!

```
git clone https://github.com/somecho/metrome
cd metrome
./render_examples.sh
```

After running this, the root of the metrome directory should now contain many
WAV files which you can run directly in the command line with cvlc if you have
VLC player installed. Otherwise, just play the WAV files in your player of
choice.

#### Providing your own score
The Metrome score is just a textfile with no extensions. To provide your own
score to Metrome, simple use the `-p` flag like such: `metrome -p my_score` or
`cargo run -- -p my_score` in the metrome project directory. For more
information on how to write a rhythm score, read the following section.

## The Metrome Score

The score metrome uses to create click track is just a simple text file. Here's
a short example of what it looks like.


**Example:**

```
| q. e q. e    | e e e e h 
| q. e q. e    | e e e e h 
| q q. e e e   | e e q. e e e
| e e q. e e e | e e q. e e e | 
```

### Bars

As you can see, it follows traditional notation in that notes are grouped in
bars (as denoted by the barlines `|` ). However, unlike traditional notation,
you do not need to provide a time signature. Bars serve the purpose of
differentiating strong and weak beats, with the first beat of every bar strong
and the rest weak.

```
| q  q  q |
  ^  ^  ^
  |  |  |
  |  |  Weak
  |  Weak
  Strong
```

> ⚠ every score **must** begin and end with a barline!

> Tip: you can organize your score however you like. Line breaks do not make a
> difference.

### Notes, Durations and Ratios

Notes can be represented using these letters:
- `w` - whole note
- `h` - half note
- `q` - quarter note
- `e` - eighth note
- `s` - sixteenth note
- `t` - thirtysecond note

Alternatively, you can also write notes using ratios:

```
these two bars are the same: | 1/4 1/4 1/4 1/4 | q q q q |
ratios are great for triples: | 1/6 1/6 1/6 1/6 1/6 1/6 | 
```

You can use any combination of this. For example, `| q 1/6 1/6 1/6 h |` is a
valid bar.

### Dotted Notes

Any note, duration or ratio can be extended using dots `.`, just like in
traditional notation. For example, `q.` is as long as `3/8` and `q..` is as
long as `7/16`.

### Tempo
Tempo can be specified like this `q = 140` (read: 140 quarter notes per
minute). The grammar for this is `<note> = <number>`, where `<note>` is either
a duration represented as a letter or a ratio.

The tempo can be specified anywhere within a score:
```
At the beginning like a normal score
q = 140
| q q q q |

After a bar 
| q q q q | h = 90 h h |

In the middle of a bar like a psycho
| q q q=150 1/6 1/6 1/6 |
```

> Note: When no tempo is specified, the Metrome defaults to `q = 120` or 120
> quarter notes per minute. Pretty sane if you ask me.

### Relative Tempo Changes
In many music, tempo changes can be done relatively. For example, ♩=♩. ,
meaning dotted quarter notes are now as long as quarter notes. You will
commonly find this in music that switched from 4/4 (simple time) to 6/8
(compound time), where the pulse division turns from two to three stays the
same length.

In Metrome, the equivalent is `q=q.`. Simple, clean. You can do all sorts of
wild things. You can also do this `1/6=q`, where the quarter notes are now as
fast as the triplet quarter notes (essentially the same thing, but semantically
and musically different.

## Examples

Check out the [examples](./examples/valid) for reference.

## Roadmap 
- [ ] take `stdin` input
- [ ] floating point tempo 
- [ ] web interface
- [ ] `-o` flag for specifying custom output path

---

© 2024 Somē Cho
