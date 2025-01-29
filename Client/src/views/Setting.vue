<script setup>
import {disable, enable, isEnabled} from '@tauri-apps/plugin-autostart';

// 启用 autostart
await enable();
// 检查 enable 状态
console.log(`registered for autostart? ${await isEnabled()}`);
// 禁用 autostart
disable();
</script>

<template>

</template>

<style scoped>

</style>
