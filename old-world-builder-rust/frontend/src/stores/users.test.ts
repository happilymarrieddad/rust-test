import { describe, expect, it, test } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

import { useAuthStore } from '@/stores/auth'
import { useUsersStore } from '@/stores/users'
import type { GetUsersResponse, User } from '@/types/user';
import { beforeEach } from 'node:test';

describe('user store tests', () => {
    let users: User[];

    beforeEach(async () => {
        setActivePinia(createPinia());

        const authStore = useAuthStore();
        const userStore = useUsersStore();
        const err = await authStore.login('nick@mail.com', '1234');
        expect(err).toBeUndefined();

        const [usrs, err2] = await userStore.getUsers(50, 0)
        expect(err2).toBeUndefined();
        expect(usrs).not.toHaveLength(0);

        const userData = usrs as GetUsersResponse;
        users = userData.users;
    })

    it('should successfully get a user', async () => {
        const userStore = useUsersStore();
        
        const [usr, err3] = await userStore.getUser(users[0].id)
        expect(err3).toBeUndefined();
        expect(usr?.email).toEqual(users[0].email);
    })
})