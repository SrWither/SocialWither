import { defineStore } from 'pinia';

interface User {
  id: number;
  username: string;
}

export const useOnlineUsersStore = defineStore('onlineUsers', {
  state: () => ({
    onlineUsers: [] as User[],
  }),
  actions: {
    updateOnlineUsers(users: User[]) {
      this.onlineUsers = users;
    },
  },
});
