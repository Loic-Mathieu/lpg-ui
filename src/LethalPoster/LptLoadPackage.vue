<script setup lang="ts">
import {ref} from "vue";

const isOutputFolderSet = ref(true);
const isModFolderSet = ref(true);
</script>

<script lang="ts">
export default {
  data: () => ({
    generatedPackages: [
      {
        title: 'A'
      },
      {
        title: 'B'
      },
    ]
  })
}
</script>

<template>
  <v-toolbar density="compact">
    <v-btn variant="elevated" color="secondary">Import package</v-btn>
    <v-spacer></v-spacer>
    <v-btn variant="tonal" color="secondary" :disabled="!isOutputFolderSet">Open folder in explorer</v-btn>
  </v-toolbar>
  <v-container>
    <v-list>
      <v-list-item v-for="pkg in generatedPackages"
                   :key="pkg.title"
                   :title="pkg.title"
                   prepend-icon="mdi-folder-zip">
        <template v-slot:append>
          <v-btn icon="mdi-upload-box" color="primary" class="ma-1" :disabled="!isModFolderSet"></v-btn>
          <v-btn icon="mdi-delete" color="error" class="ma-1"></v-btn>
        </template>
      </v-list-item>
    </v-list>
  </v-container>
</template>