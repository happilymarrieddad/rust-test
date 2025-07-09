<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter();
const authStore = useAuthStore()

const email = ref('')
const password = ref('')

const login = function() {
    authStore.login(email.value, password.value).then((err) => {
        if (err) {
            console.log(err)
        return
        }

        router.push('/')
    })
}
</script>

<template>
    <input v-model="email" type="text" placeholder="email" />
    <input v-model="password" type="password" placeholder="password" />
    <button @click="login">Login</button>
</template>