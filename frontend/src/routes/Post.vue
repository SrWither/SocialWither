<template>
  <div v-if="post" class="bg-zinc-800 text-white p-8 rounded-md m-4">
    <div class="flex items-center mb-4">
      <img
        :src="`${imageApi}/${postProfile?.avatar}`"
        alt="User Avatar"
        class="rounded-full w-8 h-8 mr-2"
      />
      <p>{{ postProfile?.username }}</p>
    </div>
    <h1 class="text-3xl font-bold mb-2">{{ post.title }}</h1>
    <Markdown
      :markdownContent="post.content"
      class="text-lg"
      @clickImage="handleImageClick"
    />

    <transition name="fade">
      <div v-if="showLightbox" class="lightbox">
        <img
          :src="lightboxImageUrl"
          alt="Lightbox Image"
          class="max-w-full max-h-full"
          @click="closeLightbox"
        />
      </div>
    </transition>

    <p v-if="post.edited" class="text-purple-300 mt-2">Editado</p>

    <div v-if="profileStore.profile?.user_id === post.user_id">
      <div class="mt-4 flex">
        <button
          @click="editPost"
          class="bg-indigo-600 text-white px-4 py-2 rounded mr-4"
        >
          Editar
        </button>
        <button
          @click="confirmDelete"
          class="bg-red-600 text-white px-4 py-2 rounded"
        >
          Eliminar
        </button>
      </div>

      <div v-if="showDeleteConfirmation" class="mt-4">
        <p class="mb-4">¿Estás seguro de que deseas eliminar este post?</p>
        <button
          @click="handleDelete"
          class="bg-red-600 text-white px-4 py-2 rounded mr-2"
        >
          Confirmar Eliminación
        </button>
        <button
          @click="cancelDelete"
          class="bg-gray-600 text-white px-4 py-2 rounded"
        >
          Cancelar
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Post, getPostById, deletePost } from "@/api/posts";
import { Profile, getProfileById } from "@/api/profiles";
import { ProfileStore } from "@/states/profile";
import { ref, onBeforeMount } from "vue";
import { useRoute, useRouter } from "vue-router";
import useToasterStore from "@/states/toast";
import { AuthStore } from "@/states/auth";
import Markdown from "@/components/Markdown.vue";

const toasterStore = useToasterStore();
const post = ref<Post | null>(null);
const postProfile = ref<Profile | null>(null);
const imageApi = `${import.meta.env.VITE_APIURL}/images`;
const showDeleteConfirmation = ref(false);
const showLightbox = ref(false);
const lightboxImageUrl = ref("");
const profileStore = ProfileStore();
const authStore = AuthStore();
const router = useRouter();

onBeforeMount(async () => {
  const route = useRoute();
  const postId = Array.isArray(route.params.id)
    ? route.params.id[0]
    : route.params.id;
  const postData = await getPostById(postId);
  post.value = postData;
  if (postData) {
    postProfile.value = await getProfileById(postData.user_id);
  }
});

const handleImageClick = (imageUrl: string) => {
  lightboxImageUrl.value = imageUrl;
  showLightbox.value = true;
};

const closeLightbox = () => {
  showLightbox.value = false;
};

const editPost = () => {
  router.push(`/edit-post/${post.value?.id}`);
};

const confirmDelete = () => {
  showDeleteConfirmation.value = true;
};

const handleDelete = async () => {
  if (post.value) {
    try {
      await deletePost(post.value.id, authStore.token);
      toasterStore.success({ text: "Publicación eliminada exitosamente" });
      router.push("/");
    } catch (error) {
      toasterStore.error({ text: "Error al eliminar la publicación" });
      console.error("Failed to delete post:", error);
    }
  }
};

const cancelDelete = () => {
  showDeleteConfirmation.value = false;
};
</script>

<style>
#content img:not(.emoji) {
  max-width: 800px;
  max-height: 600px;
  height: auto;
  border-radius: 6px;
}

.emoji {
  height: 1.6em;
  width: 1.6em;
  display: inline;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
}
.fade-enter-to, .fade-leave-from {
  opacity: 1;
}

.lightbox {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
}

.lightbox img {
  max-width: 90vw;
  max-height: 90vh;
}
</style>

