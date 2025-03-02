import {defineStore} from "pinia";
import {load} from "@tauri-apps/plugin-store";
import {computed, onUnmounted, ref} from "vue";
import {listen, UnlistenFn} from "@tauri-apps/api/event";
import {isNotEmpty} from "../utils/utils.ts";

interface GlobalSettings {
    plugin_path: string
    theme: 'dark' | 'light'
}

interface LpgSettings {
    output: string
}

export interface Settings {
    global: GlobalSettings,
    lpg: LpgSettings
}

const DEFAULT_GLOBAL: GlobalSettings = {
    plugin_path: '',
    theme: 'dark'
}

const DEFAULT_LPG: LpgSettings = {
    output: ''
}

const SETTINGS_KEY = 'settings';
const jsonKey = `${SETTINGS_KEY}.json`

/**
 * SETTINGS STORE
 */
export const useSettingsStore = defineStore(SETTINGS_KEY, () => {
    /**
     * The app settings
     */
    const settings = ref<Settings>({
        global: DEFAULT_GLOBAL,
        lpg: DEFAULT_LPG,
    });
    /**
     * True is the app loading any settings
     */
    const isLoading = ref(false);
    let unlisten: UnlistenFn | null = null;

    onUnmounted(() => cleanupListener);

    /**
     * Load settings
     */
    async function loadStore(): Promise<void> {
        isLoading.value = true;
        return load(jsonKey, {autoSave: false}).then(async (store) => {
            settings.value.global = await store.get('global') ?? DEFAULT_GLOBAL;
            settings.value.lpg = await store.get('lpg') ?? DEFAULT_LPG;

            isLoading.value = false;
        });
    }

    /**
     * Save settings
     */
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

    /**
     * Inits the store
     */
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

    /**
     * Return true if the theme is dark
     */
    const isThemeDark = computed(() => settings.value.global.theme === 'dark');

    /**
     * Return true if the plugin path is set in the settings
     */
    const isPluginPathSet = computed(() => {
        const path = settings.value.global.plugin_path;
        return isNotEmpty(path);
    });

    /**
     * Return true if the package output path is set in the settings
     */
    const isPackagePathSet = computed(() => {
        const path = settings.value.lpg.output;
        return isNotEmpty(path);
    });

    return {settings, isLoading, loadStore, saveStore, init, isThemeDark, isPluginPathSet, isPackagePathSet}
})
