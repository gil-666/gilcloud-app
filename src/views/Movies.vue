<template>
  <main @click="store.closeMenus()" class="bg-neutral-800 w-full lg:p-5 p-2 pt-2 text-center h-full overflow-hidden">
    <div class="flex relative place-items-center w-full justify-center">
      <h1 class="text-3xl pb-2 relative text-center" title="Movies">Movies</h1>
    </div>
    <div class="m-5 relative bottom-5 overflow-y-auto h-full max-h-11/12 movies-view">
      <!-- <p class="text-lg text-left mb-5">Your movie collection will appear here.</p> -->
      <!-- Movie content will go here -->
      <div class="grid xl:grid-cols-6 lg:grid-cols-4 md:grid-cols-4 sm:grid-cols-3 grid-cols-2 gap-2">
        <div @click="playMovie(movie)" v-for="movie in movies" :key="movie.id" class=" p-2 rounded-lg cursor-pointer">
          <img ref="movieCoverArtRef" :src="movie.cover" alt="Movie Thumbnail"
            class="cover-art w-full h-auto mb-4 rounded"
            @mousemove="(e) => (handleMouseMove(e, e.currentTarget as HTMLImageElement))"
            @mouseleave="(e) => resetTransform(e.currentTarget as HTMLImageElement)">
          <h2 class="text-xl mb-2">{{ movie.title }}</h2>
          <!-- <p class="text-sm mb-4">{{ movie.description }}</p> -->
          <!-- <button @click="playMovie(movie)" class="bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded">Play</button> -->
        </div>
      </div>
    </div>
    <VideoPlayer v-if="selectedMovie" :subtracks="selectedMovie.subtracks" :title="selectedMovie.title"
      :src="selectedMovie.master" :movie="selectedMovie" @close="selectedMovie = null" />
  </main>
</template>

<script setup lang="ts">
import { useHead } from '@unhead/vue'

// Set static meta for SSG
useHead({
  title: 'GilCloud | Movies',
  meta: [
    { name: 'description', content: 'Browse all movies on GilCloud.' },
    { property: 'og:title', content: 'GilCloud | Movies' },
    { property: 'og:description', content: 'Watch movies online with ease' },
    { property: 'og:type', content: 'website' },
    { property: 'og:image', content: 'https://api.hanekawa.online/movies/2/cover.jpg' }
  ]
})
import { useAppStore } from '../stores/app';
import { onMounted, Ref, ref } from 'vue';

import { GetMovies } from '../service/movies';
import { handleMouseMove, resetTransform } from '../util/anim';
import VideoPlayer from '../components/media/VideoPlayer.vue';
import { MovieStructure } from '../util/fileTypes';
import { useApiUrl } from '../main';
const movieCoverArtRef: Ref<HTMLImageElement | null> = ref(null)
const store = useAppStore();
const selectedMovie: Ref<MovieStructure | null> = ref(null);
const movies = ref()
onMounted(async () => {
  const rawMovies = await GetMovies();


  movies.value = rawMovies.map((movie: { title: string, master: string; cover: string; audiotracks: string[]; subtracks: string[]; }) => ({
    ...movie,
    title: movie.title,
    master: getAbsoluteHlsUrl(movie.master),
    cover: getAbsoluteHlsUrl(movie.cover),
    audiotracks: movie.audiotracks?.map(getAbsoluteHlsUrl) ?? [],
    subtracks: movie.subtracks?.map(getAbsoluteHlsUrl) ?? []
  }));
})

console.log(movies);

function playMovie(movie: MovieStructure) {
  selectedMovie.value = movie;
}
function getAbsoluteHlsUrl(src: string): string {
  if (src.startsWith('http')) return src; // already absolute
  return `${useApiUrl()}${src}`;
}
</script>

<style scoped>
.cover-art {
  /* transition: transform 0.1s linear; */
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
}

.movies-view {
  padding: 2rem;
}
</style>