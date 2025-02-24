<script setup lang="ts">
import {ref} from "vue";
import {open} from "@tauri-apps/plugin-dialog";
import {load} from "@tauri-apps/plugin-store";

export interface OutputDirSettings {
  output: string | null;
}

export interface Settings {
  global: {plugin_path: string | null},
  lpg: OutputDirSettings
}

const formData = ref<Settings>({
  global: {plugin_path: null},
  lpg: {output: null},
});
const form = ref();

function openFolder() {
  open({directory: true}).then((selection: string | null) => {
    // Todo this is not efficient
    if (selection && formData.value) {
      formData.value.global.plugin_path = selection;
    }
  });
}
function openFolder2() {
  open({directory: true}).then((selection: string | null) => {
    // Todo this is not efficient
    if (selection && formData.value) {
      formData.value.lpg.output = selection;
    }
  });
}

function validatePath(value: string): boolean | string {
  if (value?.length > 0) {
    return true;
  }

  return 'Path required !';
}

load('settings.json', {autoSave: false}).then(async (store) => {
  formData.value.global = await store.get('global') ?? {plugin_path: ''};
  formData.value.lpg = await store.get('lpg') ?? {output: ''};
})

async function submit() {
  const {valid} = await form.value.validate();

  if (valid) {
    // Update the settings
    const store = await load('settings.json', {autoSave: false});

    Promise.all([
      store.set('global', formData.value?.global),
      store.set('lpg', formData.value?.lpg)
    ])
        .then(() => store.save().then(form.value.resetValidation))
  }
}
</script>

<template>
  <v-container>
    <v-form ref="form" @submit.prevent="submit">
      <h1>General settings</h1>

      <v-text-field
          class="path-picker"
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

      <v-divider></v-divider>

      <h1>Lethal Poster settings</h1>

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

      <v-btn type="submit"
             color="primary"
             block>
        Save
      </v-btn>
    </v-form>
  </v-container>
</template>

<style scoped>
.path-picker {
  padding-bottom: 10px;
}
</style>
