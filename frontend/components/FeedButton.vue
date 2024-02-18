<script setup lang="ts">
import "~/assets/styles/button.scss";

const timeToHold = 1000;
const isButtonHeld = ref(false);
const timeHeld = ref(0);

const progress = computed(() => Math.min(timeHeld.value, timeToHold));

const emit = defineEmits<{
  (e: "feedingConfirmed"): void;
}>();

function holdStart() {
  isButtonHeld.value = true;

  let lastTs = performance.now();
  requestAnimationFrame(function frame(ts) {
    if (isButtonHeld.value) {
      if (timeHeld.value < timeToHold) {
        requestAnimationFrame(frame);
      } else {
        emit("feedingConfirmed");
      }
    }

    const delta = ts - lastTs;
    lastTs = ts;
    timeHeld.value += delta;
  });
}

function holdEnd() {
  isButtonHeld.value = false;
  timeHeld.value = 0;
}
</script>

<template>
  <div class="feed-button-container">
    <button
      type="button"
      class="btn feed-button"
      @mousedown="holdStart"
      @mouseup="holdEnd"
    >
      FEED
    </button>
    <progress
      v-if="isButtonHeld"
      class="feed-progress"
      :max="timeToHold"
      :value="timeHeld"
    />
  </div>
</template>

<style lang="scss">
@use "~/assets/styles/variables/colours.scss" as colours;
@use "~/assets/styles/variables/borders.scss" as borders;

.feed-button-container {
  position: relative;
  width: 100%;
}

.feed-button {
  position: relative;
  font-size: 24px;
  padding: 6px 24px;
  z-index: 100;
  width: 100%;
}

.feed-progress {
  -webkit-appearance: none;
  appearance: none;
  background-color: transparent;
  display: block;
  position: absolute;
  z-index: 0;
  top: 1px;
  left: 1px;
  height: calc(100% - 2px);
  width: calc(100% - 2px);
  border-radius: borders.$borderRadiusMd;

  &::-webkit-progress-value,
  &::-moz-progress-bar {
    background-color: colours.$blue300;
  }
}
</style>
