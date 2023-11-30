#ifndef SCREEN_H_
#define SCREEN_H_

#include "game/board.h"

namespace gol {
namespace graphics {

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
 * \brief Render a sprite per live cell on the \p board.
 * \param [in] board 2D Game of Life Board containing current game state.
 */
void DrawBoard(const game::GameOfLifeBoard& board) noexcept;

/**
 * \brief Print a help message on screen.
 * \param [in] screen_dim Screen dimensions.
 */
void DrawInstructions(const ScreenDimension& screen_dim) noexcept;

/**
 * \brief Return \c true if the user has chosen to quit.
 * \details Quitting in this case means the user pressed the 'q' key with the
 *          ncurses window in focus.
 */
[[nodiscard]] bool Quit() noexcept;

}  // namespace graphics
}  // namespace gol

#endif
