<template>
    <main @click="store.closeMenus()" class="bg-neutral-800 w-full lg:p-5 p-2 pt-2 text-center h-full overflow-hidden">
    <div class="flex relative place-items-center w-full justify-center">
      <h1 class="text-3xl pb-2 relative text-center" title="Movies">Movies</h1>
    </div>
    <div class="border-1 border-neutral-600 m-5 relative bottom-5 p-10 overflow-y-auto h-full max-h-11/12 movies-view">
      <!-- <p class="text-lg text-left mb-5">Your movie collection will appear here.</p> -->
      <!-- Movie content will go here -->
      <div class="grid xl:grid-cols-6 lg:grid-cols-4 md:grid-cols-4 sm:grid-cols-3 grid-cols-2 gap-4">
        <div @click="playMovie(movie)" v-for="movie in movies" :key="movie.id" class="bg-neutral-700 p-4 rounded-lg cursor-pointer hover:bg-neutral-600 transition">
          <img :src="movie.cover" alt="Movie Thumbnail" class="w-full h-auto mb-4 rounded">
          <h2 class="text-xl mb-2">{{ movie.title }}</h2>
          <!-- <p class="text-sm mb-4">{{ movie.description }}</p> -->
          <!-- <button @click="playMovie(movie)" class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded">Play</button> -->
        </div>
      </div>
    </div>
    <VideoPlayer v-if="selectedMovie" :subtracks="selectedMovie.subtracks" :title="selectedMovie.title" :src="selectedMovie.master" @close="selectedMovie = null" />
  </main>
</template>

<script setup lang="ts">
import { useAppStore } from '../stores/app';
import { ref } from 'vue';
import { GetMovies } from '../service/movies';
import VideoPlayer from '../components/media/VideoPlayer.vue';
const store = useAppStore();
const selectedMovie = ref(null);
const rawMovies = await GetMovies();
const movies = rawMovies.map(movie => ({
    ...movie,
    master: getAbsoluteHlsUrl(movie.master),
    cover: getAbsoluteHlsUrl(movie.cover),
    audiotracks: movie.audiotracks?.map(getAbsoluteHlsUrl) ?? [],
    subtracks: movie.subtracks?.map(getAbsoluteHlsUrl) ?? []
}));
console.log(movies);

function playMovie(movie) {
  selectedMovie.value = movie;
}
function getAbsoluteHlsUrl(src: string): string {
    if (src.startsWith('http')) return src; // already absolute
    return `${window.API_URL}${src}`;
}
</script>

<style scoped>
.movies-view {
    padding: 2rem;
}
</style>