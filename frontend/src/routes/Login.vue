<template>
  <div class="flex flex-1 overflow-hidden m-40 items-center justify-center">
    <form
      @submit.prevent="handleLogin"
      class="bg-purple-800 p-10 rounded-lg shadow-lg w-full max-w-md"
    >
      <h2 class="text-3xl font-semibold text-white mb-8">Iniciar Sesión</h2>

      <div class="mb-6">
        <label for="email" class="text-white">Email:</label>
        <input
          type="email"
          id="email"
          v-model="email"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
      </div>

      <div class="mb-8">
        <label for="password" class="text-white">Contraseña:</label>
        <input
          type="password"
          id="password"
          v-model="password"
          class="w-full px-4 py-3 mt-1 text-lg text-white bg-gray-700 rounded-md focus:outline-none focus:ring focus:border-purple-500"
        />
      </div>

      <button
        type="submit"
        class="w-full bg-purple-600 text-white p-3 rounded-md hover:bg-purple-700 focus:outline-none focus:ring focus:border-purple-500"
      >
        Ingresar
      </button>

      <p class="mt-4 text-white">
        ¿No tienes una cuenta?
        <router-link to="/register" class="underline">Registrate</router-link>.
      </p>
    </form>
  </div>
</template>

<script setup lang="ts">
import { authLogin } from "@/api/auth";
import { AuthStore } from "@/states/auth";
import { ref } from "vue";
import { useRouter } from "vue-router";
import useToasterStore from "@/states/toast";

const toasterStore = useToasterStore();

const router = useRouter();
const email = ref<string>("");
const password = ref<string>("");
const authStore = AuthStore();

// Handle Login
const handleLogin = async (): Promise<void> => {
  try {
    const login = await authLogin(email.value, password.value);
    if (login.token) {
      authStore.setToken(login.token);
      toasterStore.success({ text: "Inicio de sesión exitoso" });
      router.push("/");
    } else {
      toasterStore.error({ text: "Credenciales invalidas" });
      authStore.clearToken();
    }
  } catch (error) {
    console.error("Error during login:", error);
  }
};
</script>
