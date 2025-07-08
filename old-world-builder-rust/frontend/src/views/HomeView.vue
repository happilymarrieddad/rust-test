<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()

const login = function () {
  authStore.login('nick@mail.com', '1234').then((data) => console.log(data))
}
</script>

<template>
  <main>
    <input type="button" @click="login" />
  </main>
</template>
