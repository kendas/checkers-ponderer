export enum PlayerColor {
  white,
  black,
}

export interface Piece {
  color: PlayerColor;
  isKing: boolean;
}
