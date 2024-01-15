<template>
  <div class="bg-zinc-800 text-white p-8 m-8 rounded-lg shadow-lg">
    <div v-if="profileStore.profile">
      <h1 class="text-3xl font-bold mb-4">
        {{ profileStore.profile.username }}
      </h1>
      <img
        :src="`${imageApi}/${profileStore.profile.avatar}`"
        alt="Avatar"
        class="w-24 h-24 rounded-full mb-4"
      />
      <p class="text-sm mb-2">{{ profileStore.profile.bio }}</p>

      <div class="flex space-x-4">
        <button
          @click="handleLogout"
          class="bg-red-500 hover:bg-red-700 text-white py-2 px-4 rounded"
        >
          Cerrar sesión
        </button>

        <button
          class="bg-blue-500 hover:bg-blue-700 text-white py-2 px-4 rounded"
        >
          Editar Perfil
        </button>
      </div>
    </div>
    <div v-else>
      <p>No se pudo cargar el perfil.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { AuthStore } from "@/states/auth";
import router from ".";
import useToasterStore from "@/states/toast";
import { ProfileStore } from "@/states/profile";

const toasterStore = useToasterStore();
const profileStore = ProfileStore();
const authStore = AuthStore();
const imageApi = `${import.meta.env.VITE_APIURL}/images`;

const handleLogout = () => {
  authStore.clearToken();
  profileStore.clearProfile();
  toasterStore.warning({ text: "Sesión cerrada" });
  router.push("/login");
};
</script>
