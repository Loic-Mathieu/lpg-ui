<script setup lang="ts">
import LptGeneratePackage from "./LptGeneratePackage.vue";
import LptLoadPackage from "./LptLoadPackage.vue";
import {ref} from "vue";

const tab = ref();
</script>

<template>
  <v-tabs
      v-model="tab"
      fixed-tabs
      bg-color="primary"
  >
    <v-tab value="generate">Generate package</v-tab>
    <v-tab value="load">Load package</v-tab>
  </v-tabs>

  <v-tabs-window v-model="tab">
    <v-tabs-window-item value="generate">
      <LptGeneratePackage></LptGeneratePackage>
    </v-tabs-window-item>

    <v-tabs-window-item value="load">
      <LptLoadPackage></LptLoadPackage>
    </v-tabs-window-item>
  </v-tabs-window>
</template>
