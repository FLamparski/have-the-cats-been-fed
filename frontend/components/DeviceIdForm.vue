<script setup lang="ts">
import "~/assets/styles/input.scss";
import "~/assets/styles/button.scss";

import { useSetDeviceIdMutation } from "~/composables/deviceIdService";

const deviceId = ref('');
const { mutate } = useSetDeviceIdMutation();

function saveDeviceId() {
    if (deviceId.value) {
        mutate(deviceId.value);
    }
}
</script>

<template>
  <div class="device-id-form">
    <label for="deviceId">Device ID:</label>
    <input id="deviceId" v-model="deviceId" type="text" class="input" required />
    <button type="button" class="btn" @click="saveDeviceId">SAVE</button>
  </div>
</template>

<style lang="scss">
@use "~/assets/styles/variables/colours.scss" as colours;

.device-id-form {
  display: flex;
  align-items: baseline;

  * + * {
    margin-inline-start: 6px;
  }
}

.input:invalid {
    border-color: colours.$red600;
}
</style>
