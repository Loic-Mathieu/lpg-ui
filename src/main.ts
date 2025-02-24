import {createApp} from "vue";
import App from "./App.vue";

const pinia = createPinia()

// Vuetify
import 'vuetify/styles';
import '@mdi/font/css/materialdesignicons.css' // Ensure you are using css-loader
import {createVuetify} from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import {createPinia} from "pinia";

const vuetify = createVuetify({
    theme: {
        defaultTheme: 'dark'
    },
    components,
    directives,
});

createApp(App).use(pinia).use(vuetify).mount("#app");
