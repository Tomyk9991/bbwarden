import { createApp } from "vue";
import "./styles.css";

import "../src/scss/styles.scss";

// Import all of Bootstrap's JS
import * as bootstrap from 'bootstrap'
import Login from "./components/Login.vue";
import Tresor from "./components/tresor/Tresor.vue";

import { createRouter, createWebHashHistory } from 'vue-router'
import App from "./App.vue";
import { ToastSingleton } from "./services/ToastManager.vue";

const routes = [
    { path: "/", component: Login },
    { path: "/unlocked/:sessionID", component: Tresor }
];

const router = createRouter({
    history: createWebHashHistory(),
    routes: routes
});



let app = createApp(App);
app.use(router);
app.provide('TOAST_INSTANCE', new ToastSingleton());

app.mount("#app");
