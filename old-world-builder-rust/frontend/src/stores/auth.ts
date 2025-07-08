import { ref } from 'vue'
import { defineStore } from 'pinia'
import cookie from 'cookiejs'
import { request } from '@/stores/requester'

const cookieToken = 'rust-test-token'

interface loginResponse {
  status: string
  token: string
}

export const useAuthStore = defineStore('auth', () => {
  const cookie_token = cookie.get(cookieToken)

  const token = ref(typeof cookie_token === 'string' ? cookie_token : '')
  const loading = ref(false)

  function setToken(new_token: string) {
    token.value = new_token
  }

  function clearToken() {
    cookie.clear()
    token.value = ''
  }

  async function login(
    email: string,
    password: string,
  ): Promise<[string | undefined, string | undefined]> {
    return new Promise(async (resolve) => {
      const [tokenData, err] = await request('/login', { email, password }, 'POST', '')
      if (err) {
        return resolve([undefined, err])
      }

      const data = tokenData as loginResponse

      setToken(data.token)
      return resolve([data.token, undefined])
    })
  }

  return {
    // vars
    token,
    loading,
    // helpers
    setToken,
    clearToken,
    // funcs
    login,
  }
})
