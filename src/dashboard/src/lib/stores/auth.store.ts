import type { Readable } from "svelte/store"
import type { Identity } from "@dfinity/agent"
import { writable } from "svelte/store"
import { AuthClient } from "@dfinity/auth-client"
import { goto } from "$app/navigation"

interface AuthStore extends Readable<Identity | null> {
  login: () => Promise<void>
  logout: () => Promise<void>
  isAuthenticated: () => Promise<boolean>
  identity: () => Promise<Identity>
}

let authClient = await AuthClient.create()

const initAuthStore = async (): Promise<AuthStore> => {
  const { set, subscribe } = writable<Identity | null>(null)

  ;(await authClient.isAuthenticated())
    ? set(authClient.getIdentity())
    : set(null)

  return {
    subscribe,

    login: async () => {
      authClient = authClient ?? (await AuthClient.create())
      await authClient.login({
        identityProvider: "https://identity.internetcomputer.org",
        maxTimeToLive: BigInt(98 * 60 * 60) * BigInt(1_000_000_000_000),
        onSuccess: async () => {
          set(authClient.getIdentity())
          console.log("login success")
          console.log(authClient.getIdentity().getPrincipal().toString())
        },
      })
    },

    logout: async () => {
      authClient = authClient ?? (await AuthClient.create())
      await authClient.logout()
      set(null)
    },

    isAuthenticated: async (): Promise<boolean> => {
      // re-init authClient to check delegation chain validity
      authClient = await AuthClient.create()
      if (await authClient.isAuthenticated()) {
        return true
      } else {
        set(null)
        goto("/")
        return false
      }
    },

    identity: async (): Promise<Identity> => {
      // re-init authClient to check delegation chain validity
      authClient = await AuthClient.create()
      if (await authClient.isAuthenticated()) {
        return authClient.getIdentity()
      } else {
        set(null)
        goto("/")
        throw new Error("Not authenticated")
      }
    },
  }
}

export const authStore = await initAuthStore()
