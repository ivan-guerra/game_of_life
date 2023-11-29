#ifndef BOARD_H_
#define BOARD_H_

#include <cstddef>
#include <vector>

namespace gol {

class GameOfLifeBoard {
 public:
  using CellStateVec = std::vector<bool>;

  GameOfLifeBoard() = delete;
  [[nodiscard]] GameOfLifeBoard(std::size_t num_rows, std::size_t num_cols);
  ~GameOfLifeBoard() = default;

  GameOfLifeBoard(const GameOfLifeBoard &) = default;
  GameOfLifeBoard &operator=(const GameOfLifeBoard &) = default;
  GameOfLifeBoard(GameOfLifeBoard &&) = default;
  GameOfLifeBoard &operator=(GameOfLifeBoard &&) = default;

  [[nodiscard]] std::size_t Rows() const noexcept { return state_.size(); }
  [[nodiscard]] std::size_t Cols() const noexcept { return state_[0].size(); }
  [[nodiscard]] const CellStateVec &operator[](int i) const noexcept {
    return state_[i];
  }
  [[nodiscard]] CellStateVec &operator[](int i) noexcept { return state_[i]; }

  void Tick() noexcept;

 private:
  using CellStateMatrix = std::vector<CellStateVec>;

  [[nodiscard]] int CountLiveNeighbors(std::size_t row,
                                       std::size_t col) const noexcept;

  CellStateMatrix state_;
};

}  // namespace gol

#endif
