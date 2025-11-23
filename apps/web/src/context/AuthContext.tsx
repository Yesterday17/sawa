import { createContext, useContext, type ReactNode } from 'react'
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import {
  postUserLoginMutation,
  postUserLogoutMutation,
  postUserRegisterMutation,
  getUserMeQueryKey,
  getUserMeOptions,
} from '../client/@tanstack/react-query.gen'
import type {
  PostUserLoginData,
  PostUserRegisterData,
  PublicUser,
} from '../client/types.gen'
import { useNavigate } from '@tanstack/react-router'

interface AuthContextType {
  user: PublicUser | null
  isAuthenticated: boolean
  isLoading: boolean
  login: (data: PostUserLoginData['body']) => Promise<void>
  register: (data: PostUserRegisterData['body']) => Promise<void>
  logout: () => Promise<void>
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: ReactNode }) {
  const navigate = useNavigate()
  const queryClient = useQueryClient()

  const { data: user, isLoading } = useQuery({
    ...getUserMeOptions(),
    retry: false,
  })

  const loginMutation = useMutation(postUserLoginMutation())
  const logoutMutation = useMutation(postUserLogoutMutation())
  const registerMutation = useMutation(postUserRegisterMutation())

  const login = async (data: PostUserLoginData['body']) => {
    await loginMutation.mutateAsync({ body: data })
    await queryClient.invalidateQueries({ queryKey: getUserMeQueryKey() })
    navigate({ to: '/products' })
  }

  const register = async (data: PostUserRegisterData['body']) => {
    await registerMutation.mutateAsync({ body: data })
    await queryClient.invalidateQueries({ queryKey: getUserMeQueryKey() })
    navigate({ to: '/products' })
  }

  const logout = async () => {
    try {
      await logoutMutation.mutateAsync({})
    } catch (e) {
      console.error('Logout failed', e)
    } finally {
      queryClient.setQueryData(getUserMeQueryKey(), null)
      navigate({ to: '/login' })
    }
  }

  return (
    <AuthContext.Provider
      value={{
        user: user || null,
        isAuthenticated: !!user,
        isLoading,
        login,
        register,
        logout,
      }}
    >
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
