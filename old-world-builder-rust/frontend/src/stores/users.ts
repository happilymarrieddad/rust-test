import { defineStore } from 'pinia'
import { ref } from 'vue'

import type { User, GetUsersResponse, CreateUser, UpdateUser } from '@/types/user';
import { useAuthStore } from './auth';
import { request } from './requester';

const userRouteUri = "/v1/users"

const useUsersStore = defineStore('users', () => {
  const user = ref({} as User)
  const authStore = useAuthStore();

  function setUser(u: User) {
    user.value = u
  }

  async function getUser(id: number): Promise<[User | undefined, string | undefined]> {
    return new Promise(async (resolve) => {
      const [u, err] = await request(`${userRouteUri}/${id}`, {}, 'GET', authStore.token)
      if (err) {
        return resolve([undefined, err])
      }

      return resolve([u as User,undefined])
    })
  }

  async function getUsers(limit: number, offset: number, email: string): Promise<[GetUsersResponse | undefined, string | undefined]> {
    return new Promise(async (resolve) => {
      const [res, err] = await request(`${userRouteUri}?limit=${limit}&offset=${offset}&email=${email}`, {}, 'GET', authStore.token)
      if (err) {
        return resolve([undefined, err])
      }

      return resolve([res as GetUsersResponse, undefined])
    })
  }

  async function createUser(u: CreateUser): Promise<[User | undefined, string | undefined]> {
    return new Promise(async (resolve) => {
      const [res, err] = await request(`${userRouteUri}`, u, 'POST', authStore.token)
      if (err) {
        return resolve([undefined, err])
      }

      return resolve([res as User, undefined])
    })
  }

  async function updateUser(id: number, u: UpdateUser): Promise<[User | undefined, string | undefined]> {
    return new Promise(async (resolve) => {
      const [res, err] = await request(`${userRouteUri}/${id}`, u, 'PUT', authStore.token)
      if (err) {
        return resolve([undefined, err])
      }

      return resolve([res as User, undefined])
    })
  }

  async function deleteUser(id: number): Promise<string | undefined> {
    return new Promise(async (resolve) => {
      const [_, err] = await request(`${userRouteUri}/${id}`, {}, 'DELETE', authStore.token)
      if (err) {
        return resolve(err)
      }

      return resolve(undefined)
    })
  }

  return { user, setUser, getUser, getUsers, createUser, updateUser, deleteUser }
})

type UsersStore = ReturnType<typeof useUsersStore>

export { useUsersStore, type UsersStore }
