<script>

import {invoke} from "@tauri-apps/api/tauri";

export default {
    inject: ['TOAST_INSTANCE'],
    data() {
        return {
            latestSavedObject: {
                userName: '',
                encrypted_password: ''
            },
            modalOpen: false
        }
    },
    props: {
        currentEntry: {
            type: Object,
            required: true
        },
        onDeleted: {
            type: Function,
            required: true
        },
        sessionId: {
            type: String,
            required: true
        }
    },
    methods: {
        copyClipboard(data) {
            this.TOAST_INSTANCE.append('Zwischenablage', `<Code>'${data}'</Code> wurde in die Zwischenablage kopiert`);
            navigator.clipboard.writeText(data);
        },
        editEntry(entry) {
            switch (entry) {
                case 0:
                    if (this.currentEntry.readOnlyUserName === undefined) {
                        this.currentEntry.readOnlyUserName = true;
                    }
                    this.currentEntry.readOnlyUserName = !this.currentEntry.readOnlyUserName;
                    break;
                case 1:
                    if (this.currentEntry.readOnlyPassword === undefined) {
                        this.currentEntry.readOnlyPassword = true;
                    }
                    this.currentEntry.readOnlyPassword = !this.currentEntry.readOnlyPassword;
                    break;
            }
        },
        openModal() {
            this.modalOpen = true;
        },
        closeModal() {
            this.modalOpen = false;
        },
        async deleteEntry() {
            await invoke('remove_entry', {
                id: this.currentEntry.id,
                sessionId: this.sessionId,
            });
            
            this.closeModal();
            this.onDeleted();
        },
        async outOfFocus() {
            this.currentEntry.readOnlyUserName = true;
            this.currentEntry.readOnlyPassword = true;

            await this.saveEntry();
        },
        async saveEntry() {
            if (this.currentEntry.user_name === this.latestSavedObject.userName && this.currentEntry.encrypted_password === this.latestSavedObject.encrypted_password) {
                return;
            }
            
            this.latestSavedObject.userName = this.currentEntry.user_name;
            this.latestSavedObject.encrypted_password = this.currentEntry.encrypted_password;
            
            await invoke("edit_entry", {
                id: this.currentEntry.id,
                sessionId: this.sessionId,
                userName: this.currentEntry.user_name,
                password: this.currentEntry.encrypted_password
            });
        }
    }
}
</script>

<template>
    <div v-if="currentEntry !== undefined" class="tab-pane fade show active">
        <div class="input-group mb-2">
            <button @click="copyClipboard(currentEntry.user_name)" class="btn-secondary bi-clipboard input-group-text"
                    style="font-size: 24px;"/>
            <button @click="editEntry(0)"
                    :class="currentEntry.readOnlyUserName !== undefined && !currentEntry.readOnlyUserName ? 'bg-primary' : ''"
                    class="btn-secondary bi-pencil input-group-text" style="font-size: 18px;"/>
            <input @focusout="outOfFocus" class="form-control" v-model="currentEntry.user_name" type="text"
                   :readonly="currentEntry.readOnlyUserName !== undefined ? currentEntry.readOnlyUserName : true">
        </div>
        <div class="input-group mb-2">
            <button @click="copyClipboard(currentEntry.encrypted_password)"
                    class="btn-secondary bi-clipboard input-group-text" style="font-size: 24px;"/>
            <button @click="editEntry(1)"
                    :class="currentEntry.readOnlyPassword !== undefined && !currentEntry.readOnlyPassword ? 'bg-primary' : ''"
                    class="btn-secondary bi-pencil input-group-text" style="font-size: 18px;"/>
            <input @focusout="outOfFocus" class="form-control" type="password"
                   :readonly="currentEntry.readOnlyPassword !== undefined ? currentEntry.readOnlyPassword : true"
                   v-model="currentEntry.encrypted_password">
        </div>
    </div>

    <div v-if="currentEntry !== undefined" class="position-absolute bottom-0 mb-5">
        <button @click="openModal" class="btn btn-danger ms-2" style="width: 100px"><i class="bi-trash"/></button>
    </div>

<!--    Modal-->
    <div class="modal" :class="modalOpen ? 'active' : ''" tabindex="-1" role="dialog">
        <div class="modal-dialog" role="document">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title">Eintrag löschen</h5>
                    <button @click="closeModal" type="button" class="btn close" data-dismiss="modal" aria-label="Close">
                        <span aria-hidden="true">&times;</span>
                    </button>
                </div>
                <div class="modal-body">
                    <p>Sind Sie sicher, dass Sie den Eintrag löschen wollen?</p>
                </div>
                <div class="modal-footer">
                    <button @click="deleteEntry" type="button" class="btn btn-danger">Eintrag löschen</button>
                    <button @click="closeModal" type="button" class="btn btn-secondary">Schließen</button>
                </div>
            </div>
        </div>
    </div>
</template>


<style scoped>
</style>