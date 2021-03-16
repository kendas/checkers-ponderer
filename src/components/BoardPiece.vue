<template>
  <div :class="classes" />
</template>

<script lang="ts">
import Vue, { PropType } from "vue";

import { PlayerColor } from "@/interfaces";

export default Vue.extend({
  props: {
    color: { type: Number as PropType<PlayerColor>, required: true },
    isKing: { type: Boolean, required: true },
  },
  computed: {
    classes(): { king: boolean; white: boolean; black: boolean } {
      return {
        king: this.isKing,
        white: this.color === PlayerColor.white,
        black: this.color === PlayerColor.black,
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
