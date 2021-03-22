<template>
  <div :class="classes">
    <i v-if="isKing" class="far fa-star"></i>
  </div>
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
  text-align: center;
}

i {
  font-size: 2.5em;
  margin-top: 0.25em;
}

.white {
  background: white;
  border: 1px solid black;
}

.white > i {
  color: black;
}

.black {
  background: black;
  border: 1px solid white;
}

.black > i {
  color: white;
}
</style>
