import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";

import Home from "./Home.vue";
import Login from "./Login.vue"
import MyProfile from "./MyProfile.vue";
import Register from "./Register.vue"
import RegisterProfile from "./RegisterProfile.vue";
import Post from "./Post.vue"
import CreatePost from "./CreatePost.vue";
import EditPost from "./EditPost.vue";
import Profile from "./Profile.vue";

const routes: RouteRecordRaw[] = [
  {
    path: "/",
    name: "Home",
    component: Home,
  },
  {
    path: "/login",
    name: "Login",
    component: Login,
  },
  {
    path: "/register",
    name: "Register",
    component: Register,
  },
  {
    path: "/create-profile",
    name: "CreateProfile",
    component: RegisterProfile,
  },
  {
    path: "/myprofile",
    name: "MyProfile",
    component: MyProfile,
  },
  {
    path: "/create-post",
    name: "CreatePost",
    component: CreatePost,
  },
  {
    path: "/post/:id",
    name: "Post",
    component: Post,
  },
  {
    path: "/edit-post/:id",
    name: "EditPost",
    component: EditPost,
  },
  {
    path: "/profile/:id",
    name: "Profile",
    component: Profile,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
