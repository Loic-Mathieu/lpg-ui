<script setup lang="ts">
import {ref} from "vue";
import {readDir, watch, remove, copyFile} from '@tauri-apps/plugin-fs';
import {open, save} from "@tauri-apps/plugin-dialog";
import {PathData, getPathData, joinPath, filters} from "../utils/fileUtils.ts";

// Todo get from store
const PATH = 'D:\\work\\lpg-ui\\src-tauri\\output';
const MODS = 'D:\\Jeux\\r2modman\\LethalCompany\\profiles\\Mace\\BepInEx\\plugins';

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
        .map((from) => copyFile(from.path, joinPath(PATH, from.file)));

    Promise.all(copies).then(loadDir);
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

    copyFile(from.path, joinPath(to.uri, file)).then(loadDir)
  });
}

function loadDir() {
  readDir(PATH).then((entries) => {
    // Clear
    packages.value = [];

    // Populate
    entries.forEach((entry) => {
      packages.value.push({
        ...getPathData(joinPath(PATH, entry.name)),
        created: '23/02/2030',
      });
    });
  });
}

function deletePackage(path: string) {
  remove(path).then(loadDir);
}

watch(PATH, loadDir).then(loadDir);
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
                <v-btn variant="outlined" class="ml-1 mr-5" color="secondary" :disabled="!isModFolderSet">
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