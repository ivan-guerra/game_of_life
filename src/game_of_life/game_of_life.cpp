#include <cstdlib>
#include <iostream>
#include <string>

#include "graphics/screen.h"

void RunDrawLoop(const gol::graphics::CellVec &cells,
                 const gol::graphics::ScreenDimension &dim) {
  while (!gol::graphics::Quit()) {
    gol::graphics::Clear();
    gol::graphics::DrawCells(cells);
    gol::graphics::DrawInstructions(dim);
  }
}

int main() {
  /* ncurses screen initialization */
  gol::graphics::ScreenDimension dim = gol::graphics::InitScreen();

  /* set a reasonable input delay keeping in mind that higher delays make the
   * application seem laggy and that lower delays will waste CPU cycles
   * re-drawing the view */
  const int kInputDelayMs = 100;
  gol::graphics::EnableInputDelay(kInputDelayMs);

  /* repeatedly draw the cells until the user commands exit */
  gol::graphics::CellVec cells;
  RunDrawLoop(cells, dim);

  /* cleanup ncurses resources */
  gol::graphics::DisableInputDelay();
  gol::graphics::TerminateScreen();

  std::exit(EXIT_SUCCESS);
}
