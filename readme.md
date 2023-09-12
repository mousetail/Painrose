# Penrose, an esoteric language where code is layed out in aperiodic tile patterns

See https://tilings.math.uni-bielefeld.de/substitution/penrose-rhomb/ and https://en.wikipedia.org/wiki/Penrose_tiling

Each tile has a coordinate like ABBA. From top to bottom the different tile types are labeled A, B, C, D, and E. A, B, and C are on the fat rhomb while D and E are on
the thin romb. B and D are themselves thin, the rest are fat.

The start is at a infinite pattern of C, E, D, B, A, A, ... repeating forever. This ensures the edge should be unreachable. If you reach the edge let me know and a different
pattern will need to be found. Any numbers on the trailing end of the coordinate that match the pattern are ommited. We can't really store infinite coordinates so...

## Builtins

### Control Flow

Note: All directions are relative and will push you in a different direction depending on where you came from.

* `<` Turn Left, angles are relative
* `>` Turn Right
* `^` Turn left if the top of the stack is truthy
* `v` Turn right if the top of the stack is truthy
* `|` Turn around

* `(` less than
* `)` greater than
* `=` Equal

### Stack Manipulation

* `:` Duplicate the top element
* `#` Duplicate the top two elements
* `~` Pop the top element
* `s` Swap the top two elements
* `{}` Rotate the stack left or right
* `d` Pop a number then duplicate the top N elements
* `c` Push a copy Nth element of the stack
* `u` Unwrap an array onto the stack
* `a` Collect the top N elements of the stack into an array

### Strings

* `"` Start/End a string, pushed as seperate character values
* `'` Start/End a string, pushed as a single array
* `\`` Push a single character

### Pushing Constants

* `0123456789` Push a value 0-9
* `π` Push PI
* `ϕ` Push the golden ratio
* `e` Push e

### Math

* `+-*/` Basic arithmetic. On arrays does concat, difference, product, 
* `!` Power
* `sc` Since, Cosine
* `g` Natural Logarithm
* `_` Negate

### Input, Output

#### Input

* `c` Take a character as input
* `l` Take an entire line as input as an array
* `w` Take a word as input
* `n` Parse a word as a number and take that as input

#### Output

* `C` Output a character
* `W` Output an array at once
* `L` Pop N then output the top N characters from the stack, flattens over arrays
* `N` Output a number

### Arrays

* `[` Pop N then push the Nth item of an array
* `]` Pop N and R then set the Nth item of the array to R