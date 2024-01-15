<template>
  <Toast />

  <div id="app" class="flex flex-col h-screen">
    <Navbar title="SocialWither" :links="authStatus ? authLinks : noauthLinks">
      <button
        @click="toggleSidebar"
        class="rounded-full bg-purple-500 p-2 hover:bg-purple-700"
      >
        <Bars3Icon class="h-8 w-8 text-white" />
      </button>
    </Navbar>

    <div class="flex flex-1 overflow-hidden">
      <Transition name="slide">
        <Sidebar position="left" v-show="sidebarActive">
          <div class="flex flex-col h-full text-white p-4">
            <div class="flex items-center justify-center mb-8">
              <img
                v-if="authStatus && profileStore.profile"
                :src="`${imageApi}/${profileStore.profile.avatar}`"
                alt="Profile Avatar"
                class="h-24 w-24 rounded-full border-4 border-purple-500"
              />
            </div>

            <div v-if="authStatus && profileStore.profile" class="mb-4">
              <p class="text-lg font-semibold">
                {{ profileStore.profile.username }}
              </p>
              <p class="text-sm">{{ profileStore.profile.bio }}</p>
            </div>

            <div v-if="authStatus && profileStore.profile" class="mb-4">
              <hr class="border-t border-gray-600 my-2" />
              <router-link to="/create-post">
                <button
                  class="text-purple-700 transition duration-300 transform hover:scale-105 rounded-full px-4 py-2 bg-white hover:bg-purple-900 hover:text-white w-full mb-2"
                >
                  Nueva publicación
                </button>
              </router-link>
              <hr class="border-t border-gray-600 my-2" />
              <p class="font-bold m-1">Usuarios en linea:</p>
              <div>
                <div class="flex flex-wrap">
                  <div
                    v-for="user in onlineUsersStore.onlineUsers"
                    :key="user.id"
                    class="p-1"
                  >
                    <div
                      v-if="user.id.toString() === profileStore.profile.user_id"
                    >
                      <router-link to="/myprofile" class="text-center">{{
                        user.username
                      }}</router-link>
                    </div>
                    <div v-else>
                      <router-link
                        :to="`/profile/${user.id}`"
                        class="text-center"
                        >{{ user.username }}</router-link
                      >
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div v-else class="mb-4">
              <p class="text-lg">¡Hola!</p>
            </div>
          </div>
        </Sidebar>
      </Transition>

      <Container class="overflow-y-auto">
        <router-view />
      </Container>
    </div>
  </div>
</template>

<script setup lang="ts">
import Navbar from "@/components/Navbar.vue";
import Container from "@/components/Container.vue";
import Sidebar from "./components/Sidebar.vue";
import Toast from "./components/Toast.vue";
import { Bars3Icon } from "@heroicons/vue/24/solid";
import { onBeforeMount, ref, watch } from "vue";
import { checkAuth } from "@/api/auth";
import { checkProfile, getMyProfile } from "@/api/profiles";
import { AuthStore } from "@/states/auth";
import { ProfileStore } from "./states/profile";
import { useRouter } from "vue-router";
import { useOnlineUsersStore } from "@/states/users";

const imageApi = `${import.meta.env.VITE_APIURL}/images`;
const sidebarActive = ref<boolean>(true);
const authStatus = ref<boolean>(false);
const authStore = AuthStore();
const profileStore = ProfileStore();
const router = useRouter();
const onlineUsersStore = useOnlineUsersStore();

const noauthLinks = [
  { text: "Registrarse", url: "/register" },
  { text: "Iniciar sesión", url: "/login" },
];

const authLinks = [
  { text: "Inicio", url: "/" },
  { text: "Mi perfil", url: "/myprofile" },
];

const socket = ref<WebSocket | null>(null);

const connectToWebSocket = () => {
  if (!profileStore.profile) {
    return;
  }
  const newSocket = new WebSocket(
    `ws://localhost:3000?username=${encodeURIComponent(
      profileStore.profile.username
    )}&userId=${profileStore.profile.id}`
  );

  newSocket.addEventListener("open", (event) => {
    console.log("WebSocket Connection Established:", event);
  });

  newSocket.addEventListener("message", (event) => {
    const data = JSON.parse(event.data);

    if (data.type === "onlineUsers") {
      onlineUsersStore.updateOnlineUsers(data.users);
    }
  });

  newSocket.addEventListener("close", (event) => {
    console.log("WebSocket Connection Closed:", event);
  });

  socket.value = newSocket;
};

const closeWebSocket = () => {
  if (socket.value) {
    socket.value.close();
    console.log("WebSocket Connection Closed");
  }
};

router.beforeEach(async (to, _from, next) => {
  let token = authStore.token;
  const status = await checkAuth(token);
  authStatus.value = status;

  if (!status) {
    if (to.path === "/login" || to.path === "/register") {
      next();
    } else {
      next({ path: "/login" });
    }
  } else {
    if (token === null) {
      next({ path: "/login" });
    } else {
      const hasProfile = await checkProfile(token);

      if (!hasProfile) {
        if (to.path === "/create-profile") {
          next();
        } else {
          next({ path: "/create-profile" });
        }
      } else {
        if (
          to.path === "/login" ||
          to.path === "/register" ||
          to.path === "/create-profile"
        ) {
          next({ path: "/" });
        } else {
          next();
        }
      }
    }
  }
});

onBeforeMount(async () => {
  let token = authStore.token;
  const status = await checkAuth(token);
  authStatus.value = status;
  if (status && token) {
    const profile = await getMyProfile(token);
    if (profile) {
      profileStore.setProfile(profile);
      connectToWebSocket();
    }
  } else {
    profileStore.clearProfile();
  }
});

watch(
  () => authStore.token,
  async (newToken) => {
    console.log("updated!:", newToken);
    if (!newToken) {
      authStatus.value = false;
      closeWebSocket();
    } else {
      const status = await checkAuth(newToken);
      const profile = await getMyProfile(newToken);
      authStatus.value = status;
      if (profile) {
        profileStore.setProfile(profile);
        connectToWebSocket();
      }
    }
  }
);

const toggleSidebar = () => {
  sidebarActive.value = !sidebarActive.value;
};
</script>

<style>
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.5s ease, opacity 0.5s ease;
}

.slide-enter-from,
.slide-leave-to {
  transform: translateX(-100%);
  opacity: 0;
}

.slide-enter-to,
.slide-leave-from {
  transform: translateX(0);
  opacity: 1;
}
</style>
