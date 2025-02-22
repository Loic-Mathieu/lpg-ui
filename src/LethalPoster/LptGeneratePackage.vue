<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface Data {
  packageName: string;
  files: string[];
  modes: string[];
}

const form = ref();
const formData = ref<Data>({
  packageName: 'TEMP',
  files: [],
  modes: []
});

function validateModes(value: string[]): boolean | string {
  if (value?.length > 0) {
    return true;
  }

  return 'Select at least one mode';
}

async function submit () {
  const {valid} = await form.value.validate();

  console.log(formData.value);

  if (valid) {
    // TODO validation and error handling
    await invoke("generate", formData.value);
    form.value.resetValidation();
  }
}
</script>

<template>
  <v-container>
    <v-form ref="form" @submit.prevent="submit">
      <div>
        <v-file-input
            accept="image/*"
            label="Input pictures"
            prepend-icon="mdi-image"
            :multiple="true"
        ></v-file-input>

        <v-text-field
            v-model="formData.packageName"
            label="Output package Name"
            prepend-icon="mdi-label"
        ></v-text-field>
      </div>

      <div>
        <h3>Generation mode</h3>
        <v-container fluid>
          <v-switch
              v-model="formData.modes"
              color="primary"
              label="Generate posters"
              value="posters"
              :rules="[(values: string[]) => validateModes(values)]"
              hide-details
          ></v-switch>
          <v-switch
              v-model="formData.modes"
              color="primary"
              label="Generate paintings"
              value="paintings"
              :rules="[(values: string[]) => validateModes(values)]"
          ></v-switch>
        </v-container>
      </div>

      <div>
        <v-btn type="submit"
               color="primary"
               block>
          Generate
        </v-btn>
      </div>
    </v-form>
  </v-container>
</template>
