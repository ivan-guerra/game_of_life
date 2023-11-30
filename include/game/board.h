#ifndef BOARD_H_
#define BOARD_H_

#include <cstddef>
#include <vector>

namespace gol {
namespace game {

/**
 * \brief A 2D representation of the Game of Life board.
 * \details The GameOfLifeBoard class implements the core game logic. Users of
 *          the class can construct an MxN game board. Cells on the board can
 *          be manually set live/dead. A Tick() method can called to apply the
 *          Game of Life rules to the current board to achieve the next state.
 */
class GameOfLifeBoard {
 public:
  using CellStateVec = std::vector<bool>;

  /**
   * \brief Consruct an MxN game board.
   * \details By default, all cells are marked dead on construction.
   * \param [in] num_rows Number of rows.
   * \param [in] num_cols Number of columns.
   */
  [[nodiscard]] GameOfLifeBoard(std::size_t num_rows, std::size_t num_cols);
  GameOfLifeBoard() = delete;
  ~GameOfLifeBoard() = default;

  GameOfLifeBoard(const GameOfLifeBoard &) = default;
  GameOfLifeBoard &operator=(const GameOfLifeBoard &) = default;
  GameOfLifeBoard(GameOfLifeBoard &&) = default;
  GameOfLifeBoard &operator=(GameOfLifeBoard &&) = default;

  /**
   * \brief Return the number of board rows.
   */
  [[nodiscard]] std::size_t Rows() const noexcept { return state_.size(); }

  /**
   * \brief Return the number of board columns.
   */
  [[nodiscard]] std::size_t Cols() const noexcept { return state_[0].size(); }

  /**
   * \brief Return the CellStateVec corresponding to index \p i.
   * \param [in] i A GameOfLifeBoard row index.
   * \return The #CellStateVec associated with the ith row of the game board.
   */
  [[nodiscard]] const CellStateVec &operator[](int i) const noexcept {
    return state_[i];
  }
  [[nodiscard]] CellStateVec &operator[](int i) noexcept { return state_[i]; }

  /**
   * \brief Apply the Game of Life rules to the current board.
   */
  void Tick() noexcept;

 private:
  using CellStateMatrix = std::vector<CellStateVec>;

  [[nodiscard]] int CountLiveNeighbors(std::size_t row,
                                       std::size_t col) const noexcept;

  CellStateMatrix state_; /**< 2D boolean state matrix. */
};

}  // namespace game
}  // namespace gol

#endif
