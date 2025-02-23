<script setup lang="ts">
import {computed, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import {PathData, getPathData, filters} from "../utils/fileUtils.ts";

interface ListedFile extends PathData {
  poster: boolean;
  painting: boolean;
}

interface Data {
  packageName: string;
  files: Map<string, ListedFile>;
}

const form = ref();
const formData = ref<Data>({
  packageName: '',
  files: new Map<string, ListedFile>([]),
});

const fileList = computed<ListedFile[]>(() => {
  return [...formData.value.files.values()];
});

function addFiles() {
  form.value.resetValidation();

  open({
    multiple: true,
    directory: false,
    filters: filters.Pictures
  }).then((files: string[] | null) => {
    if (!files) {
      return;
    }

    files.forEach((path: string) => {
      formData.value.files.set(path, {
        ...getPathData(path),
        poster: true,
        painting: true
      });
    });
  });
}

function removeFile(key: string) {
  formData.value.files.delete(key);
}

function removeAllFiles() {
  formData.value.files.clear();
}

function validateFiles(): boolean | string {
  if (fileList.value?.some((value: ListedFile) => value.poster || value.painting)) {
    return true;
  }

  return 'Select at least one file to generate';
}

function validateName(value: string): boolean | string {
  if (value?.length > 0) {
    return true;
  }

  return 'Enter a name for the package';
}

async function submit() {
  const {valid} = await form.value.validate();

  console.log(formData.value);

  if (valid) {
    // TODO validation and error handling
    await invoke("generate", {
      packageName: formData.value.packageName,
      files: fileList.value
    });
    form.value.resetValidation();
  }
}
</script>

<template>
  <v-container>
    <v-form ref="form" @submit.prevent="submit">
      <v-row dense>
        <v-col>
          <v-data-iterator :items="fileList" :items-per-page="-1">
            <template v-slot:header>
              <v-toolbar density="compact">
                <v-btn variant="flat" color="secondary" @click="addFiles">
                  <span>Add files to convert</span>
                  <v-icon class="ma-1" icon="mdi-file-plus-outline"></v-icon>
                </v-btn>
                <v-input class="ml-2" :rules="[() => validateFiles()]"></v-input>
                <v-spacer></v-spacer>
                <v-tooltip location="top" text="Remove all items">
                  <template v-slot:activator="{ props }">
                    <v-btn v-bind="props" color="secondary" @click="removeAllFiles">Clear</v-btn>
                  </template>
                </v-tooltip>
              </v-toolbar>
            </template>

            <template v-slot:default="{ items }">
              <v-list>
                <template v-for="(item, i) in items" :key="item.raw.path">
                  <v-list-item
                      :prepend-icon="'mdi-image'"
                      :title="item.raw.name"
                      :subtitle="item.raw.uri"
                      @click=""
                      :ripple="false"
                  >
                    <template v-slot:append>
                      <v-switch
                          class="ml-8"
                          v-model="item.raw.poster"
                          color="secondary"
                          label="Poster"
                          hide-details
                      ></v-switch>
                      <v-switch
                          class="ml-8"
                          v-model="item.raw.painting"
                          color="secondary"
                          label="Painting"
                          hide-details
                      ></v-switch>
                      <v-divider class="ml-4" vertical></v-divider>
                      <v-tooltip location="top" text="Remove">
                        <template v-slot:activator="{ props }">
                          <v-btn
                              @click="removeFile(item.raw.path)"
                              v-bind="props"
                              class="ml-3"
                              variant="text"
                              density="compact"
                              color="error"
                              icon="mdi-close"
                          ></v-btn>
                        </template>
                      </v-tooltip>
                    </template>
                  </v-list-item>
                  <v-divider v-if="i < items.length - 1" class="ma-1"></v-divider>
                </template>
              </v-list>
            </template>
          </v-data-iterator>
        </v-col>
      </v-row>

      <v-row dense>
        <v-col>
          <v-text-field
              v-model="formData.packageName"
              label="Output package Name"
              density="comfortable"
              :rules="[(value) => validateName(value)]"
          >
            <template v-slot:append>
              <v-btn type="submit"
                     color="primary"
                     block
              >
                <span>Generate Package</span>
                <v-icon class="ma-1" icon="mdi-package-variant-closed"></v-icon>
              </v-btn>
            </template>
          </v-text-field>
        </v-col>
      </v-row>
    </v-form>
  </v-container>
</template>
