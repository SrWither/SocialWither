<template>
  <div class="bg-zinc-800 text-white p-8 m-8 rounded-lg shadow-lg">
    <div v-if="profile">
      <h1 class="text-3xl font-bold mb-4">{{ profile.username }}</h1>
      <img
        :src="`${imageApi}/${profile.avatar}`"
        alt="Avatar"
        class="w-24 h-24 rounded-full mb-4"
      />
      <p class="text-sm mb-2">{{ profile.bio }}</p>
    </div>
    <div v-else>
      <p>No se pudo cargar el perfil.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getProfileById, Profile } from "@/api/profiles";
import { ref, onBeforeMount } from "vue";
import { useRoute } from "vue-router";

const profile = ref<Profile | null>(null);
const imageApi = `${import.meta.env.VITE_APIURL}/images`;

onBeforeMount(async () => {
  const route = useRoute();
  const profileId = Array.isArray(route.params.id)
    ? route.params.id[0]
    : route.params.id;
  profile.value = await getProfileById(profileId);
});
</script>
