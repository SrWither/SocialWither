<template>
  <div :class="sidebarClasses">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
const props = defineProps({
  position: String,
});

const sidebarClasses = {
  "h-screen": true,
  "w-72": true,
  "shadow-lg": true,
  "bg-purple-800": true,
  "text-white": true,
  "p-4": true,
  "ml-auto": props.position === "right",
};
</script>
