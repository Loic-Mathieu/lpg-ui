import { createApp } from "vue";
import App from "./App.vue";

// Vuetify
import 'vuetify/styles';
import '@mdi/font/css/materialdesignicons.css' // Ensure you are using css-loader
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

const vuetify = createVuetify({
    theme: {
        defaultTheme: 'dark'
    },
    components,
    directives,
});

createApp(App).use(vuetify).mount("#app");
