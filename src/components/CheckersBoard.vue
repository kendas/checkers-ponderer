<template>
  <div class="board">
    <template v-for="(cols, row) in board" class="column">
      <div class="label" :key="row">{{ rows[row] }}</div>
      <square
        v-for="(square, col) in cols"
        :key="`${row}-${col}`"
        class="square"
        :is-white="square.isWhite"
        :is-selected="isSelected(row, col)"
        :is-highlighted="canMove(row, col) || isPossibleMovement(row, col)"
        @click.native="onSquareClick(row, col)"
      >
        <board-piece
          v-if="pieces.some((p) => p.row === row && p.col === col)"
          :color="pieces.find((p) => p.row === row && p.col === col).color"
          :is-king="pieces.find((p) => p.row === row && p.col === col).isKing"
        />
      </square>
    </template>
    <div class="label" />
    <div v-for="col in cols" v-bind:key="col" class="label">
      {{ col }}
    </div>
  </div>
</template>

<script lang="ts">
import Vue, { PropType } from "vue";

import { Board, Color, MovementType } from "engine";

import Square from "@/components/Square.vue";
import BoardPiece from "@/components/BoardPiece.vue";
import { generateStartingBoard } from "@/utils/board";
import { chunks } from "@/utils/array";
import { BoardSquare, Piece, Position } from "@/interfaces";

interface Move extends Position {
  type: MovementType;
}

interface ComponentData {
  rows: ["8", "7", "6", "5", "4", "3", "2", "1"];
  cols: ["A", "B", "C", "D", "E", "F", "G", "H"];
  board: BoardSquare[][];
  selectedSquare: Position | null;
  turn: Color;
  move_count: number;
}

export default Vue.extend({
  components: { Square, BoardPiece },
  props: {
    gameBoard: { type: Object as PropType<Board>, required: true },
  },
  data(): ComponentData {
    return {
      rows: ["8", "7", "6", "5", "4", "3", "2", "1"],
      cols: ["A", "B", "C", "D", "E", "F", "G", "H"],
      board: generateStartingBoard(),
      selectedSquare: null,
      turn: Color.White,
      move_count: 0,
    };
  },
  computed: {
    pieces(): Piece[] {
      this.move_count; // referencing this forces us to reevaluate when a turn is taken
      const result = [];
      for (const [color, isKing, row, col] of chunks(
        this.gameBoard.all_pieces(),
        4
      )) {
        result.push({ color: color as Color, isKing: !!isKing, row, col });
      }
      return result;
    },
    possibleMoves(): Move[] {
      if (this.selectedSquare !== null) {
        const { row, col } = this.selectedSquare;
        return chunks(this.gameBoard.moves_for(row, col), 3).map(
          ([type, row, col]) => {
            return { type: type as MovementType, row, col };
          }
        );
      }
      return [];
    },
    movablePieces(): Position[] {
      if (this.selectedSquare === null) {
        return chunks(this.gameBoard.get_movable_pieces(this.turn), 4).map(
          ([color, isKing, row, col]) => {
            return { row, col };
          }
        );
      }
      return [];
    },
  },
  methods: {
    onSquareClick(row: number, col: number) {
      if (this.selectedSquare === null) {
        if (this.canMove(row, col)) {
          this.selectedSquare = { row, col };
        }
      } else {
        if (this.isPossibleMovement(row, col)) {
          this.makeMove(row, col);
        } else {
          this.selectedSquare = null;
        }
      }
    },
    isSelected(row: number, col: number): boolean {
      return (
        this.selectedSquare?.row === row && this.selectedSquare?.col === col
      );
    },
    hasPiece(row: number, col: number): boolean {
      return this.pieces.some((p) => p.row === row && p.col === col);
    },
    canMove(row: number, col: number): boolean {
      return this.movablePieces.some((p) => p.row === row && p.col === col);
    },
    isPossibleMovement(row: number, col: number): boolean {
      return this.possibleMoves.some((p) => p.row === row && p.col === col);
    },
    isForcedMovement(row: number, col: number): boolean {
      const move = this.possibleMoves.find(
        (p) => p.row === row && p.col === col
      )!;
      return move.type === MovementType.Forced;
    },
    makeMove(row: number, col: number) {
      const shouldRecheck = this.isForcedMovement(row, col);
      this.gameBoard.make_move(
        this.selectedSquare!.row,
        this.selectedSquare!.col,
        row,
        col
      );
      if (shouldRecheck) {
        this.selectedSquare = { row, col };
        if (this.possibleMoves.every((m) => m.type === MovementType.Free)) {
          this.selectedSquare = null;
          if (this.turn === Color.White) {
            this.turn = Color.Black;
          } else {
            this.turn = Color.White;
          }
        }
      } else {
        this.selectedSquare = null;
        if (this.turn === Color.White) {
          this.turn = Color.Black;
        } else {
          this.turn = Color.White;
        }
      }
      this.move_count += 1;
    },
  },
});
</script>

<style scoped>
.board {
  display: grid;
  grid-template-columns: 4em 4em 4em 4em 4em 4em 4em 4em 4em;
  grid-template-rows: 4em 4em 4em 4em 4em 4em 4em 4em 4em;
  align-content: center;
  vertical-align: middle;
}

.square {
  border: 1px solid black;
  padding-right: 1px;
  padding-bottom: 1px;
}

.label {
  font-size: 1.5em;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
