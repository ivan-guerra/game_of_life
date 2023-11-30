#include "graphics/screen.h"

#include <curses.h>
#include <ncurses.h>

#include <cstddef>

#include "game/board.h"

namespace gol {
namespace graphics {

ScreenDimension InitScreen() noexcept {
  initscr();
  cbreak();             /* disable line buffering */
  keypad(stdscr, TRUE); /* enable the keypad */
  noecho();             /* disable character echoing */
  curs_set(0);          /* hide the cursor */

  /* fetch the screen dimensions */
  ScreenDimension screen_dim = {.width = 0, .height = 0};
  getmaxyx(stdscr, screen_dim.height, screen_dim.width);

  return screen_dim;
}

void TerminateScreen() noexcept { endwin(); }

void Clear() noexcept { clear(); }

void EnableInputDelay(int delay_ms) noexcept { timeout(delay_ms); }

void DisableInputDelay() noexcept { timeout(-1); }

void DrawBoard(const game::GameOfLifeBoard& board) noexcept {
  for (std::size_t i = 0; i < board.Rows(); ++i) {
    for (std::size_t j = 0; j < board.Cols(); ++j) {
      if (board[i][j]) {
        mvaddch(i, j, ' ' | A_REVERSE);
      }
    }
  }
  refresh();
}

void DrawInstructions(const ScreenDimension& screen_dim) noexcept {
  mvprintw(screen_dim.height - 1, 0, "%s", "press q to quit");
  refresh();
}

bool Quit() noexcept { return ('q' == getch()); }

}  // namespace graphics
}  // namespace gol
