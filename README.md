# Conway's Game of Life

https://github.com/ivan-guerra/game_of_life/assets/13009513/afda9bc9-252a-439f-bbcf-896512117e88

`life` is a Linux only application that renders a [Conway's Game of Life][1]
simulation in your terminal window. The user can define their own custom initial
state or use one of the many [examples](examples/).

### Building

To build the project locally, you will need the following libraries and tools
installed:

* CMake3.16+
* C++ compiler supporting C++20 features
* [Doxygen][2]
* Ncurses Developer Libs

To build, change directory to `scripts/linux` and run `build.sh`.

After a successful build, you will find the binary installed to
`game_of_life/bin/`.

### Program Usage

Below is the program usage message that can be seen by running `life --help`:

```text
usage: life [OPTION]... INIT_STATE
ncurses rendering of Conway's game of life
	-t, --update-rate-ms	speed of simulation in milliseconds
	-h, --help		print this help page
	INIT_STATE		coordinates of the initial live cells
```

The `INIT_STATE` argument is a path to a text file containing 2D coordinates
that define the initial state of the game board. The dimensions of the game
board are equal to the dimensions (width/height) of the terminal window.
Reference the [example](examples/) initial state configs when creating your own
config.

### Doxygen Docs

This project is documented using Doxygen. Doxygen docs are built automatically
by the Linux build script. Docs can be found under `docs/game_of_life/`.

[1]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#
[2]: https://www.doxygen.nl/
