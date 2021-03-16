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
        @click.native="onSquareClick(row, col)"
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

import Square from "@/components/Square.vue";
import BoardPiece from "@/components/BoardPiece.vue";
import { generateStartingBoard } from "@/utils/board";
import { BoardSquare } from "@/interfaces";

interface ComponentData {
  rows: ["8", "7", "6", "5", "4", "3", "2", "1"];
  cols: ["A", "B", "C", "D", "E", "F", "G", "H"];
  board: BoardSquare[][];
  selectedSquare: [number, number] | null;
}

export default Vue.extend({
  components: { Square, BoardPiece },
  data(): ComponentData {
    return {
      rows: ["8", "7", "6", "5", "4", "3", "2", "1"],
      cols: ["A", "B", "C", "D", "E", "F", "G", "H"],
      board: generateStartingBoard(),
      selectedSquare: null,
    };
  },
  computed: {},
  methods: {
    onSquareClick(row: number, col: number) {
      if (
        this.board[row][col].piece !== undefined &&
        !this.isSelected(row, col)
      ) {
        this.selectedSquare = [row, col];
      } else {
        this.selectedSquare = null;
      }
    },
    isSelected(row: number, col: number): boolean {
      return (
        this.selectedSquare?.[0] === row && this.selectedSquare?.[1] == col
      );
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
