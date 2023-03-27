<script>
import {invoke} from "@tauri-apps/api/tauri";

export default {
    data() {
        return {
            password: '',
            userName: '',
            loginRequested: false,
            sessionID: '',
            showPasswordInvalidLabel: false
        }
    },
    mounted() {
        this.loginRequested = false;
        const KEY = "userName";

        let item = localStorage.getItem(KEY);
        
        if (item == null) {
            console.log(`Caching ${KEY}`);
            localStorage.setItem(KEY, "sample@web.de");
        }

        this.userName = localStorage.getItem(KEY);
    },
    beforeRouteLeave(to, from, next) {
        if (this.loginRequested) {
            next();
        }
    },
    methods: {
        async onUnlock() {
            let passwordHash = await this.hash256(this.password);
            invoke("unlock", {userName: this.userName, password: passwordHash})
                .then((result) => {
                    this.sessionID = result.session_id;

                    this.showPasswordInvalidLabel = false;
                    this.loginRequested = true;
                    this.$router.push(`/unlocked/${this.sessionID}`);
                    return;
                })
                .catch((err) => {
                    console.log(err);
                    this.showPasswordInvalidLabel = true;
                    setTimeout(() => {
                        this.showPasswordInvalidLabel = false;
                    }, 1000);
                });
        },
        async hash256(string) {
            const utf8 = new TextEncoder().encode(string);
            return crypto.subtle.digest('SHA-256', utf8).then((hashBuffer) => {
                const hashArray = Array.from(new Uint8Array(hashBuffer));
                const hashHex = hashArray
                    .map((bytes) => bytes.toString(16).padStart(2, '0'))
                    .join('');
                return hashHex.trim();
            });
        }
    }
}


</script>

<template>
    <div class="container mt-5">
        <!--        Locked header-->
        <div class="d-flex justify-content-center">
            <i class="bi-lock" style="font-size: 60px"/>
        </div>
        <div class="d-flex justify-content-center">
            Dein Tresor ist gesperrt. Überprüfe deine Identität, um fortzufahren
        </div>

        <div class="mt-5">
            <div class="form-group mx-5 px-5">
                <label class="d-flex justify-content-center">Password</label>
                <div class="input-group">
                    <input ref="passwordInput" @keyup.enter="onUnlock" type="password" v-model="password"
                           class="form-control" :class="{ 'is-invalid': showPasswordInvalidLabel }"
                           placeholder="Password">
                    <div class="invalid-feedback">
                        Passwort ist ungültig
                    </div>
                </div>
            </div>
            <div class="d-flex justify-content-center mt-3" style="font-size: 12px">
                Lokal eingeloggt als "Sample Name"
            </div>
        </div>
        <div class="d-flex justify-content-center mt-3">
            <button @click="onUnlock" class="me-1 btn btn-primary">Entsperren</button>
        </div>
    </div>
</template>
