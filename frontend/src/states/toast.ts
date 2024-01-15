import { defineStore } from "pinia";

export type TToastStatus = "info" | "success" | "warning" | "error";

interface IToast {
  text: string;
  status: TToastStatus;
  id: number;
}

type ToastPayload = { timeout?: number; text: string };

const defaultTimeout = 2000;

const createToast = (text: string, status: TToastStatus): IToast => ({
  text,
  status,
  id: Math.random() * 1000,
});

export default defineStore("toaster-store", {
  state: (): { toasts: IToast[] } => ({
    toasts: [],
  }),
  actions: {
    updateState(payload: ToastPayload, status: TToastStatus) {
      const { text, timeout } = payload;

      const toast = createToast(text, status);

      // Instead of pushing the toast to the end, unshift it to the beginning
      this.toasts.unshift(toast);

      setTimeout(() => {
        this.toasts = this.toasts.filter((t) => t.id !== toast.id);
      }, timeout ?? defaultTimeout);
    },

    info(payload: ToastPayload) {
      this.updateState(payload, "info");
    },

    success(payload: ToastPayload) {
      this.updateState(payload, "success");
    },

    warning(payload: ToastPayload) {
      this.updateState(payload, "warning");
    },

    error(payload: ToastPayload) {
      this.updateState(payload, "error");
    },
  },
});
