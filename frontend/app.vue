<script setup>
import dayjs from 'dayjs';
import customParseFormat from 'dayjs/plugin/customParseFormat';
import relativeTime from 'dayjs/plugin/relativeTime';

dayjs.extend(customParseFormat).extend(relativeTime);
</script>

<template>
  <NuxtPage pageKey="static" />
</template>
