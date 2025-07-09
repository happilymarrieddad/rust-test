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

interface CreateUser {
  first_name: string
  last_name: string
  email: string
  password: string
  password_confirm: string
}

interface UpdateUser {
  first_name: string
  last_name: string
}

interface GetUsersResponse {
  total: number;
  users: User[];
}

export type { User, CreateUser, UpdateUser, GetUsersResponse }