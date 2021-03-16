<template>
  <div :class="{ white: isWhite, black: !isWhite, selected: isSelected }">
    <slot />
  </div>
</template>

<script lang="ts">
import Vue, { PropType } from "vue";

export default Vue.extend({
  props: {
    isWhite: { type: Boolean, required: true },
    isSelected: { type: Boolean },
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
</style>
