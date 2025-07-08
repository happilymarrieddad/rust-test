import { defineStore } from 'pinia'
import { ref } from 'vue'

interface User {
  id: number
  first_name: string
  last_name: string
  email: string
  created_at: Date
  updated_at: Date
  active: boolean
  deleted_at: Date
}

export const useUsersStore = defineStore('users', () => {
  const user = ref({} as User)

  function setUser(u: User) {
    user.value = u
  }

  return { user }
})
