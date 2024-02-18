<script setup>
</script>

<template>
    <div class="page-layout">
        <FeedingStatus />
        <div class="bottom-sheet">
            <BottomSheet />
        </div>
    </div>
</template>

<style lang="scss">
.page-layout {
    height: 100vh;
    display: flex;
    flex-direction: column;
    padding: 24px;
}

.feeding-status-container {
    margin-block-end: 24px;
}

.bottom-sheet {
    width: 100%;
    margin-block-start: auto;
}
</style>