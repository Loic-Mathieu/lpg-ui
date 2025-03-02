<script setup lang="ts">
import {onMounted, onUnmounted, ref} from "vue";
import {open} from "@tauri-apps/plugin-dialog";
import {useSettingsStore} from "../stores/settingsStore.ts";

const settingsStore = useSettingsStore();
const formData = settingsStore.settings;
const form = ref();

onMounted(() => {
  settingsStore.init();
});

onUnmounted(() => {
  // Load the saved settings
  settingsStore.loadStore();
});

function openFolder() {
  open({directory: true}).then((selection: string | null) => {
    // Todo this is not efficient
    if (selection && formData.global) {
      formData.global.plugin_path = selection;
    }
  });
}

function openFolder2() {
  open({directory: true}).then((selection: string | null) => {
    // Todo this is not efficient
    if (selection && formData.lpg) {
      formData.lpg.output = selection;
    }
  });
}

function validatePath(value: string): boolean | string {
  if (value?.length > 0) {
    return true;
  }

  return 'Path required !';
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
            <v-text-field
                v-model="formData.global.plugin_path"
                :rules="[(path: string) => validatePath(path)]"
                label="Mod tool folder path"
                variant="outlined"
                clearable
            >
              <template v-slot:append>
                <v-btn @click="openFolder">Open</v-btn>
              </template>
            </v-text-field>
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
            <v-text-field
                class="path-picker"
                v-model="formData.lpg.output"
                :rules="[(path: string) => validatePath(path)]"
                label="Exported zip path"
                variant="outlined"
                clearable
            >
              <template v-slot:append>
                <v-btn @click="openFolder2">Open</v-btn>
              </template>
            </v-text-field>
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
