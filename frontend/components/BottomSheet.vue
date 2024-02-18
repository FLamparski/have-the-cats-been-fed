<script setup lang="ts">
const { data: deviceId, isLoading: isCheckingDeviceId } = useDeviceId();
const { mutate } = useFeedMutation();

function onFeedingConfirmed() {
    mutate();
}
</script>

<template>
    <ClientOnly>
        <template v-if="isCheckingDeviceId">Please wait...</template>
        <DeviceIdForm v-else-if="!deviceId" />
        <FeedButton v-else @feedingConfirmed="onFeedingConfirmed" />
    </ClientOnly>
</template>
