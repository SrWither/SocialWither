<template>
  <div class="bg-purple-800 text-white p-8 rounded-md m-4">
    <h1 class="text-4xl font-bold mb-4">Crear nueva publicación</h1>

    <form @submit.prevent="handlePost">
      <div class="mb-4">
        <label for="title" class="block text-white">Titulo:</label>
        <input
          type="text"
          id="title"
          v-model="newPost.title"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
        <div
          v-if="!$v.title?.$pending && !$v.title.required"
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
              v-model="newPost.content"
              rows="5"
              class="w-full h-5/6 px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
            ></textarea>
            <div
              v-if="!$v.content?.$pending && !$v.content.required"
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

      <button
        type="submit"
        class="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700"
        :disabled="$v.$pending"
      >
        Crear
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Picker, EmojiIndex } from "emoji-mart-vue-fast/src";
import "emoji-mart-vue-fast/css/emoji-mart.css";
import data from "emoji-mart-vue-fast/data/twitter.json";
import { createPost } from "@/api/posts";
import { AuthStore } from "@/states/auth";
import { useRouter } from "vue-router";
import useToasterStore from "@/states/toast";
import { useVuelidate } from "@vuelidate/core";
import { required } from "@vuelidate/validators";
import { uploadImage } from "@/api/profiles";

let emojiIndex = new EmojiIndex(data);

const toasterStore = useToasterStore();
const authStore = AuthStore();
const token = authStore.token;
const imageApi = `${import.meta.env.VITE_APIURL}/images`;

const fileInput = ref<HTMLInputElement | null>(null);

const newPost = ref({
  title: "",
  content: "",
});

const validations = {
  title: { required },
  content: { required },
};

const $v = useVuelidate(validations, newPost);

const router = useRouter();

const showEmoji = (emoji) => {
  newPost.value.content += emoji.native;
};

const handleUploadImage = async (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];

  if(!token) {
    return;
  }

  if (file) {
    const image = await uploadImage(file, token);
    const mdImage = `![image](${imageApi}/${image})`
    newPost.value.content += mdImage
  }
};

const handlePost = async () => {
  if ($v.value.$pending) return;

  if ($v.value.$invalid) {
    toasterStore.error({ text: "Please fill in all required fields" });
    return;
  }

  const { title, content } = newPost.value;

  try {
    const response = await createPost(title, content, token);

    if (response) {
      console.log("Post created successfully:", response);

      newPost.value.title = "";
      newPost.value.content = "";

      toasterStore.success({ text: "Post created" });
      router.push(`/post/${response.id}`);
    } else {
      toasterStore.error({ text: "Error creating post" });
      console.error("Failed to create post.");
    }
  } catch (error) {
    console.error("Error creating post:", error);
  }
};
</script>

<style>
.row {
  display: flex;
}
.row > * {
  margin: auto;
}
</style>
