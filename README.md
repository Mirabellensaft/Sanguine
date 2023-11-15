# Sanguine

Or red chalk is a mineral pigment made of clay and hematite (Fe<sub>2</sub>O<sub>3</sub>). It is deep red in color and has been used for artistic expression already by the neandertales. 
Since the renaissance it has been shaped into a kind of pencil, used mainly for sketches similar to graphite pencils today.

The goal is to build a framework for generative art for pen plotters in Rust.

Hello, World!

![Hello, World! a 3 by 3 grid of 9 groups of 2 distorted squares, black lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/hello_world.png?raw=true)

## Features I have in mind:

- [ ] generate art as svg
- [ ] generate art without svg output, by adressing the pen plotter directly
- [ ] input from hardware rngs


## Changes, Thoughts and Learnings (newest first)
- November 15th, 2023
    - split code into modules, reduced a lot of duplicate code.
    - wrote abstractions for the functions provided by the svg crate, since colors, line width, the absence of fill etc. can be hard coded, as those things don't matter to a pen plotter.
    - added lines from border from the grid field to the circle center
      
![groups of 2 cirles in a 5 by 5 grid, bleck lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00003.png?raw=true)

- November 14th, 2023
    - changed distored squares to circles in a 5x5 grid
      
![groups of 2 cirles in a 5 by 5 grid, bleck lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00000.png?raw=true)

- November 11th, 2023
    - Hello, World!
      
    ![Hello, World! a 3 by 3 grid of 9 groups of 2 distorted squares, black lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/hello_world.png?raw=true)
  
    - I ran into an ownership problem with instances of the type `std::ops::Range<i32>` when generating that range early and using it in an iteration. The current solution is passing the values of the range and generating the range inside that function that is called in the iteration.
