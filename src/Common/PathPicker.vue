<script setup lang="ts">
import {open} from "@tauri-apps/plugin-dialog";
import {isNotEmpty} from "../utils/utils.ts";

const model = defineModel({ required: true });
const props = defineProps<{
  label: string
}>();

function openFolder() {
  open({directory: true}).then((selection: string | null) => {
    if (selection) {
      model.value = selection;
    }
  });
}

function validatePath(value: string): boolean | string {
  if (isNotEmpty(value)) {
    return true;
  }

  return 'Path required !';
}
</script>

<template>
  <v-text-field
      v-model="model"
      :rules="[(path: string) => validatePath(path)]"
      :label="props.label"
      variant="outlined"
      clearable
  >
    <template v-slot:append>
      <v-btn @click="openFolder">Open</v-btn>
    </template>
  </v-text-field>
</template>