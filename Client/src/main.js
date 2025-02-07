import { createApp } from 'vue';
import App from './App.vue';
import initI18n from './i18n';
import router from './router';

const app = createApp(App);
app.use(router).use(initI18n({ locale: 'zhCN' }));
app.mount('#app');
