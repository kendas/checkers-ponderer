import { BoardSquare, PiecePosition, PlayerColor } from "@/interfaces";

const generateStartingPieces = (): PiecePosition[] => {
  const pieces: PiecePosition[] = [];
  for (let row = 0; row < 3; row++) {
    for (let col = row % 2; col < 8; col += 2) {
      pieces.push({
        position: [row, col],
        piece: { color: PlayerColor.black, isKing: false },
      });
    }
  }
  for (let row = 5; row < 8; row++) {
    for (let col = row % 2; col < 8; col += 2) {
      pieces.push({
        position: [row, col],
        piece: { color: PlayerColor.white, isKing: false },
      });
    }
  }
  return pieces;}

export const generateStartingBoard = (): BoardSquare[][] => {
  const board: BoardSquare[][] = [];
  const pieces = generateStartingPieces();
  for (let row = 0; row < 8; row++) {
    const rowCells: BoardSquare[] = [];
    for (let col = 0; col < 8; col++) {
      const isWhite = !!((row + col) % 2);
      const piece = pieces.find(
        (p) => p.position[0] === row && p.position[1] === col
      );
      const cell = { isWhite, piece: piece?.piece, isHighlighted: false };
      rowCells.push(cell);
    }
    board.push(rowCells);
  }
  return board;
};
