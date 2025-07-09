import { useAuthStore } from '@/stores/auth'
import { delay } from '@/utils/delay'
import { createRouter, createWebHistory } from 'vue-router'

let verified = false;

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      meta: { auth: true },
      component: () => import('../views/HomeView.vue')
    },
    {
      path: '/about',
      name: 'about',
      meta: { auth: true },
      component: () => import('../views/AboutView.vue'),
    },

    // Auth
    {
      path: '/login',
      name: 'login',
      component: () => import('../views/LoginView.vue'),
    },
    {
      path: '/logout',
      name: 'logout',
      meta: { auth: true },
      component: () => import('../views/LogoutView.vue'),
    },

    // Last Route
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('../views/HomeView.vue')
    },
  ],
})

router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()
  const authRequired = to && to.meta && to.meta.auth;
  await delay(0.25)

  if (to.name == 'not-found') {
    return next('/')
  }

  if (authRequired) {
    if (!authStore.token || authStore.token.length == 0) {
      return next('/login')
    }
  }

  return next()
})

router.afterEach(async () => {
  await delay(0.05) // I think this makes it feel better
})

export default router
