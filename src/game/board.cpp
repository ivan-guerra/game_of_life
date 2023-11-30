#include "game/board.h"

#include <stdexcept>
#include <string>
#include <vector>

namespace gol {

int GameOfLifeBoard::CountLiveNeighbors(std::size_t row,
                                        std::size_t col) const noexcept {
  using Offset = std::pair<int, int>;
  static const std::vector<Offset> kDirections = {
      {0, 1}, {1, 0}, {0, -1}, {-1, 0}, {1, 1}, {1, -1}, {-1, 1}, {-1, -1},
  };

  const int kRowLimit = Rows();
  const int kColLimit = Cols();
  int neighbor_row = 0;
  int neighbor_col = 0;
  int num_live_neighbors = 0;
  for (const auto& direction : kDirections) {
    neighbor_row = row + direction.first;
    neighbor_col = col + direction.second;
    if ((neighbor_row >= 0) && (neighbor_row < kRowLimit) &&
        (neighbor_col >= 0) && (neighbor_col < kColLimit) &&
        state_[neighbor_row][neighbor_col]) {
      num_live_neighbors++;
    }
  }
  return num_live_neighbors;
}

GameOfLifeBoard::GameOfLifeBoard(std::size_t num_rows, std::size_t num_cols)
    : state_(num_rows, CellStateVec(num_cols, false)) {}

void GameOfLifeBoard::Tick() noexcept {
  CellStateMatrix tmp = state_;
  int num_live_neighbors = 0;
  for (std::size_t i = 0; i < Rows(); ++i) {
    for (std::size_t j = 0; j < Cols(); ++j) {
      num_live_neighbors = CountLiveNeighbors(i, j);
      if (state_[i][j]) {
        if (num_live_neighbors < 2) {
          /* death by underpopulation */
          tmp[i][j] = false;
        } else if (num_live_neighbors > 3) {
          /* death by overpopulation */
          tmp[i][j] = false;
        }
      } else if (num_live_neighbors == 3) {
        /* life by reproduction */
        tmp[i][j] = true;
      }
    }
  }
  state_ = std::move(tmp);
}

}  // namespace gol
