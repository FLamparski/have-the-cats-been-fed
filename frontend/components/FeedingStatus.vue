<script setup>
import dayjs from 'dayjs';
import { useFeedingLogQuery } from '~/composables/feedingService';

const { isLoading, data: logs } = useFeedingLogQuery();
const lastTs = computed(() => logs.value ? logs.value[0].ts : null);
const tsDayJs = computed(() => lastTs.value ? dayjs(lastTs.value, "YYYY-MM-DD HH:mm:ss.S Z") : null);
const timeAgo = computed(() => tsDayJs.value ? tsDayJs.value.toNow(true) : null);
</script>

<template>
    <template v-if="isLoading">Please wait...</template>
    <div v-else class="feeding-status-container">
        <div class="feeding-heading">
            <h1 class="last-fed-heading">LAST FED:</h1>
            <h1>{{ timeAgo }} ago</h1>
        </div>
        <div v-if="lastTs" class="last-timestamp">
            <h2>({{ lastTs }})</h2>
        </div>
    </div>
</template>

<style lang="scss">
h1, h2 {
    margin: 6px 12px;
}

.feeding-status-container {
    text-align: center;

    .last-fed-heading {
        font-size: 24px;
    }

    // Landscape
    @media screen and (min-aspect-ratio: 4/3) {
        .feeding-heading {
            display: flex;
            flex-direction: row;
            align-items: baseline;
            justify-content: center;
        }

        .last-fed-heading {
            margin-inline-end: 9px;
        }
    }

    // Portrait
    @media screen and (max-aspect-ratio: 4/3) {
        .last-timestamp {
            font-size: 12px;
        }
    }
}
</style>