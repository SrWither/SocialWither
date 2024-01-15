<template>
  <div class="bg-purple-800 text-white p-8 rounded-md m-4">
    <h1 class="text-4xl font-bold mb-4">Editar Publicación</h1>

    <form @submit.prevent="handleEditPost">
      <div class="mb-4">
        <label for="title" class="block text-white">Titulo:</label>
        <input
          type="text"
          id="title"
          v-model="editedPost.title"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
        <div
          v-if="!$v.editedPost.title?.$pending && !$v.editedPost.title.required"
          class="text-red-500"
        >
          Titulo requerido
        </div>
      </div>

      <div class="mb-4">
        <div class="grid grid-cols-12 gap-4">
          <div class="col-span-9">
            <label for="content" class="block text-white">Contenido:</label>
            <textarea
              id="content"
              v-model="editedPost.content"
              rows="5"
              class="w-full h-5/6 px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
            ></textarea>
            <div
              v-if="
                !$v.editedPost.content?.$pending &&
                !$v.editedPost.content.required
              "
              class="text-red-500"
            >
              Contenido requerido
            </div>
          </div>
          <div class="col-span-3">
            <Picker
              ref="emojiPicker"
              :data="emojiIndex"
              set="twitter"
              @select="showEmoji"
            />
          </div>
        </div>
      </div>

      <div class="block my-3">
        <label
          for="imageInput"
          class="bg-orange-400 text-white px-4 py-2 rounded-md cursor-pointer hover:bg-orange-600"
        >
          Agregar Imagen
        </label>
        <input
          id="imageInput"
          type="file"
          ref="fileInput"
          accept="image/*"
          @change="handleUploadImage"
          capture
          class="hidden"
        />
      </div>

      <div class="flex justify-between">
        <button
          type="submit"
          class="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700"
          :disabled="$v.$pending"
        >
          Actualizar
        </button>

        <button
          type="button"
          class="bg-gray-600 text-white px-4 py-2 rounded-md hover:bg-gray-700"
          @click="handleCancel"
        >
          Cancelar
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, onBeforeMount } from "vue";
import { Picker, EmojiIndex } from "emoji-mart-vue-fast/src";
import "emoji-mart-vue-fast/css/emoji-mart.css";
import data from "emoji-mart-vue-fast/data/twitter.json";
import { getPostById, editPost } from "@/api/posts";
import { AuthStore } from "@/states/auth";
import { useRouter, useRoute } from "vue-router";
import useToasterStore from "@/states/toast";
import { ProfileStore } from "@/states/profile";
import { useVuelidate } from "@vuelidate/core";
import { required } from "@vuelidate/validators";
import { uploadImage } from "@/api/profiles";

const toasterStore = useToasterStore();
const authStore = AuthStore();
const profileStore = ProfileStore();
const token = authStore.token;

const imageApi = `${import.meta.env.VITE_APIURL}/images`;

const fileInput = ref<HTMLInputElement | null>(null);

const editedPost = ref({
  title: "",
  content: "",
});

const validations = {
  editedPost: {
    title: { required },
    content: { required },
  },
};

const $v = useVuelidate(validations, { editedPost });

const router = useRouter();
const route = useRoute();

// Emoji Picker
const emojiIndex = new EmojiIndex(data);

const showEmoji = (emoji) => {
  editedPost.value.content += emoji.native;
};

const handleUploadImage = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];

  if (!token) {
    return;
  }

  if (file) {
    const image = await uploadImage(file, token);
    const mdImage = `![image](${imageApi}/${image})`;
    editedPost.value.content += mdImage;
  }
};

const handleEditPost = async () => {
  if ($v.value.$pending) return;

  if ($v.value.$invalid) {
    toasterStore.error({ text: "Rellenar todo el formulario" });
    return;
  }

  const postId = Array.isArray(route.params.id)
    ? route.params.id[0]
    : route.params.id;

  try {
    const existingPost = await getPostById(postId);

    if (existingPost) {
      const { title, content } = editedPost.value;

      if (existingPost.user_id !== profileStore.profile?.user_id) {
        toasterStore.error({
          text: "No tienes permisos para editar este post",
        });
        router.push("/");
        return;
      }

      const response = await editPost(postId, title, content, token);

      if (response) {
        console.log("Post updated successfully:", response);

        toasterStore.success({ text: "Publicación actualizada" });
        router.push(`/post/${response.id}`);
      } else {
        toasterStore.error({ text: "Error al actualizar la publicación" });
        console.error("Failed to update post.");
      }
    } else {
      toasterStore.error({ text: "No se encontró el post para editar" });
      console.error("Post not found for editing.");
    }
  } catch (error) {
    console.error("Error editing post:", error);
  }
};

const handleCancel = () => {
  const postId = Array.isArray(route.params.id)
    ? route.params.id[0]
    : route.params.id;

  router.push(`/post/${postId}`);
};

onBeforeMount(async () => {
  const postId = Array.isArray(route.params.id)
    ? route.params.id[0]
    : route.params.id;
  const existingPost = await getPostById(postId);

  if (existingPost) {
    editedPost.value.title = existingPost.title;
    editedPost.value.content = existingPost.content;

    if (existingPost.user_id !== profileStore.profile?.user_id) {
      toasterStore.error({
        text: "No tienes permisos para editar este post",
      });
      router.push("/");
    }
  }
});
</script>
