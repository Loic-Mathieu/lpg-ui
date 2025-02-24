import {defineStore} from "pinia";
import {load} from "@tauri-apps/plugin-store";
import {onUnmounted, ref} from "vue";
import {listen, UnlistenFn} from "@tauri-apps/api/event";

export interface Settings {
    global: { plugin_path: string },
    lpg: { output: string }
}

const SETTINGS_KEY = 'settings';
const jsonKey = `${SETTINGS_KEY}.json`

export const useSettingsStore = defineStore(SETTINGS_KEY, () => {
    const settings = ref<Settings>({
        global: {plugin_path: ''},
        lpg: {output: ''},
    });
    const isLoading = ref(false);
    let unlisten: UnlistenFn | null = null;

    async function loadStore(): Promise<void> {
        console.log('INIT', SETTINGS_KEY);
        isLoading.value = true;
        return load(jsonKey, {autoSave: false}).then(async (store) => {
            settings.value.global = await store.get('global') ?? {plugin_path: ''};
            settings.value.lpg = await store.get('lpg') ?? {output: ''};

            isLoading.value = false;
        });
    }

    async function saveStore() {
        isLoading.value = true;
        return load(jsonKey, {autoSave: false}).then(async (store) => {
            return Promise.all([
                store.set('global', settings.value.global),
                store.set('lpg', settings.value.lpg)
            ])
                .then(() => store.save())
                .then(() => isLoading.value = false);
        });
    }

    async function init() {
        unlisten = await listen('settings-updated', loadStore);
        await loadStore();
    }

    function cleanupListener() {
        if (unlisten) {
            unlisten();
            unlisten = null;
        }
    }

    onUnmounted(() => cleanupListener);

    return {settings, isLoading, loadStore, saveStore, init}
})
