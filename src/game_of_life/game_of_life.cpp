#include <getopt.h>

#include <chrono>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>
#include <thread>
#include <vector>

#include "game/board.h"
#include "graphics/screen.h"

struct Position2D {
  std::size_t x = 0;
  std::size_t y = 0;
};

using Position2DVec = std::vector<Position2D>;

static void PrintUsage() noexcept {
  std::cout << "usage: life [OPTION]... INIT_STATE" << std::endl;
  std::cout << "ncurses rendering of Conway's game of life" << std::endl;
  std::cout << "\t-t, --update-rate-ms\tspeed of simulation in milliseconds"
            << std::endl;
  std::cout << "\t-h, --help\t\tprint this help page" << std::endl;
  std::cout << "\tINIT_STATE\t\tcoordinates of the initial live cells"
            << std::endl;
}

static void PrintErrorAndExit(const std::string &err_msg) noexcept {
  std::cerr << "error: " << err_msg;
  std::exit(EXIT_FAILURE);
}

[[nodiscard]] static Position2DVec LoadInitState(const std::string &filename) {
  std::ifstream fhandle(filename);
  if (!fhandle) {
    throw std::invalid_argument("invalid file path ->" + filename);
  }

  std::string line;
  Position2D pos;
  Position2DVec init_state;
  while (std::getline(fhandle, line)) {
    std::sscanf(line.c_str(), "(%zu, %zu)", &pos.x, &pos.y);
    init_state.push_back(pos);
  }
  return init_state;
}

static void InitializeBoard(const Position2DVec &init_state,
                            gol::GameOfLifeBoard &board) {
  auto CenterCoordinate = [](const Position2D &pos, std::size_t height,
                             std::size_t width) -> Position2D {
    return {.x = height / 2 - pos.x, .y = width / 2 - pos.y};
  };

  for (const Position2D &pos : init_state) {
    Position2D cpos = CenterCoordinate(pos, board.Rows(), board.Cols());
    if ((cpos.x >= board.Rows()) || (cpos.y >= board.Cols())) {
      throw std::runtime_error("position does not fit within board boundaries");
    }
    board[cpos.x][cpos.y] = true;
  }
}

static void RunDrawLoop(const gol::graphics::ScreenDimension &dim,
                        int update_rate_ms, gol::GameOfLifeBoard &board) {
  while (!gol::graphics::Quit()) {
    gol::graphics::Clear();
    gol::graphics::DrawBoard(board);
    gol::graphics::DrawInstructions(dim);

    board.Tick();

    std::this_thread::sleep_for(std::chrono::milliseconds(update_rate_ms));
  }
}

int main(int argc, char **argv) {
  try {
    struct option long_options[] = {
        {"update-rate-ms", required_argument, 0, 't'},
        {"help", no_argument, 0, 'h'},
        {0, 0, 0, 0},
    };
    int opt = '\0';
    int long_index = 0;
    int update_rate_ms = 750;
    while (-1 != (opt = getopt_long(argc, argv, "ht:",
                                    static_cast<struct option *>(long_options),
                                    &long_index))) {
      switch (opt) {
        case 't':
          update_rate_ms = std::stod(optarg);
          if (update_rate_ms <= 0) {
            throw std::invalid_argument(
                "update rate must be a positive integer");
          }
          break;
        case 'h':
          PrintUsage();
          std::exit(EXIT_SUCCESS);
        case '?':
          std::exit(EXIT_FAILURE);
      }
    }
    if (!argv[optind]) {
      PrintErrorAndExit("missing initial state configuration file");
    }

    /* ncurses screen initialization */
    gol::graphics::ScreenDimension dim = gol::graphics::InitScreen();

    /* construct the game board, the -1 on the height is intentional to avoid
     * accidentally bumping into the quit message that is displayed at the
     * bottom of the screen */
    gol::GameOfLifeBoard board(dim.height - 1, dim.width);
    InitializeBoard(LoadInitState(argv[optind]), board);

    /* set a reasonable input delay keeping in mind that higher delays make the
     * application seem laggy and that lower delays will waste CPU cycles
     * re-drawing the view */
    const int kInputDelayMs = 100;
    gol::graphics::EnableInputDelay(kInputDelayMs);

    /* repeatedly draw the cells until the user commands exit */
    RunDrawLoop(dim, update_rate_ms, board);

    /* cleanup ncurses resources */
    gol::graphics::DisableInputDelay();
    gol::graphics::TerminateScreen();
  } catch (const std::exception &e) {
    PrintErrorAndExit(e.what());
  }

  std::exit(EXIT_SUCCESS);
}
