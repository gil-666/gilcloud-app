<template>
  <div class="banner shadow-lg w-full inset-0 flex top-0 items-center h-fit justify-between px-4 py-2 z-100">
    <!-- Logo -->
    <router-link class="flex place-items-center space-x-2" to="/">
      <img src="@/assets/logtrans.png" alt="GilCloud Logo" class="h-10" />
      <p class="text-2xl font-bold h-fit">GilCloud</p>
    </router-link>

    <!-- Navigation -->
    <nav>
      <button
        v-if="!isLoggedIn"
        @click="goLogin"
        class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 transition"
      >
        Log In
      </button>

      <div v-else class="flex items-center">
        <p class="text-white mr-4">Hello, {{ userName }}</p>
        <router-link
          to="/"
          class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 transition"
        >
          Go to Drive
        </router-link>
      </div>
    </nav>
</div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { getLocalStorageItem } from "../../util/browser";

// Example auth state (replace with your Pinia or Vuex store)
const user = ref<{ name: string } | null>(null);

// Router instance
const router = useRouter();

// Computed helpers
const isLoggedIn = computed(() => getLocalStorageItem('username','') !== null);
const userName = computed(() => getLocalStorageItem('username','') ?? "");

// Methods
const goLogin = () => {
  router.push("/login");
};
</script>

<style scoped>
/* Optional: sticky header */
.banner{
    background-color: rgb(34, 34, 34);
    transition: background-color 300ms ease-in-out;
}
.banner:hover{
    background-color: rgb(20, 20, 20);
}
header {
  position: sticky;
  top: 0;
  z-index: 50;
}
</style>
