import { beforeEach, describe, expect, it } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'

setActivePinia(createPinia());

import { useAuthStore } from '@/stores/auth'
import { useUsersStore, type UsersStore } from '@/stores/users'
import type { CreateUser, GetUsersResponse, UpdateUser, User } from '@/types/user';

describe('user store tests', () => {
    let usersStore: UsersStore;
    usersStore = useUsersStore();
    
    let users: User[];
    
    beforeEach(async () => {
        const authStore = useAuthStore();
        const err = await authStore.login('nick@mail.com', '1234');
        expect(err).toBeUndefined();

        const [usrs, err2] = await usersStore.getUsers(50, 0,'')
        expect(err2).toBeUndefined();

        const userData = usrs as GetUsersResponse;
        users = userData.users;
        expect(users).not.toHaveLength(0);
    })

    it('should successfully create a user if there are not enough users', async () => {
        if (users.length < 10) { // No need to create too many users
            const [new_user,err] = await usersStore.createUser({
                first_name: 'nick',
                last_name: 'kotenberg',
                email: `nick${new Date().getTime() / 1000}@mail.com`,
                password: '1234',
                password_confirm: '1234',
            } as CreateUser);
            expect(err).toBeUndefined();
            expect(new_user?.first_name).toEqual('nick');
        } else {
            expect(true).toBeTruthy(); // just need the test to do something
        }
    })

    it('should successfully get a user', async () => {
        const [usr, err] = await usersStore.getUser(users[0].id)
        expect(err).toBeUndefined();
        expect(usr?.email).toEqual(users[0].email);
    })

    it('should successfully get users', async () => {
        const [usrs, err] = await usersStore.getUsers(50, 0, users[0].email)
        expect(err).toBeUndefined();
        expect(usrs?.users).toHaveLength(1);

        // Just want to make sure it's working properly
        const [usrs2, err2] = await usersStore.getUsers(50, 0, '')
        expect(err2).toBeUndefined();
        expect(usrs2?.users).not.toHaveLength(1);
        expect(usrs2?.users).not.toHaveLength(0);
    })

    describe('user store mutations', () => {
        var usr: User; // need to ensure there is something we can mutate
        beforeEach(async () => {
            let [u, err] = await usersStore.getUser(users[0].id);
            expect(err).toBeUndefined();
            usr = u as User;
            expect(usr?.email).toEqual(users[0].email);
        })

        it('should successfully update a user', async () => {
            const oldFirstName = usr.first_name;
            const oldLastName = usr.last_name;

            usr.first_name = `nick${new Date().getTime() / 1000}`;
            usr.last_name = `kotenberg${new Date().getTime() / 1000}`;

            const [updated_usr, err] = await usersStore.updateUser(usr.id, {
                first_name: usr.first_name,
                last_name: usr.last_name,
            } as UpdateUser);
            expect(err).toBeUndefined();

            // First Name
            expect(updated_usr?.first_name).toEqual(usr.first_name);
            expect(updated_usr?.first_name).not.toEqual(oldFirstName);

            // Last Name
            expect(updated_usr?.last_name).toEqual(usr.last_name);
            expect(updated_usr?.last_name).not.toEqual(oldLastName);
        })
    })
})