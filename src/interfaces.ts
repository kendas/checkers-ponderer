import { Color, MovementType } from 'engine';

export interface Piece {
  color: Color;
  isKing: boolean;
  row: number;
  col: number;
}

export interface PiecePosition {
  piece: Piece;
  position: [number, number];
}

export interface BoardSquare {
  isWhite: boolean;
  isHighlighted: boolean;
}

export interface Movement {
  type: MovementType;
  row: number;
  col: number;
}

export interface Position {
  row: number;
  col: number;
}
