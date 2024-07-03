import { Principal } from "@dfinity/principal";
import { tryCall } from "$lib/utils/call.utils";
import { authStore } from "$lib/stores/auth.store";
import { Actor } from "@dfinity/agent";
import {
  idlFactory as monitor_idl,
  type _SERVICE as _MONITOR_SERVICE,
  type CanisterCycles,
  type CanisterMemorySize,
  type CycleHistory,
  type GroupInfo,
  type Log,
  type Logger,
  type RewardableActivity,
} from "$lib/declarations/monitor.did.js";
import { createAgent } from "@dfinity/utils";
import { _SERVICE as _REWARD_SERVICE, idlFactory as rewardIdl } from "$lib/api/reward.declaration";
import { _SERVICE as _PROXY_SERVICE, idlFactory as proxyIdl } from "./proxy.declaration";

const rewardCanisterId = "zgfl7-pqaaa-aaaap-accpa-cai";
const monitorCanisterId = "6or45-oyaaa-aaaap-absua-cai";
const proxyCanisterId = "2jvhk-5aaaa-aaaap-ahewa-cai";

const rewardActor = async () => {
  const identity = await authStore.identity();
  const agent = await createAgent({
    identity,
    host: "https://icp-api.io",
  });
  return Actor.createActor<_REWARD_SERVICE>(rewardIdl, {
    canisterId: rewardCanisterId,
    agent,
  });
};

const proxyActor = async () => {
  const identity = await authStore.identity();
  const agent = await createAgent({
    identity,
    host: "https://icp-api.io",
  });
  return Actor.createActor<_PROXY_SERVICE>(proxyIdl, {
    canisterId: proxyCanisterId,
    agent,
  });
};

const monitorActor = async () => {
  const identity = await authStore.identity();
  const agent = await createAgent({
    identity,
    host: "https://icp-api.io",
  });
  return Actor.createActor<_MONITOR_SERVICE>(monitor_idl, {
    canisterId: monitorCanisterId,
    agent,
  });
};

// Monitor interface
export const icpBalance = async () => {
  const monitor = await monitorActor();
  return await tryCall<[], string>(monitor.icp_balance);
};

export const latestIcpBalances = async (n: bigint) => {
  const monitor = await monitorActor();
  return await tryCall<[bigint], [bigint, number][]>(monitor.latest_icp_balances, n);
};

export const sortedCanisterCycles = async () => {
  const monitor = await monitorActor();
  return await tryCall<[], CanisterCycles[]>(monitor.sorted_canister_cycles);
};

export const sortedMemorySizes = async () => {
  const monitor = await monitorActor();
  return await tryCall<[], CanisterMemorySize[]>(monitor.sorted_memory_sizes);
};

export const canisterCycleHistory = async (n: bigint) => {
  const monitor = await monitorActor();
  return await tryCall<[bigint], CycleHistory>(monitor.canister_cycle_history, n);
};

export const latestLogs = async (n: bigint) => {
  const monitor = await monitorActor();
  return await tryCall<[bigint], Log[]>(monitor.get_latest_logs, n);
};

// Proxy interface
export const latestProxyLogs = async (n: bigint) => {
  const monitor = await monitorActor();
  return await tryCall<[bigint], Logger[]>(monitor.latest_proxy_logs, n);
};

export const proxyLogSize = async () => {
  const monitor = await monitorActor();
  return await tryCall<[], bigint>(monitor.proxy_log_size);
};

export const readProxyRewardBuffer = async () => {
  const actor = await proxyActor();
  const response = await actor.read_reward_buffer();

  return response.map((r) => {
    if ("UserActivity" in r.activity) {
      return { activity: "UserActivity", id: r.activity.UserActivity.toString(), timestamp: r.timestamp };
    }
    if ("GroupMemberCount" in r.activity) {
      return { activity: "GroupMemberCount", id: r.activity.GroupMemberCount.toString(), timestamp: r.timestamp };
    }

    return { activity: "unknown", id: "", timestamp: r.timestamp };
  });
};

export const rewardTimerNextTrigger = async () => {
  const actor = await proxyActor();
  return await actor.reward_timer_next_trigger();
};

export const proxyStoreStats = async () => {
  const monitor = await monitorActor();
  return await tryCall(monitor.proxy_store_stats);
};

// Rewards interface
export const groupInfo = async () => {
  const canister = await rewardActor();
  return canister.all_group_info();
};

export const userInfo = async () => {
  const canister = await rewardActor();
  return canister.all_user_info();
};

export const tokenBalances = async () => {
  const canister = await rewardActor();
  return await canister.all_token_balances();
};

export const tokenLogSize = async () => {
  const monitor = await monitorActor();
  return await tryCall<[], bigint>(monitor.token_log_size);
};

export const graphMemberCountRewards = async () => {
  const canister = await rewardActor();
  return await canister.graph_member_count_rewards();
};

export const graphMemberActivityRewards = async () => {
  const canister = await rewardActor();
  return await canister.graph_member_activity_rewards();
};
