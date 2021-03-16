export enum PlayerColor {
  white,
  black,
}

export interface Piece {
  color: PlayerColor;
  isKing: boolean;
}

export interface PiecePosition {
  piece: Piece;
  position: [number, number];
}

export interface BoardSquare {
  isWhite: boolean;
  piece?: Piece;
  isHighlighted: boolean;
}
