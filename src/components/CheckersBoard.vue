<template>
  <div class="board">
    <template v-for="(row, rowIndex) in rows" class="column">
      <div class="label" :key="row">{{ row }}</div>
      <div
        v-for="(col, colIndex) in cols"
        :key="`${row}-${col}`"
        :class="{
          cell: true,
          white: !!((colIndex + rowIndex) % 2),
          black: !((colIndex + rowIndex) % 2),
        }"
      />
    </template>
    <div class="label" />
    <div v-for="col in cols" v-bind:key="col" class="label">
      {{ col }}
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  data() {
    return {
      rows: ["8", "7", "6", "5", "4", "3", "2", "1"],
      cols: ["A", "B", "C", "D", "E", "F", "G", "H"],
    };
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

.cell {
  border: 1px solid black;
}

.label {
  display: flex;
  justify-content: center;
  align-items: center;
}

.white {
  background: white;
}

.black {
  background: black;
}
</style>
