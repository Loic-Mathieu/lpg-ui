<script setup lang="ts">
import {onMounted, ref} from "vue";
import {listen} from "@tauri-apps/api/event";
import {useSettingsStore} from "../stores/settingsStore.ts";

const settingsStore = useSettingsStore();
const isBackendLoading = ref<boolean>(false);

onMounted(() => {
  settingsStore.init();

  listen<boolean>('loading', ($event) => {
    isBackendLoading.value = $event.payload;
  });
});

</script>

<template>
  <v-overlay :model-value="isBackendLoading || settingsStore.isLoading" :persistent="true" class="align-center justify-center">
    <v-progress-circular
        color="primary"
        size="64"
        indeterminate
    ></v-progress-circular>
  </v-overlay>
</template>