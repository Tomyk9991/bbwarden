<template>
    <div aria-live="polite" aria-atomic="true">
        <div class="toast-container position-absolute start-0 top-0 p-3">
            <div v-for="toast in toasts" class="toast show" role="alert" aria-live="assertive" aria-atomic="true">
                <div class="toast-header">
                    <strong class="me-auto">{{ toast.title }}</strong>
                    <button type="button" class="btn-close" aria-label="Close"></button>
                </div>
                <div class="toast-body" v-html="toast.message" />
            </div>
        </div>
    </div>
</template>

<script>
export class ToastSingleton {
    constructor() {
        this.toasts = [];
        this.toastsChangedCallbacks = [];
    }

    append(title, message) {
        this.toasts.push({ title, message });
        this.notifyToastsChanged();
        
        setTimeout(() => {
            this.remove(title);
        }, 1000);
    }

    onToastsChanged(callback) {
        this.toastsChangedCallbacks.push(callback);
    }
    
    notifyToastsChanged() {
        for (let callback of this.toastsChangedCallbacks) {
            callback(this.toasts);
        }
    }
    
    remove(title) {
        this.toasts = this.toasts.filter((toast) => toast.title !== title);
        this.notifyToastsChanged();
    }
}
export default {
    inject: ['TOAST_INSTANCE'],
    data() {
        return {
            toasts: []
        }
    },
    mounted() {
        this.TOAST_INSTANCE.onToastsChanged((toasts) => {
            if (toasts.length == 0) {
                this.toasts = [];
            } else {
                this.toasts = toasts;
            }
        });
    }
}
</script>