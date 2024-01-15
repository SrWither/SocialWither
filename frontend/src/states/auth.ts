import { defineStore } from "pinia";
import { Auth } from "@/api/auth";

const LOCAL_STORAGE_KEY = "auth";

const getSettings = (): string | null => {
  const token = localStorage.getItem(LOCAL_STORAGE_KEY);

  return token ? JSON.parse(token) : null;
};

export const AuthStore = defineStore("auth", {
  state: (): Auth => ({
    token: getSettings(),
  }),
  actions: {
    setToken(token: string): void {
      this.token = token;
      localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(this.token));
    },
    clearToken(): void {
      this.token = null;
      localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(this.token));
    },
  },
});
