<template>
    <div>
        <video controls :src="props.src">
        </video>
    </div>
</template>

<script setup lang="ts">
const props = defineProps({
    src: {
        type: String,
        required: true,
    }
})
</script>