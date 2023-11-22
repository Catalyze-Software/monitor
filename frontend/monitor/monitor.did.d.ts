import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Tokens { 'e8s' : bigint }
export interface _SERVICE {
  'get_icp_balance' : ActorMethod<[], Tokens>,
  'icp_balance' : ActorMethod<[], bigint>,
  'last_poll_time' : ActorMethod<[], bigint>,
  'ops' : ActorMethod<[], undefined>,
}
