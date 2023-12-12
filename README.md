# Sanguine

Or red chalk is a mineral pigment made of clay and hematite (Fe<sub>2</sub>O<sub>3</sub>). It is deep red in color and has been used for artistic expression already by the neanderthals. 
Since the renaissance it has been shaped into a kind of pencil, used mainly for sketches similar to graphite pencils today.

The goal is to build a framework for generative art for pen plotters in Rust.

Hello, World!

![Hello, World! a 3 by 3 grid of 9 groups of 2 distorted squares, black lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/hello_world.png?raw=true)

## Features I have in mind:

- [x] generate art as svg
- [ ] parse a limited number of shapes from svg files (WIP).
- [x] add more organic ways to generate images, such as voronoi diagrams
- [ ] adapt composition things to work with voronoi diagrams
- [ ] experiment with L-systems.
- [ ] add camera input, so the generated art can be a reaction to what already is on paper.
- [ ] project layout that allows for easy transitions between creating art work with lots of predefined functions, writing custom parts for the frame work and switching between different works of art.
- [ ] generate art without svg output, by addressing the pen plotter directly
- [ ] input from hardware rngs

## Changes, Thoughts and Learnings (newest first)

### December 12th, 2023

Refactored the way Grid and Voronoi Diagrams are constructed. Now all diagram types are constructed with the same constructor, only with different variants, which in the end decides, which diagram type is first built. I added a trait, so that the diagram types can share behavior regarding compositional aspects.
This way I was able to remove a lot of somewhat similar, somewhat duplicate code, and suddenly I had way less types named Voronoi something.
I slowly get the way, one needs to think code a lot bigger then what you're currently working on. The ability to see future variants of something, future expansions, and functions. Not being able to see that leads to rewriting code a lot.


### December 8th, 2023

Added a characteristic timestamp to the filename of the generated images. The timestamp can later be part of the stamp on the backside of the image. 

![Screenshot showing file names in the format "nr_000_%Y%m%d_%H%M%S.](https://github.com/Mirabellensaft/sanguine/blob/main/images/filename.png)

Added the type that keeps data for the polygons, so that the center may be saved with each cell, to be able to address each cell by the center point. Following this I am also starting to think about how to order the center points in a meaningful way. 

### December 5th, 2023

Off to new experiments! I implemented the ability to generate the cells of a voronoi diagram, and draw the lines.

<figure>
    <img 
        src="https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00011.png"
        width="150"
        height="300"
        alt="Drawn cells of a voronoi diagram">
    <figcaption>Drawn cells of a voronoi diagram.</figcaption>
</figure>

### December 4th, 2023

I have started to work on parsing svg files. While the results for my art work are currently not very usable, the general principle works and it lead to implementing `Shape` as a `trait` and usage of it as a trait object with all the pitfalls of sized types that comes with it. I decided to drop the topic of including vectorization from a camera for now, to pic it up later again, as I figured I don't want to work on a grid based system with this. 

<figure>
    <img 
        src="https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00010.png"
        width="150"
        height="300"
        alt="An experiment with colliding lines with the edges of an ellipse that is in the parsed file">
    <figcaption>An experiment with colliding lines with the edges of an ellipse that is in the parsed file.</figcaption>
</figure>

### December 1st, 2023

[link to a video on instagram showing the finished plots](https://www.instagram.com/p/C0T4Er_KML1/)

You can buy one to support this work!

### November 26th, 2023

- put an order to the generated coordinates for the lines, so they are no completely random. The effort was well spent, the images look a lot cleaner, without looking too ordered, but the many "stray" looking lines are gone.
- added a compositional feature that sets the focus of generation into the bottom third of the image. This feature needs a lot of improvement.
- decided this is good enough for a first series, generated 100, and selected the 10 I liked and started plotting them.



<figure>
    <img 
        src="https://github.com/Mirabellensaft/sanguine/blob/main/images/plot.jpeg"
        width="166"
        height="295"
        alt="Photograph of my plotter doing the work">
    <figcaption>Photograph of my plotter doing the work.</figcaption>
</figure>

### November 24th, 2023

- changed how coordinates are generated to be more versatile in use
- avoid doubles and coordinates that are too close together (this still needs improvement, but its also good enough for now)
- added more graphical elements that are derived from the original star_burst motif and different algorithms that place them. (This turned out to be a key part in the development of the first series)


<figure>
    <img 
        src="https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00009.png"
        width="150"
        height="300"
        alt="Empty space, different compositional elements derived from the star_burst pattern">
    <figcaption>Empty space, different compositional elements derived from the star_burst pattern.</figcaption>
</figure>

### November 22th, 2023

- added compositional layer, that sets random centers of attention

<figure>
    <img 
        src="https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00008.png"
        width="150"
        height="300"
        alt="Few larger circles are surrounded by smaller ones, the rest are the smallest circles.">
    <figcaption>Few larger circles are surrounded by smaller ones, the rest are the smallest circles.</figcaption>
</figure>

### November 20th, 2023

- support for tesselation 
- cleaner collision of lines with circles. Probably good enough for plotting. 
- also, we now get into the territory of panics, calculations that go on indefinetly... so there was `Some()` introduction of Options.

![the single tiles become less dominant when the rays of the cicle each meet a ray from the neighboring fiel on the border.](https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00007.png?raw=true)


### November 17th, 2023

 - added support for some math operations to generate lines and points and not just plot random shapes. This lead to an understanding, that this framework has different layers: a more abtract one that generates the lines to be plottet, and the svg functions that visualize the generated works. Currently it makes sense to me, to have the an `fn draw()` method for every shape that does just that. 
- another oldie but good to remember: on paper algebraic solutions are simpler, in programming, iterative methods are more elegant and flexible. 

![a 5x5 grid of the star burst shape, lines sort of end at the circle. sort of.](https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00005.png?raw=true)

### November 15th, 2023
- split code into modules, reduced a lot of duplicate code.
- wrote abstractions for the functions provided by the svg crate, since colors, line width, the absence of fill etc. can be hard coded, as those things don't matter to a pen plotter.
- split code into modules, reduced a lot of duplicate code.
- wrote abstractions for the functions provided by the svg crate, since colors, line width, the absence of fill etc. can be hard coded, as those things don't matter to a pen plotter. 

- added lines from border from the grid field to the circle center

![a 5x5 grid of random circles with lines going from the field border to the center of the circle. The star burst shape](https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00003.png?raw=true)


### November 14th, 2023
- changed distored squares to circles in a 5x5 grid

![groups of 2 cirles in a 5 by 5 grid, black lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/image_00001.png?raw=true)


### November 11th, 2023

- I ran into an ownership problem with instances of the type `std::ops::Range<i32>` when generating that range early and using it in an iteration. The current solution is passing the values of the range and generating the range inside that function that is called in the iteration.

- Hello, World!
      
![Hello, World! a 3 by 3 grid of 9 groups of 2 distorted squares, black lines on white ground](https://github.com/Mirabellensaft/sanguine/blob/main/images/hello_world.png?raw=true)
  
   