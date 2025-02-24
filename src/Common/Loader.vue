<script setup lang="ts">
import {ref} from "vue";
import {listen} from "@tauri-apps/api/event";

const isLoading = ref<boolean>(false);

listen<boolean>('loading', ($event) => {
  isLoading.value = $event.payload;
});
</script>

<template>
  <v-overlay :model-value="isLoading" :persistent="true" class="align-center justify-center">
    <v-progress-circular
        color="primary"
        size="64"
        indeterminate
    ></v-progress-circular>
  </v-overlay>
</template>