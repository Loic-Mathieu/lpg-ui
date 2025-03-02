<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref} from "vue";
import {useSettingsStore} from "../stores/settingsStore.ts";
import PathPicker from "./PathPicker.vue";

const settingsStore = useSettingsStore();
const formData = settingsStore.settings;
const form = ref();

const themeInfo = computed(() => {
  return {
    isThemeDark: settingsStore.isThemeDark,
    icon: settingsStore.isThemeDark ? 'mdi-moon-waxing-crescent' : 'mdi-white-balance-sunny',
    name: settingsStore.isThemeDark ? 'dark' : 'light',
  }
})

onMounted(() => {
  settingsStore.init();
});

onUnmounted(() => {
  // Load the saved settings
  settingsStore.loadStore();
});

function changeTheme() {
  settingsStore.settings.global.theme = themeInfo.value.isThemeDark ? 'light' : 'dark';
}

async function submit() {
  const {valid} = await form.value.validate();

  if (valid && !settingsStore.isLoading) {
    await settingsStore.saveStore();
  }
}
</script>

<template>
  <v-container>
    <v-form ref="form" @submit.prevent="submit">
      <!-- GENERAL -->
      <v-row>
        <v-col class="ma-3">
          <v-row>
            <h1>General settings</h1>
          </v-row>
          <v-row>
            <PathPicker v-model="formData.global.plugin_path" label="Mod tool folder path"/>
          </v-row>
          <v-row>
            <v-col cols="8" class="align-content-end">
              <h2 class="align-content-end">Theme :</h2>
            </v-col>
            <v-col>
              <v-btn block :prepend-icon="themeInfo.icon" @click="changeTheme">{{themeInfo.name}}</v-btn>
            </v-col>
          </v-row>
        </v-col>
      </v-row>

      <v-divider></v-divider>

      <!-- LETHAL PACKAGE -->
      <v-row>
        <v-col class="ma-3">
          <v-row>
            <h1>Lethal Poster settings</h1>
          </v-row>
          <v-row>
            <PathPicker v-model="formData.lpg.output" label="Exported zip path"/>
          </v-row>
        </v-col>
      </v-row>

      <!-- SUBMIT -->
      <v-row>
        <v-col>
          <a href="#/" class="text-decoration-none text-white">
            <v-btn block
                   :disabled="settingsStore.isLoading"
            >
              Cancel
            </v-btn>
          </a>
        </v-col>
        <v-col>
          <v-btn type="submit"
                 color="primary"
                 block
                 :disabled="settingsStore.isLoading"
          >
            Save
          </v-btn>
        </v-col>
      </v-row>
    </v-form>
  </v-container>
</template>
