<script setup lang="ts">
import {ref} from "vue";
import {open} from "@tauri-apps/plugin-dialog";
import {invoke} from "@tauri-apps/api/core";

const formData = ref<Record<string, any>>({
  modToolPath: "",
  picturesInputPath: "",
  picturesOutputPath: "",
  zipOutputPath: "",
});
const form = ref();

async function openFolder(key: string) {
  const selected = await open({directory: true});
  if (selected) {
    formData.value[key] = selected as string;
  }
}

function validatePath(value: string): boolean | string {
  if (value?.length > 0) {
    return true;
  }

  return 'Path required';
}

async function submit () {
  const {valid} = await form.value.validate();

  if (valid) {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    const response = await invoke<{message: string}>("generate", formData.value);
    console.log(response)
    form.value.resetValidation();
  }
}
</script>

<template>
  <v-container>
    <v-form ref="form" @submit.prevent="submit">
      <h1>General settings</h1>

      <v-text-field
          class="path-picker"
          v-model="formData.modToolPath"
          :rules="[(path: string) => validatePath(path)]"
          label="Mod tool folder path"
          variant="outlined"
          clearable
      >
        <template v-slot:append>
          <v-btn @click="openFolder(formData.modToolPath)">Open</v-btn>
        </template>
      </v-text-field>

      <v-divider></v-divider>

      <h1>Lethal Poster settings</h1>

      <v-text-field
          class="path-picker"
          v-model="formData.picturesInputPath"
          :rules="[(path: string) => validatePath(path)]"
          label="Input pictures path"
          variant="outlined"
          clearable
      >
        <template v-slot:append>
          <v-btn @click="openFolder(formData.picturesInputPath)">Open</v-btn>
        </template>
      </v-text-field>

      <v-text-field
          class="path-picker"
          v-model="formData.zipOutputPath"
          :rules="[(path: string) => validatePath(path)]"
          label="Exported zip path"
          variant="outlined"
          clearable
      >
        <template v-slot:append>
          <v-btn @click="openFolder(formData.zipOutputPath)">Open</v-btn>
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
