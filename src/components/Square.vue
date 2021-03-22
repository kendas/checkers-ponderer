<template>
  <div
    :class="{
      white: isWhite,
      black: !isWhite,
      selected: isSelected,
      highlighted: isHighlighted,
    }"
  >
    <slot />
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: {
    isWhite: { type: Boolean, required: true },
    isSelected: { type: Boolean },
    isHighlighted: { type: Boolean },
  },
  methods: {},
});
</script>

<style scoped>
.white {
  background: #e2d19c;
}

.black {
  background: #331a00;
}

.selected {
  background-color: #00ffff;
}

.highlighted {
  background-color: #00ff88;
}
</style>
