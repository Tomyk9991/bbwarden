<script>
import {invoke} from "@tauri-apps/api/tauri";
import TresorEntry from "./TresorEntry.vue";

export default {
    components: {TresorEntry},
    data() {
        return {
            session: '',
            passwords: [],
            renderedPasswords: [],
            currentEntry: undefined,
            logoutRequested: false
        }
    },
    beforeRouteLeave(to, from, next) {
        if (this.logoutRequested) {
            next();
        }
        
        if (to.path === '/') {
            next(false);
        } else {
            next();
        }
    },
    async mounted() {
        this.logoutRequested = false;
        this.session = this.$route.params.sessionID;
        this.passwords = await invoke("fetch_passwords", { sessionId: this.session });
        this.renderedPasswords = this.passwords;
    },
    methods: {
        isActiveToClass(entry) {
            return this.currentEntry ?
                this.currentEntry.service === entry.service ? 'active' : ''
                : '';
        },
        async logout() {
            this.logoutRequested = true;
            await invoke('logout', { sessionId: this.session });
            this.$router.push("/");
        },
        onSearchChanged(e) {
            this.renderedPasswords = this.passwords.filter(entry => entry.service.toLowerCase().includes(e.target.value.toLowerCase()));
        },
        onEntrySelected(entry) {
            this.currentEntry = entry;
        }
    }
}

</script>
<style scoped>
.grid-container {
    display: grid;
    height: 100vh;
    width: 100vw;
    grid-template-columns: 1fr 3fr;
    padding: 10px
}

.grid-item {
    padding: 20px;
}

.scrollable {
    overflow-y: auto;
}
</style>

<template>
    <div class="d-flex flex-column justify-content-center mt-3 me-1">
        <div class="d-flex">
            <h1 class="flex-grow-1 text-center m-0">Tresor</h1>
            <button @click="logout" class="h-auto btn btn-secondary">Ausloggen</button>
        </div>
    </div>
    <hr class="hr"/>
    
    <div class="grid-container">
        <div class="col grid-item scrollable mb-5">
            <div>
                Suche im Tresor
                <input @input="onSearchChanged" placeholder="Suche..." type="text" class="form-control"/>
            </div>
            <hr class="hr"/>
            <div class="list-group" id="list-tab" role="tablist">
                <a v-for="entry in renderedPasswords" 
                   @click="onEntrySelected(entry)" :class="{ 'active': isActiveToClass(entry) }"
                   class="list-group-item list-group-item-action" data-bs-toggle="list" role="tab">{{entry.service}}</a>
            </div>
        </div>
        <div class="tab-content grid-item">
            <TresorEntry :current-entry="currentEntry" :session-id="session" />
        </div>
    </div>
</template>
