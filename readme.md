# Penrose, an esoteric language where code is layed out in aperiodic tile patterns

## How it works

### What even is a penrose tiling?

The penrose tilings are a family of aperodic infinite tilings. That they are aperiodic means that, while some patterns appear many times, no sequence of them will repeat end to end forever.

The basic shapes used in the tilings can create simple repeating patterns, but penrose tilings puts certain restrictions on which edges can touch which other edges. These are known as "matching rules" and are the main identifying feature of penrose tiles compared to other aperiodic tilings. The specific matching rules do allow
filling an infinite plane but do not allow any patch to repeat end to end forever.

### How decomposition works?

While the matching rules guarentee a way to fill a infinite plane exist, [you can't just lay tiles randomly and expect not get stuck](https://math.stackexchange.com/questions/2548075/penrose-tilings-with-physical-tiles). Additionally, we need a convenient "coordinate system" to allow us to easily refer to a specific tile.

One method to fill the grid is with so called "substitution tiling". We can define a group of tiles such that the group can tile the same way as the smaller shape, assuming the matching rules are followed. We can then combine some number of these groups into larger groups based on the same rules, and so on forever. This allows us to fill a shape of any size.

This is the substition pattern we use:

![A diagram of where on the big shapes each small shape appears](./images/shapeA.svg)

You can combine rhombs A, B, and C to create a shape that behaves like a thick rhomb. 

Note the orientation is important. For example notice that A is flipped upside down. While the shape may look symetric the matching rules don't work if any shape is flipped the other way around. This is nececairy because the matching rules mean only some edges can match.

You may notice the larger shapes are not actually shaped like rhombs themselves. However, this is not actually an issue. As long as the matching rules are followed, the parts that "stick out" of one supershape exactly match the "holes" in the shape on the other side of the edge. 

The entire right half of the thin rhomb seems to be missing but it will always be
filled by a thick rhomb coming in from the south (bottom right) and a thin rhomb from the east (top right), making the final shape symetric.

### Coordinates

We can now use this to define a coordinate system. A coordinate consists of the letter followed by the next bigger tile that contains it. For example, an AEB means "An thick rhomb at the position A relative to a virtual larger thin rhomb at the E position relative to a hypothetical even bigger B tile".

However, we want to be able to represent coordinates on a potentially infinite plane, and having coordinates of infinite length is inconvenient. Thus we define a starting tile whose position will be at the pattern `CEDBAA` repeating forever. So if your coodinate is printed as `A` it will actually be be `AEDBAACEDBAACEDBAAA...`. 

### Traversing the grid

Now all we need is rules to cover how to traverse the grid. Each tile has 4 sides, North, East, South, and West, where north is represented by the left indentation. Now all we need to define for each side of every tile tipe where you would end up if you go that way. Some examples:

* If you go North from tile B, you end up coming in from the north side of the tile in the A position sharing the same parent tile.
* If you go East from tile B, you will leave the parent tile on the east side, specifically on the left half of the right half of that edge.

There are 20 such rules plus 10 more for what happens if you enter one of the large tiles from a specific edge, half edge, or quarter edge.

When leaving the parent tile, repeat the process for the parent's parent, and the parent's parent parent, until you reach an instruction that allows you to enter a tile, at which point you use the matching rule for what happens when entering a tile from a specifc edge until you reach a physical, first level tile again.

In some rare cases entering a tile form a specific angle connects directly back outwards. For example entering the thin rhomb from the south (bottom right) immediatly connects back out to the right half of the top. This rhomb is guarenteed to be filled by a thick rhomb from the tile that connects to the east edge. In this case the algorithm will transition from a inwards motion to an outwards motion again.

Some resources that where incredibly helpful working this out: https://tilings.math.uni-bielefeld.de/substitution/penrose-rhomb/ and https://en.wikipedia.org/wiki/Penrose_tiling



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
* <code>`</code> Push a single character

### Pushing Constants

* `0123456789` Push a value 0-9
* `π` Push PI
* `ϕ` Push the golden ratio
* `e` Push e

### Math

* `+-*/` Basic arithmetic. On arrays does concat, difference, product, 
* `!` Power
* `sc` Sine, Cosine
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