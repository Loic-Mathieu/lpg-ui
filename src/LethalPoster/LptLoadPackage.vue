<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import {readDir, watch, remove, copyFile} from '@tauri-apps/plugin-fs';
import {open, save} from "@tauri-apps/plugin-dialog";
import {PathData, getPathData, joinPath, filters} from "../utils/fileUtils.ts";
import {invoke} from "@tauri-apps/api/core";
import {useSettingsStore} from "../stores/settingsStore.ts";

const settingsStore = useSettingsStore();
const path = computed(() => settingsStore.settings.lpg.output);

onMounted(() => {
  settingsStore.init();
});

interface Package extends PathData {
  created: string,
}

const isOutputFolderSet = ref(true);
const isModFolderSet = ref(true);

const packages = ref<Package[]>([]);

function importPackage() {
  open({
    multiple: true,
    directory: false,
    filters: filters.Package
  }).then((files: string[] | null) => {
    if (!files) {
      return;
    }

    const copies = files.map(getPathData)
        .map((from) => copyFile(from.path, joinPath(path.value, from.file)));

    Promise.all(copies).then(initDir);
  });
}

function exportPackage(from: PathData) {
  save({
    filters: filters.Package
  }).then((toPath: string | null) => {
    if (!toPath) {
      return;
    }

    const to = getPathData(toPath);
    const file = to.file ?? from.file;

    copyFile(from.path, joinPath(to.uri, file)).then(initDir)
  });
}

function initDir() {
  readDir(path.value).then((entries) => {
    // Clear
    packages.value = [];

    // Populate
    entries.forEach((entry) => {
      packages.value.push({
        ...getPathData(joinPath(path.value, entry.name)),
        created: '23/02/2030',
      });
    });
  });
}

function deletePackage(path: string) {
  remove(path).then(initDir);
}

watch(path.value, initDir).then(initDir);

function loadPackage(packageName: string | undefined) {
  if (packageName) {
    invoke("load", {packageName}).then((response) => {
      console.log(response)
    });
  }
}
</script>

<template>
  <v-container>
    <v-data-iterator :items="packages" :items-per-page="-1">
      <template v-slot:header>
        <v-toolbar density="compact">
          <v-btn @click="importPackage" variant="elevated" color="secondary">
            <span>Import package</span>
            <v-icon class="ml-1" icon="mdi-folder-download"></v-icon>
          </v-btn>
          <v-spacer></v-spacer>
          <v-btn variant="tonal" color="secondary" :disabled="!isOutputFolderSet">Open folder in explorer</v-btn>
        </v-toolbar>
      </template>

      <template v-slot:default="{ items }">
        <v-list>
          <template v-for="(pkg, i) in items" :key="pkg.raw.name">
            <v-list-item :title="pkg.raw.name"
                         prepend-icon="mdi-folder-zip"
                         @click=""
                         :ripple="false"
            >
              <template v-slot:append>
                <v-btn @click="loadPackage(pkg.raw.name)"
                       variant="outlined"
                       class="ml-1 mr-5"
                       color="secondary"
                       :disabled="!isModFolderSet"
                >
                  <span>Load</span>
                  <v-icon class="ml-1" icon="mdi-upload-box"></v-icon>
                </v-btn>
                <v-tooltip location="top" text="Export">
                  <template v-slot:activator="{ props }">
                    <v-btn v-bind="props"
                           @click="exportPackage(pkg.raw)"
                           variant="plain"
                           class="ml-1"
                           icon="mdi-folder-upload"
                           color="secondary"
                    ></v-btn>
                  </template>
                </v-tooltip>
                <v-tooltip location="top" text="Delete">
                  <template v-slot:activator="{ props }">
                    <v-btn v-bind="props"
                           @click="deletePackage(pkg.raw.path)"
                           variant="plain"
                           class="ml-1"
                           icon="mdi-delete"
                           color="error"
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
  </v-container>
</template>