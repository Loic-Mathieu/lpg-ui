<script setup lang="ts">
import LptGeneratePackage from "./LptGeneratePackage.vue";
import LptLoadPackage from "./LptLoadPackage.vue";
</script>

<script lang="ts">
export default {
  data: () => ({
    tab: null,
  }),
}
</script>

<template>
  <v-tabs
      v-model="tab"
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
