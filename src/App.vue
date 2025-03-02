<script setup lang="ts">
import Lpt from "./LethalPoster/Lpt.vue";
import Settings from "./Common/Settings.vue";
import Loader from "./Common/Loader.vue";
import {computed, onMounted, ref} from "vue";
import {useSettingsStore} from "./stores/settingsStore.ts";

const routes: Record<string, object> = {
  '/': Lpt,
  '/settings': Settings
}

const currentPath = ref(window.location.hash);

const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || '/'];
});

const settingsStore = useSettingsStore();

const theme = computed(() => settingsStore.isThemeDark ? 'dark' : 'light');

onMounted(() => {
  settingsStore.init();

  window.addEventListener('hashchange', () => {
    currentPath.value = window.location.hash
  })
});

</script>

<template>
  <v-app :theme="theme">
    <Loader></Loader>
    <v-app-bar>
      <v-app-bar-title>
        <a href="#/" class="" :class="[`action-${theme}`, 'text-uppercase']">
          Package tool
        </a>
      </v-app-bar-title>

      <a href="#/settings" :class="[`action-${theme}`, 'ma-1']">
        <v-btn icon="mdi-cog"></v-btn>
      </a>
    </v-app-bar>

    <v-main>
      <v-alert variant="tonal" v-if="!settingsStore.isPluginPathSet" icon="mdi-folder-alert" color="warning" text="The mod tool directory is not set in the app settings !">
        <template v-slot:append>
          <a href="#/settings" :class="[`action-${theme}`, 'ma-1']">
            <v-btn variant="text">Go to settings</v-btn>
          </a>
        </template>
      </v-alert>
      <component :is="currentView" />
    </v-main>
  </v-app>
</template>

<style>
.action-light {
  text-decoration: none;
  color: black;
}

.action-dark {
  text-decoration: none;
  color: white;
}
</style>
