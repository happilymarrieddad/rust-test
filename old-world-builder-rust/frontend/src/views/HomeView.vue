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
    <button @click="login">Click to test login func</button>

    <div class="home">
      <h1>This is a home page</h1>
    </div>
  </main>
</template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
