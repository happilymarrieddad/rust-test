<script lang="ts">
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

export default {
    setup() {
        const router = useRouter();
        const authStore = useAuthStore();

        authStore.clearToken();
        setTimeout(() => {
            router.push('/login')
        }, 100)
    }
}
</script>

<template>
    <span>logging out...</span>
</template>