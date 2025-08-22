<script setup lang="ts">
import {useHead} from '@unhead/vue'
useHead({
  title: 'GilCloud | Login',
  meta: [
    { name: 'description', content: 'Cloud Storage + Movies + Music & more on GilCloud' },
    { property: 'og:title', content: 'GilCloud | Login' },
    { property: 'og:description', content: 'Cloud Storage + Movies + Music & more on GilCloud' },
    { property: 'og:type', content: 'website' },
  ]
})
import {ref} from "vue"
import axios from "axios"
import {useAppStore} from "../stores/app.ts";
import { useRouter } from "vue-router";
import { useApiUrl } from "../main.ts";
import { getLocalStorageItem } from "../util/browser.ts";
const emit = defineEmits(["loginSuccess"])
// Reactive state
const router = useRouter();
const isLogin = ref(true)
const username = ref("")
const password = ref("")
const errorMessage = ref("")
const store = useAppStore();
  if(getLocalStorageItem('username', '')){
    router.push({name: 'home'})
  }
// Toggle form
const toggleForm = () => {
  isLogin.value = !isLogin.value
  errorMessage.value = ""
}

// Handle form submit
const handleSubmit = async () => {
  if (!username.value || !password.value) {
    errorMessage.value = "Username and password are required."
    return
  }

  try {
    const url = `${useApiUrl()}`.concat(isLogin.value ? "/login" : "/register")
    console.log(url)
    const response = await axios.post(url, {
      username: username.value,
      password: password.value,
    })

    errorMessage.value = ""

    if (isLogin.value) {
      // Save all response keys to localStorage
      const data = response.data
      for (const [key, value] of Object.entries(data)) {
        localStorage.setItem(key, JSON.stringify(value).replace(/^"|"$/g, ''))
      }
      // Optionally redirect or update UI here
      console.log(response.data)
      store.username = response.data.username
      store.setUserHomeDir(response.data.home_dir)
      store.setStorageCount({
        "maxStorage": response.data.storage_total/ 1048576,
        "currentUsage": response.data.storage_used/ 1048576,
      })
      console.log("home dir set: "+store.userHomeDir)
      alert("Login successful!")
      store.setIsLoggedIn(true)
      await router.push(router.currentRoute.value.query.redirect?.toString() || '/');
      emit("loginSuccess")
    } else {
      alert("Registration successful! You can now log in.")
      isLogin.value = true
    }
  } catch (error: any) {
    errorMessage.value =
        error.response?.data?.message || "Login incorrect."
  }
}
</script>

<template>
  <div
      class="bg-login backdrop-blur-3xl fixed inset-0 z-100 bg-neutral-900 w-screen h-screen"
  >
    <div class="flex w-full h-full items-center justify-center">
      <div
          class="flex flex-col items-center justify-center bg-neutral-800 p-8 rounded-lg shadow-lg w-96"
      >
        <h1 class="mb-2">Welcome to GilCloud</h1>
        <h2 class="text-3xl font-semibold text-white mb-6">
          {{ isLogin ? "Login" : "Register" }}
        </h2>

        <!-- Error Message -->
        <div
            v-if="errorMessage"
            class="w-full mb-4 text-red-400 text-sm text-center"
        >
          {{ errorMessage }}
        </div>

        <!-- Username input -->
        <input
            v-model="username"
            type="text"
            placeholder="Username"
            class="w-full mb-4 px-4 py-2 rounded bg-neutral-700 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-green-700"
        />

        <!-- Password input -->
        <input
            v-model="password"
            type="password"
            placeholder="Password"
            class="w-full mb-6 px-4 py-2 rounded bg-neutral-700 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-green-700"
        />

        <!-- Submit button -->
        <button
            @click="handleSubmit"
            class="w-full bg-green-600 hover:bg-green-800 text-white font-bold py-2 px-4 rounded transition duration-200 cursor-pointer"
        >
          {{ isLogin ? "Login" : "Register" }}
        </button>

        <!-- Divider -->
        <div class="my-4 text-gray-400">or</div>

        <!-- Switch mode button -->
        <button
            @click="toggleForm"
            class="w-full bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition duration-200 cursor-pointer"
        >
          {{ isLogin ? "Register" : "Login" }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* Add any extra styles if needed */
</style>
