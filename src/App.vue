<script>
import {defineComponent} from "vue";
import ToastManager from "./services/ToastManager.vue";

export default defineComponent({
    components: {ToastManager}
})
</script>
<template>
    <ToastManager/>
    <main>
        <router-view />
    </main>
</template>

<style scoped>
</style>
