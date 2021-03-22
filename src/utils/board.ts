import { BoardSquare } from "@/interfaces";

export const generateStartingBoard = (): BoardSquare[][] => {
  const board: BoardSquare[][] = [];
  for (let row = 0; row < 8; row++) {
    const rowCells: BoardSquare[] = [];
    for (let col = 0; col < 8; col++) {
      const cell = { isWhite: !((row + col) % 2), isHighlighted: false };
      rowCells.push(cell);
    }
    board.push(rowCells);
  }
  return board;
};
