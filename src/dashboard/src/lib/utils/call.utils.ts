import type { ActorMethod } from "@dfinity/agent"
import { toastsStore } from "@dfinity/gix-components"

export const tryCall = async <
  Args extends unknown[] = unknown[],
  Ret = unknown,
>(
  fn: ActorMethod<Args, Ret>,
  ...args: Args
): Promise<Ret> => {
  try {
    return await fn(...args)
  } catch (e) {
    toastsStore.show({
      text: "Failed to connect to Internet Computer",
      level: "error",
    })
    throw e
  }
}
