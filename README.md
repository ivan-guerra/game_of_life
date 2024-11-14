# Conway's Game of Life

https://github.com/user-attachments/assets/feffca94-4a83-4eb4-9729-ac8268d8db4a

`life` is a command line application that renders a [Conway's Game of Life][1]
simulation in your terminal window. The user can define their own custom initial
state or use one of the many [examples](examples/).

### Program Usage

Below is the program usage message that can be seen by running `life --help`:

```text
A visualization of Conway's Game of Life.

Usage: life [OPTIONS] <INIT_STATE_FILE>

Arguments:
  <INIT_STATE_FILE>  initial game board state

Options:
  -r, --refresh-rate-usec <REFRESH_RATE_USEC>
          delay between iterations in microseconds [default: 100]
  -h, --help
          Print help
  -V, --version
          Print version
```

The `INIT_STATE` argument is a path to a text file containing 2D coordinates
that define the initial state of the game board. The dimensions of the game
board are equal to the dimensions (width/height) of the terminal window.
Reference the [example](examples/) initial state configs when creating your own
config.

[1]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#
