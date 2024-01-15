<template>
  <div class="my-4 grid grid-cols-2 gap-4">
    <div v-for="post in posts" :key="post.id" class="col-span-1 mr-3">
      <PostComponent
        :id="post.id"
        :title="post.title"
        :content="post.content"
        :edited="post.edited"
        :user_id="post.user_id"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import PostComponent from "@/components/Post.vue";
import { Post, getPosts } from "@/api/posts";
import { onBeforeMount, ref } from "vue";

const posts = ref<Post[]>([]);

onBeforeMount(async () => {
  posts.value = await getPosts();
})
</script>
