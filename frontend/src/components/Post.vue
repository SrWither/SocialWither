<!-- Post.vue -->
<template>
  <div
    class="mb-4 cursor-pointer transition-transform transform hover:scale-105 hover:shadow-lg h-full"
    @click="showPost"
  >
    <div
      class="bg-zinc-800 hover:bg-zinc-900 p-6 rounded-lg shadow-md h-full flex flex-col justify-between"
    >
      <div>
        <h2 class="text-2xl font-bold text-purple-500">{{ title }}</h2>
        <p class="text-gray-400 mt-2 overflow-ellipsis max-w-full flex-1">
          {{ truncateContent }}
        </p>
      </div>
      <div class="mt-1">
        <div class="flex items-center">
          <span class="text-gray-500">ID: {{ id }}</span>
          <span class="mx-2 text-gray-500">|</span>
          <span class="text-gray-500">User ID: {{ user_id }}</span>
        </div>
        <div class="mt-2">
          <span v-if="edited" class="text-purple-500">Editado</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter()

const props = defineProps<{
  id: string;
  title: string;
  user_id: string;
  content: string;
  edited: boolean;
}>();

const showPost = () => {
  router.push(`/post/${props.id}`)
};

const truncateContent =
  props.content.length > 300
    ? `${props.content.slice(0, 300)}...`
    : props.content;
</script>
