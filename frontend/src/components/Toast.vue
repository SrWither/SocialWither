<template>
  <Teleport to="body">
    <Transition name="toast">
      <div
        v-if="toastStore.toasts.length"
        class="fixed top-3 right-1/2 left-1/2 z-100 flex flex-col gap-4 m-1 justify-center items-center"
      >
        <TransitionGroup name="toast" tag="ul">
          <li
            v-for="toast in toastStore.toasts"
            :class="[
              'flex items-center gap-4 border rounded p-7 m-2 border-transparent w-max max-w-3xl',
              toastClassMap[toast.status],
            ]"
            :key="toast.text"
          >
            <div v-if="toast.status === 'error'">
              <XMarkIcon class="w-6 h-6" />
            </div>
            <div v-else-if="toast.status === 'info'">
              <InformationCircleIcon class="w-6 h-6" />
            </div>
            <div v-else-if="toast.status === 'success'">
              <CheckIcon class="w-6 h-6" />
            </div>
            <div v-else-if="toast.status === 'warning'">
              <ExclamationTriangleIcon class="w-6 h-6" />
            </div>

            <span class="text-lg font-semibold">
              {{ toast.text }}
            </span>
          </li>
        </TransitionGroup>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import useToasterStore, { TToastStatus } from "@/states/toast";
import {
  XMarkIcon,
  InformationCircleIcon,
  ExclamationTriangleIcon,
  CheckIcon,
} from "@heroicons/vue/24/solid";

const toastClassMap: Record<TToastStatus, string> = {
  warning: "bg-yellow-500",
  error: "bg-red-500",
  success: "bg-green-500",
  info: "bg-purple-500",
};

const toastStore = useToasterStore();
</script>

<style scoped>
.toast-enter-from,
.toast-leave-to {
  transform: translateY(-100%);
  opacity: 0;
}

.toast-enter-active,
.toast-leave-active {
  transition: 0.25s ease all;
}
</style>
