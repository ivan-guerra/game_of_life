#include <chrono>
#include <cstdlib>
#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>
#include <thread>

#include "game/board.h"
#include "graphics/screen.h"

static void RunDrawLoop(const gol::graphics::ScreenDimension &dim,
                        gol::GameOfLifeBoard &board) {
  while (!gol::graphics::Quit()) {
    gol::graphics::Clear();
    gol::graphics::DrawBoard(board);
    gol::graphics::DrawInstructions(dim);

    board.Tick();

    /* TODO: make the tick delay a cmdline arg */
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
  }
}

int main() {
  try {
    /* ncurses screen initialization */
    gol::graphics::ScreenDimension dim = gol::graphics::InitScreen();

    /* construct the game board, the -1 on the height is intentional to avoid
     * accidentally bumping into the quit message that is displayed at the
     * bottom of the screen */
    gol::GameOfLifeBoard board(dim.height - 1, dim.width);

    /* TODO: make the initial configuration a cmdline arg */
    using Position2D = std::pair<int, int>;
    const std::vector<Position2D> kInitPositions = {
        {4, 4}, {4, 5}, {4, 6}, {5, 3}, {5, 4}, {5, 5},
    };
    for (const Position2D &pos : kInitPositions) {
      board[pos.first][pos.second] = true;
    }

    /* set a reasonable input delay keeping in mind that higher delays make the
     * application seem laggy and that lower delays will waste CPU cycles
     * re-drawing the view */
    const int kInputDelayMs = 100;
    gol::graphics::EnableInputDelay(kInputDelayMs);

    /* repeatedly draw the cells until the user commands exit */
    RunDrawLoop(dim, board);

    /* cleanup ncurses resources */
    gol::graphics::DisableInputDelay();
    gol::graphics::TerminateScreen();
  } catch (const std::exception &e) {
    std::cerr << "error: " << e.what() << std::endl;
    std::exit(EXIT_FAILURE);
  }

  std::exit(EXIT_SUCCESS);
}
