<script setup lang="ts">
import Lpt from "./LethalPoster/Lpt.vue";
import Settings from "./Common/Settings.vue";
import Loader from "./Common/Loader.vue";
import {computed, ref} from "vue";

const routes: Record<string, object> = {
  '/': Lpt,
  '/settings': Settings
}

const currentPath = ref(window.location.hash);

window.addEventListener('hashchange', () => {
  currentPath.value = window.location.hash
})

const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || '/'];
});

const isModFolderSet = ref(true);
</script>

<template>
  <v-app>
    <Loader></Loader>
    <v-app-bar>
      <v-app-bar-title>
        <a href="#/" class="text-decoration-none text-white">
          Package tool
        </a>
      </v-app-bar-title>

      <a href="#/settings" class="text-decoration-none text-white ma-1">
        <v-btn icon="mdi-cog"></v-btn>
      </a>
    </v-app-bar>

    <v-main>
      <v-alert v-if="!isModFolderSet" icon="mdi-folder-alert" color="warning" text="The mod tool directory is not set in the app settings"></v-alert>
      <component :is="currentView" />
    </v-main>
  </v-app>
</template>
