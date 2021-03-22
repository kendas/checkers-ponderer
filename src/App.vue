<template>
  <div id="app">
    <checkers-board :game-board="gameBoard" />
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import CheckersBoard from "./components/CheckersBoard.vue";
import { Board } from 'engine';

export default Vue.extend({
  name: "App",
  components: {
    CheckersBoard
  },
  data() {
    return {
      gameBoard: new Board()
    }
  }
});
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
