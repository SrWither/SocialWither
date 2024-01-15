<template>
  <div class="flex flex-1 overflow-hidden m-20 items-center justify-center">
    <form
      @submit.prevent="handleCreateProfile"
      class="bg-purple-800 p-10 rounded-lg shadow-lg w-full max-w-md"
    >
      <h2 class="text-3xl font-semibold text-white mb-8">Creación de perfil</h2>

      <div class="mb-6 flex items-center justify-center">
        <div
          v-if="avatarPreview"
          class="rounded-full overflow-hidden w-32 h-32 mb-4"
        >
          <img
            :src="avatarPreview"
            alt="Avatar Preview"
            class="w-full h-full object-cover rounded-full"
          />
        </div>
      </div>

      <div class="mb-6">
        <label for="profile-image" class="text-white">Foto de perfil</label>
        <input
          type="file"
          ref="avatarInput"
          accept="image/*"
          @change="handleAvatarChange"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
      </div>

      <div class="mb-6">
        <label for="username" class="text-white">Nombre de usuario</label>
        <input
          type="text"
          id="username"
          v-model="username"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
      </div>

      <div class="mb-8">
        <label for="bio" class="text-white">Descripción</label>
        <input
          type="text"
          id="bio"
          v-model="bio"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
      </div>

      <button
        type="submit"
        class="w-full bg-purple-600 text-white p-3 rounded-md hover:bg-purple-700 focus:outline-none focus:ring focus:border-purple-500"
      >
        Crear Perfil
      </button>
    </form>
  </div>
</template>

<script setup lang="ts">
import { createProfile } from "@/api/profiles";
import { AuthStore } from "@/states/auth";
import { ref, Ref } from "vue";
import { useRouter } from "vue-router";
import useToasterStore from "@/states/toast";

const toasterStore = useToasterStore();

const router = useRouter();
const username = ref<string>("");
const bio = ref<string>("");
const authStore = AuthStore();
const avatarInput: Ref<HTMLInputElement | null> = ref(null);
const avatarPreview: Ref<string | null> = ref(null);

const handleAvatarChange = () => {
  const file = avatarInput.value?.files?.[0];

  if (file) {
    const reader = new FileReader();

    reader.onload = (e) => {
      avatarPreview.value = e.target?.result as string;
    };

    reader.readAsDataURL(file);
  }
};

const handleCreateProfile = async () => {
  const avatarFile = avatarInput.value?.files?.[0];

  let token = authStore.token;
  if (!token) {
    toasterStore.error({ text: "Error de autenticación" });
    return;
  }

  if (avatarFile) {
    try {
      let profile = await createProfile(
        username.value,
        avatarFile,
        bio.value,
        token
      );
      if (profile) {
        toasterStore.success({ text: "Perfil creado correctamente" });
        router.push("/");
        location.reload()
      }
    } catch (e) {
      toasterStore.error({ text: "Error al crear el perfil" });
      console.error(e);
    }
  }
};

</script>
