<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import { useUsersStore } from '@/stores/users'

const authStore = useAuthStore()
const userStore = useUsersStore();

const login = function () {
  authStore.login('nick@mail.com', '1234').then((err) => {
    if (err) {
      console.log(err)
      return
    }

    userStore.getUsers(50,0,'')
      .then(([users, err]) => console.log(users))
  })
}
</script>

<template>
  <main>
    <input type="button" @click="login" />
  </main>
</template>
