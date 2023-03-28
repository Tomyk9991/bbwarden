<script>
import {invoke} from "@tauri-apps/api/tauri";
import TresorEntry from "./TresorEntry.vue";

export default {
    components: {TresorEntry},
    data() {
        return {
            session: '',
            passwords: [],
            modalOpen: false,
            renderedPasswords: [],
            currentEntry: undefined,
            logoutRequested: false,
            newEntryEntered: {
                service: '',
                userName: '',
                password: ''
            }
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
        
        await this.fetchPasswords();
    },
    methods: {
        async fetchPasswords() {
            this.passwords = await invoke("fetch_passwords", { sessionId: this.session });
            this.renderedPasswords = this.passwords;
        },
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
        async onDeleted() {
            await this.fetchPasswords();
            this.currentEntry = undefined;
        },
        async createEntry() {
            await invoke('create_entry', {
                sessionId: this.session,
                service: this.newEntryEntered.service,
                userName: this.newEntryEntered.userName,
                password: this.newEntryEntered.password
            });
            
            this.newEntryEntered.userName = '';
            this.newEntryEntered.service = '';
            this.newEntryEntered.password = '';

            await this.fetchPasswords();
            
            this.modalOpen = false;
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
            <TresorEntry :current-entry="currentEntry" :session-id="session" :onDeleted="onDeleted" />
            
            <!--        Modal-->
            <div class="modal" :class="modalOpen ? 'active' : ''" tabindex="-1" role="dialog">
                <div class="modal-dialog" role="document">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title">Eintrag erstellen</h5>
                            <button @click="modalOpen = false" type="button" class="btn close" data-dismiss="modal" aria-label="Close">
                                <span aria-hidden="true">&times;</span>
                            </button>
                        </div>
                        <div class="modal-body">
                            <div class="form-group mb-2">
                                <label class="mb-1">Service:</label>
                                <input v-model="newEntryEntered.service" type="text" class="form-control" placeholder="Service eingeben">
                            </div>
                            <div class="form-group mb-2">
                                <label class="mb-1">Nutzername:</label>
                                <input v-model="newEntryEntered.userName" type="text" class="form-control" placeholder="Nutzernamen eingeben">
                            </div>
                            <div class="form-group">
                                <label class="mb-1">Passwort:</label>
                                <input v-model="newEntryEntered.password" type="password" class="form-control" placeholder="Passwort eingeben">
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button @click="createEntry" type="button" class="btn btn-primary">Eintrag erstellen</button>
                            <button @click="modalOpen = false" type="button" class="btn btn-secondary">Schließen</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <div @click="modalOpen = true" class="position-absolute bottom-0 ms-1 mb-5">
            <button class="btn btn-primary ms-2" style="width: 100px"><i class="bi-plus"/></button>
        </div>
    </div>
</template>
