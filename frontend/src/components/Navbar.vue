<template>
    <nav
      class="bg-gradient-to-r from-purple-600 to-purple-800 p-4 shadow-md"
    >
      <div class="container mx-auto flex items-center justify-between">
        <!-- Alinea el slot a la izquierda y agrega un margen a la derecha -->
        <slot class="mr-auto"></slot>
  
        <h1
          class="text-white text-3xl font-extrabold transition duration-300 transform hover:scale-105"
        >
          {{ title }}
        </h1>
        
        <ul class="flex space-x-4">
          <li v-for="(link, index) in links" :key="index">
            <router-link
              :to="link.url"
              class="text-purple-700 transition duration-300 transform hover:scale-105 rounded-full px-4 py-2 bg-white hover:bg-purple-900 hover:text-white"
            >
              {{ link.text }}
            </router-link>
          </li>
        </ul>
      </div>
    </nav>
  </template>
  
  <script setup lang="ts">
  defineProps<{
    title: string;
    links: { text: string; url: string }[];
  }>();
  </script>
  