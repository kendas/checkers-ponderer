<template>
  <div :class="classes" />
</template>

<script lang="ts">
import Vue, { PropType } from "vue";

import { Color } from "engine";

export default Vue.extend({
  props: {
    color: { type: Number as PropType<Color>, required: true },
    isKing: { type: Boolean, required: true },
  },
  computed: {
    classes(): { king: boolean; white: boolean; black: boolean } {
      return {
        king: this.isKing,
        white: this.color === Color.White,
        black: this.color === Color.Black,
      };
    },
  },
});
</script>

<style scoped>
div {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  display: inline-block;
  margin-right: 1px;
  margin-bottom: 1px;
}

.white {
  background: white;
  border: 1px solid black;
}

.black {
  background: black;
  border: 1px solid white;
}
</style>
