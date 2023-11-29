#ifndef SCREEN_H_
#define SCREEN_H_

#include <vector>

#include "game/board.h"

namespace gol {
namespace graphics {

struct Cell;
using CellVec = std::vector<Cell>;

struct Cell {
  int row = 0;
  int col = 0;
};

/**
 * \brief Screen Dimensions
 */
struct ScreenDimension {
  int width = 0;  /**< Screen width */
  int height = 0; /**< Screen height */
};

/**
 * \brief Initialize the ncurses screen.
 * \returns The dimensions of the screen (i.e., terminal window).
 */
[[nodiscard]] ScreenDimension InitScreen() noexcept;

/**
 * \brief Cleanup ncurses window(s).
 */
void TerminateScreen() noexcept;

/**
 * \brief Clear the screen.
 */
void Clear() noexcept;

/**
 * \brief Set the ncurses input delay.
 * \details Setting the delay too high can cause the application to appear laggy
 *          whereas setting the delay too low can waste CPU cycles.
 * \param [in] delay_ms Input delay in milliseconds.
 */
void EnableInputDelay(int delay_ms) noexcept;

/**
 * \brief Clear input delay settings previously set by EnableInputDelay().
 */
void DisableInputDelay() noexcept;

/**
 * \brief Draw the render a sprite per cell in the input list of cells.
 * \param [in] cells 2D screen coordinates of Cell objects to be rendered
 *                   as sprites.
 * \param [in] sprite The character to be displayed on screen representing a
 *                    cell.
 */
void DrawBoard(const GameOfLifeBoard& board) noexcept;

/**
 * \brief Print a help message on screen.
 * \param [in] screen_dim Screen (i.e., terminal) dimensions.
 */
void DrawInstructions(const ScreenDimension& screen_dim) noexcept;

[[nodiscard]] bool Quit() noexcept;

}  // namespace graphics
}  // namespace gol

#endif
