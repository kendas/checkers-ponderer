<template>
  <div class="board">
    <template v-for="(cols, row) in board" class="column">
      <div class="label" :key="row">{{ rows[row] }}</div>
      <square
        v-for="(square, col) in cols"
        :key="`${row}-${col}`"
        class="square"
        :is-white="square.isWhite"
        @click="onSquareClick(row, col)"
      >
        <board-piece
          v-if="square.piece !== undefined"
          :color="square.piece.color"
          :is-king="square.piece.isKing"
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
import Vue from "vue";

import Square from "./Square.vue";
import BoardPiece from "./BoardPiece.vue";
import { Piece, PlayerColor } from "../interfaces";

interface GeneratedSquare {
  isWhite: boolean;
  piece?: Piece;
}

export default Vue.extend({
  components: { Square, BoardPiece },
  data() {
    return {
      rows: ["8", "7", "6", "5", "4", "3", "2", "1"],
      cols: ["A", "B", "C", "D", "E", "F", "G", "H"],
    };
  },
  computed: {
    board(): GeneratedSquare[][] {
      const result: GeneratedSquare[][] = [];
      for (let row = 0; row < 8; row++) {
        const rowCells: GeneratedSquare[] = [];
        for (let col = 0; col < 8; col++) {
          const isWhite = !!((row + col) % 2);
          let piece: Piece | undefined;
          if (!isWhite) {
            if (row < 3) {
              piece = { color: PlayerColor.black, isKing: false };
            } else if (row > 4) {
              piece = { color: PlayerColor.white, isKing: false };
            }
          }
          const cell: GeneratedSquare = { isWhite, piece };
          rowCells.push(cell);
        }
        result.push(rowCells);
      }
      return result;
    },
  },
  methods: {
    onSquareClick(row: number, col: number) {
      console.log(`Clicked: ${this.cols[col]}${this.rows[row]}`);
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
