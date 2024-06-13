import type { Principal } from "@dfinity/principal";
import type { ActorMethod } from "@dfinity/agent";
import type { IDL } from "@dfinity/candid";

export type Activity = { UserActivity: Principal } | { GroupMemberCount: bigint };
export interface Address {
  street: string;
  country: string;
  city: string;
  postal_code: string;
  label: string;
  state_or_province: string;
  house_number: string;
  house_number_addition: string;
}
export interface ApiError {
  tag: [] | [string];
  info: [] | [Array<string>];
  method_name: [] | [string];
  message: [] | [string];
  timestamp: bigint;
  error_type: ApiErrorType;
}
export type ApiErrorType =
  | { Duplicate: null }
  | { SerializeError: null }
  | { DeserializeError: null }
  | { NotFound: null }
  | { ValidationError: Array<ValidationResponse> }
  | { Unsupported: null }
  | { Unauthorized: null }
  | { Unexpected: null }
  | { NotImplemented: null }
  | { BadRequest: null };
export type ApplicationRole =
  | { Blocked: null }
  | { Guest: null }
  | { Member: null }
  | { Banned: null }
  | { Admin: null }
  | { Moderator: null }
  | { Leader: null }
  | { Owner: null }
  | { Watcher: null };
export type Asset = { Url: string } | { None: null } | { CanisterStorage: CanisterStorage };
export interface Attendee {
  invites: Array<[bigint, AttendeeInvite]>;
  joined: Array<[bigint, AttendeeJoin]>;
}
export interface AttendeeInvite {
  updated_at: bigint;
  invite_type: InviteType;
  created_at: bigint;
  notification_id: [] | [bigint];
  group_id: bigint;
}
export interface AttendeeJoin {
  updated_at: bigint;
  created_at: bigint;
  group_id: bigint;
}
export interface Boost {
  updated_at: bigint;
  subject: Subject;
  owner: Principal;
  seconds: bigint;
  created_at: bigint;
  blockheight: bigint;
  notification_id: [] | [bigint];
}
export interface CanisterOutputCertifiedMessages {
  messages: Array<CanisterOutputMessage>;
  cert: Uint8Array | number[];
  tree: Uint8Array | number[];
  is_end_of_queue: boolean;
}
export interface CanisterOutputMessage {
  key: string;
  content: Uint8Array | number[];
  client_key: ClientKey;
}
export type CanisterStorage = { None: null } | { Manifest: Manifest } | { Chunk: ChunkData };
export interface CanisterWsCloseArguments {
  client_key: ClientKey;
}
export interface CanisterWsGetMessagesArguments {
  nonce: bigint;
}
export interface CanisterWsMessageArguments {
  msg: WebsocketMessage;
}
export interface CanisterWsOpenArguments {
  gateway_principal: Principal;
  client_nonce: bigint;
}
export interface ChunkData {
  chunk_id: bigint;
  canister: Principal;
  index: bigint;
}
export interface ClientKey {
  client_principal: Principal;
  client_nonce: bigint;
}
export interface DateRange {
  end_date: bigint;
  start_date: bigint;
}
export interface DocumentDetails {
  approved_date: bigint;
  approved_version: bigint;
}
export interface Event {
  updated_on: bigint;
  banner_image: Asset;
  owner: Principal;
  metadata: [] | [string];
  date: DateRange;
  name: string;
  tags: Uint32Array | number[];
  description: string;
  created_by: Principal;
  created_on: bigint;
  website: string;
  privacy: Privacy;
  group_id: bigint;
  is_canceled: [boolean, string];
  image: Asset;
  location: Location;
  is_deleted: boolean;
}
export interface EventCallerData {
  is_starred: boolean;
  joined: [] | [JoinedAttendeeResponse];
  invite: [] | [InviteAttendeeResponse];
}
export type EventFilter =
  | { Ids: BigUint64Array | bigint[] }
  | { Tag: number }
  | { UpdatedOn: DateRange }
  | { Name: string }
  | { None: null }
  | { Groups: BigUint64Array | bigint[] }
  | { IsCanceled: boolean }
  | { StartDate: DateRange }
  | { Owner: Principal }
  | { CreatedOn: DateRange }
  | { EndDate: DateRange };
export type EventNotificationType =
  | { UserJoinEvent: [bigint, bigint] }
  | { JoinEventOwnerRequestDecline: InviteAttendeeResponse }
  | { RemoveAttendeeByOwner: JoinedAttendeeResponse }
  | { EventReminder: bigint }
  | { JoinEventUserRequestAccept: InviteAttendeeResponse }
  | { RoleAssignByOwner: JoinedAttendeeResponse }
  | { JoinEventOwnerRequestAccept: InviteAttendeeResponse }
  | { JoinEventOwnerRequest: InviteAttendeeResponse }
  | { JoinEventUserRequest: InviteAttendeeResponse }
  | { JoinEventUserRequestDecline: InviteAttendeeResponse }
  | { UserLeaveEvent: [bigint, bigint] }
  | { RemoveInviteByOwner: InviteAttendeeResponse };
export interface EventResponse {
  id: bigint;
  updated_on: bigint;
  banner_image: Asset;
  owner: Principal;
  metadata: [] | [string];
  date: DateRange;
  attendee_count: bigint;
  name: string;
  tags: Uint32Array | number[];
  description: string;
  created_by: Principal;
  created_on: bigint;
  website: string;
  boosted: [] | [Boost];
  privacy: Privacy;
  group_id: bigint;
  is_canceled: [boolean, string];
  image: Asset;
  caller_data: [] | [EventCallerData];
  location: Location;
  is_deleted: boolean;
}
export type EventSort =
  | { UpdatedOn: SortDirection }
  | { StartDate: SortDirection }
  | { CreatedOn: SortDirection }
  | { EndDate: SortDirection };
export interface EventsCount {
  new: bigint;
  total: bigint;
  starred: bigint;
  invited: bigint;
  past: bigint;
  future: bigint;
  attending: bigint;
}
export interface FriendRequestResponse {
  id: bigint;
  to: Principal;
  created_at: bigint;
  requested_by: Principal;
  message: string;
}
export type GatedType = { Neuron: Array<NeuronGated> } | { Token: Array<TokenGated> };
export interface Group {
  updated_on: bigint;
  banner_image: Asset;
  special_members: Array<[Principal, string]>;
  owner: Principal;
  name: string;
  matrix_space_id: string;
  tags: Uint32Array | number[];
  description: string;
  created_by: Principal;
  created_on: bigint;
  website: string;
  notification_id: [] | [bigint];
  privacy: Privacy;
  wallets: Array<[Principal, string]>;
  image: Asset;
  privacy_gated_type_amount: [] | [bigint];
  location: Location;
  roles: Array<Role>;
  is_deleted: boolean;
}
export interface GroupCallerData {
  is_starred: boolean;
  is_pinned: boolean;
  joined: [] | [JoinedMemberResponse];
  invite: [] | [InviteMemberResponse];
}
export type GroupFilter =
  | { Ids: BigUint64Array | bigint[] }
  | { Tag: number }
  | { UpdatedOn: DateRange }
  | { Name: string }
  | { None: null }
  | { Owner: Principal }
  | { CreatedOn: DateRange };
export type GroupNotificationType =
  | { UserLeaveGroup: bigint }
  | { UserJoinGroup: bigint }
  | { JoinGroupUserRequest: InviteMemberResponse }
  | { JoinGroupUserRequestDecline: InviteMemberResponse }
  | { RoleAssignByOwner: JoinedMemberResponse }
  | { JoinGroupOwnerRequest: InviteMemberResponse }
  | { RemoveMemberByOwner: JoinedMemberResponse }
  | { GroupReminder: bigint }
  | { JoinGroupOwnerRequestDecline: InviteMemberResponse }
  | { JoinGroupUserRequestAccept: InviteMemberResponse }
  | { RemoveInviteByOwner: InviteMemberResponse }
  | { JoinGroupOwnerRequestAccept: InviteMemberResponse };
export interface GroupResponse {
  id: bigint;
  updated_on: bigint;
  banner_image: Asset;
  owner: Principal;
  name: string;
  matrix_space_id: string;
  tags: Uint32Array | number[];
  description: string;
  created_by: Principal;
  created_on: bigint;
  website: string;
  boosted: [] | [Boost];
  privacy: Privacy;
  wallets: Array<[Principal, string]>;
  events_count: bigint;
  image: Asset;
  caller_data: [] | [GroupCallerData];
  members_count: bigint;
  privacy_gated_type_amount: [] | [bigint];
  location: Location;
  roles: Array<Role>;
  is_deleted: boolean;
}
export type GroupSort =
  | { UpdatedOn: SortDirection }
  | { MemberCount: SortDirection }
  | { Name: SortDirection }
  | { CreatedOn: SortDirection };
export interface GroupsCount {
  new: bigint;
  total: bigint;
  starred: bigint;
  invited: bigint;
  joined: bigint;
}
export interface HttpHeader {
  value: string;
  name: string;
}
export interface HttpRequest {
  url: string;
  method: string;
  body: Uint8Array | number[];
  headers: Array<[string, string]>;
}
export interface HttpResponse {
  status: bigint;
  body: Uint8Array | number[];
  headers: Array<HttpHeader>;
}
export interface InviteAttendeeResponse {
  principal: Principal;
  invite_type: InviteType;
  group_id: bigint;
  event_id: bigint;
}
export interface InviteMemberResponse {
  principal: Principal;
  group_id: bigint;
  invite: [] | [MemberInvite];
}
export type InviteType = { OwnerRequest: null } | { UserRequest: null };
export interface Join {
  updated_at: bigint;
  created_at: bigint;
  roles: Array<string>;
}
export interface JoinedAttendeeResponse {
  principal: Principal;
  group_id: bigint;
  event_id: bigint;
}
export interface JoinedMemberResponse {
  principal: Principal;
  group_id: bigint;
  roles: Array<string>;
}
export type Location =
  | { None: null }
  | { Digital: string }
  | { Physical: PhysicalLocation }
  | { MultiLocation: MultiLocation };
export type LogType = { Error: null } | { Info: null } | { Warning: null };
export interface Logger {
  principal: [] | [Principal];
  source: [] | [string];
  data: [] | [string];
  description: string;
  created_on: bigint;
}
export interface Manifest {
  entries: Array<ChunkData>;
}
export interface Member {
  invites: Array<[bigint, MemberInvite]>;
  joined: Array<[bigint, Join]>;
}
export interface MemberInvite {
  updated_at: bigint;
  invite_type: InviteType;
  created_at: bigint;
  notification_id: [] | [bigint];
}
export type MetadataValue = { Int: bigint } | { Nat: bigint } | { Blob: Uint8Array | number[] } | { Text: string };
export interface MultiLocation {
  physical: PhysicalLocation;
  digital: string;
}
export type MultisigNotificationType =
  | {
      ProposalDecline: [Principal, bigint, bigint];
    }
  | { ProposalAccept: [Principal, bigint, bigint] }
  | { WhitelistNotice: [Principal, bigint] }
  | { ProposalStatusUpdate: [Principal, bigint, bigint] }
  | { NewProposal: [Principal, bigint, bigint] };
export interface NeuronGated {
  governance_canister: Principal;
  name: string;
  description: string;
  ledger_canister: Principal;
  rules: Array<NeuronGatedRules>;
}
export type NeuronGatedRules =
  | { IsDisolving: boolean }
  | { MinStake: bigint }
  | { MinAge: bigint }
  | { MinDissolveDelay: bigint };
export interface Notification {
  updated_at: bigint;
  metadata: [] | [string];
  is_accepted: [] | [boolean];
  is_actionable: boolean;
  created_at: bigint;
  sender: Principal;
  notification_type: NotificationType;
  processed_by: [] | [Principal];
}
export interface NotificationResponse {
  id: [] | [bigint];
  user_data: [] | [UserNotificationData];
  notification: Notification;
}
export type NotificationType =
  | { Event: EventNotificationType }
  | { Relation: RelationNotificationType }
  | { Group: GroupNotificationType }
  | { Transaction: TransactionNotificationType }
  | { Multisig: MultisigNotificationType };
export interface PagedResponse {
  total: bigint;
  data: Array<EventResponse>;
  page: bigint;
  limit: bigint;
  number_of_pages: bigint;
}
export interface PagedResponse_1 {
  total: bigint;
  data: Array<GroupResponse>;
  page: bigint;
  limit: bigint;
  number_of_pages: bigint;
}
export interface PagedResponse_2 {
  total: bigint;
  data: Array<ReportResponse>;
  page: bigint;
  limit: bigint;
  number_of_pages: bigint;
}
export interface Permission {
  name: string;
  actions: PermissionActions;
  protected: boolean;
}
export interface PermissionActions {
  edit: boolean;
  read: boolean;
  delete: boolean;
  write: boolean;
}
export interface PhysicalLocation {
  longtitude: number;
  address: Address;
  lattitude: number;
}
export interface PostEvent {
  banner_image: Asset;
  metadata: [] | [string];
  date: DateRange;
  name: string;
  tags: Uint32Array | number[];
  description: string;
  website: string;
  privacy: Privacy;
  group_id: bigint;
  image: Asset;
  location: Location;
}
export interface PostGroup {
  banner_image: Asset;
  name: string;
  matrix_space_id: string;
  tags: Uint32Array | number[];
  description: string;
  website: string;
  privacy: Privacy;
  image: Asset;
  privacy_gated_type_amount: [] | [bigint];
  location: Location;
}
export interface PostLog {
  source: [] | [string];
  data: [] | [string];
  log_type: LogType;
  description: string;
}
export interface PostPermission {
  name: string;
  actions: PermissionActions;
}
export interface PostProfile {
  username: string;
  display_name: string;
  extra: string;
  privacy: ProfilePrivacy;
  first_name: string;
  last_name: string;
}
export interface PostReport {
  subject: Subject;
  group_id: bigint;
  message: string;
}
export interface PostWallet {
  principal: Principal;
  provider: string;
}
export type Privacy = { Gated: GatedType } | { Private: null } | { Public: null } | { InviteOnly: null };
export interface Profile {
  updated_on: bigint;
  profile_image: Asset;
  banner_image: Asset;
  about: string;
  country: string;
  username: string;
  starred: Array<Subject>;
  interests: Uint32Array | number[];
  city: string;
  created_on: bigint;
  email: string;
  website: string;
  terms_of_service: [] | [DocumentDetails];
  display_name: string;
  extra: string;
  privacy_policy: [] | [DocumentDetails];
  notification_id: [] | [bigint];
  pinned: Array<Subject>;
  privacy: ProfilePrivacy;
  wallets: Array<[Principal, Wallet]>;
  state_or_province: string;
  first_name: string;
  last_name: string;
  causes: Uint32Array | number[];
  code_of_conduct: [] | [DocumentDetails];
  date_of_birth: bigint;
  skills: Uint32Array | number[];
  relations: Array<[Principal, string]>;
  application_role: ApplicationRole;
}
export type ProfilePrivacy = { Private: null } | { Public: null };
export interface ProfileResponse {
  updated_on: bigint;
  profile_image: Asset;
  principal: Principal;
  banner_image: Asset;
  about: string;
  country: string;
  username: string;
  starred: Array<Subject>;
  interests: Uint32Array | number[];
  city: string;
  created_on: bigint;
  email: string;
  website: string;
  terms_of_service: [] | [DocumentDetails];
  display_name: string;
  extra: string;
  privacy_policy: [] | [DocumentDetails];
  pinned: Array<Subject>;
  privacy: ProfilePrivacy;
  wallets: Array<WalletResponse>;
  state_or_province: string;
  first_name: string;
  last_name: string;
  causes: Uint32Array | number[];
  code_of_conduct: [] | [DocumentDetails];
  date_of_birth: bigint;
  skills: Uint32Array | number[];
  application_role: ApplicationRole;
}
export type RelationNotificationType =
  | {
      FriendRequest: FriendRequestResponse;
    }
  | { FriendRequestDecline: FriendRequestResponse }
  | { FriendRemove: Principal }
  | { FriendRequestReminder: bigint }
  | { BlockUser: Principal }
  | { FriendRequestRemove: bigint }
  | { FriendRequestAccept: FriendRequestResponse };
export type RelationType = { Blocked: null } | { Friend: null };
export type ReportFilter =
  | { SubjectType: SubjectType }
  | { None: null }
  | { GroupId: bigint }
  | { ReportedBy: Principal }
  | { CreatedOn: DateRange }
  | { Subject: Subject };
export interface ReportResponse {
  id: bigint;
  subject: SubjectResponse;
  created_on: bigint;
  message: string;
  reported_by: Principal;
}
export type ReportSort =
  | { SubjectType: SortDirection }
  | { ReportedBy: SortDirection }
  | { CreatedOn: SortDirection }
  | { Subject: SortDirection };
export type Result = { Ok: Principal } | { Err: string };
export type Result_1 = { Ok: Principal } | { Err: ApiError };
export type Result_10 = { Ok: ReportResponse } | { Err: ApiError };
export type Result_11 = { Ok: Role } | { Err: ApiError };
export type Result_12 = { Ok: Topic } | { Err: ApiError };
export type Result_13 = { Ok: null } | { Err: ApiError };
export type Result_14 = { Ok: bigint } | { Err: ApiError };
export type Result_15 = { Ok: [boolean, boolean, boolean] } | { Err: ApiError };
export type Result_16 = { Ok: Array<Topic> } | { Err: ApiError };
export type Result_17 = { Ok: Array<JoinedAttendeeResponse> } | { Err: ApiError };
export type Result_18 = { Ok: Array<Principal> } | { Err: ApiError };
export type Result_19 = { Ok: Array<[ProfileResponse, Array<string>]> } | { Err: ApiError };
export type Result_2 = { Ok: boolean } | { Err: ApiError };
export type Result_20 = { Ok: Array<InviteAttendeeResponse> } | { Err: ApiError };
export type Result_21 =
  | {
      Ok: Array<[ProfileResponse, InviteAttendeeResponse]>;
    }
  | { Err: ApiError };
export type Result_22 = { Ok: PagedResponse } | { Err: ApiError };
export type Result_23 = { Ok: Array<InviteMemberResponse> } | { Err: ApiError };
export type Result_24 =
  | {
      Ok: Array<[InviteMemberResponse, ProfileResponse]>;
    }
  | { Err: ApiError };
export type Result_25 = { Ok: JoinedMemberResponse } | { Err: ApiError };
export type Result_26 = { Ok: [JoinedMemberResponse, ProfileResponse] } | { Err: ApiError };
export type Result_27 = { Ok: Array<JoinedMemberResponse> } | { Err: ApiError };
export type Result_28 =
  | {
      Ok: Array<[JoinedMemberResponse, ProfileResponse]>;
    }
  | { Err: ApiError };
export type Result_29 = { Ok: Array<Role> } | { Err: ApiError };
export type Result_3 = { Ok: Attendee } | { Err: ApiError };
export type Result_30 = { Ok: PagedResponse_1 } | { Err: ApiError };
export type Result_31 = { Ok: Array<string> } | { Err: ApiError };
export type Result_32 = { Ok: PagedResponse_2 } | { Err: ApiError };
export type Result_33 = { Ok: InviteAttendeeResponse } | { Err: ApiError };
export type Result_34 = { Ok: [bigint, Logger] } | { Err: ApiError };
export type Result_35 = { Ok: Array<[bigint, UserNotificationData]> } | { Err: ApiError };
export type Result_36 = { Ok: null } | { Err: string };
export type Result_37 = { Ok: CanisterOutputCertifiedMessages } | { Err: string };
export type Result_4 = { Ok: Member } | { Err: ApiError };
export type Result_5 = { Ok: JoinedAttendeeResponse } | { Err: ApiError };
export type Result_6 = { Ok: EventResponse } | { Err: ApiError };
export type Result_7 = { Ok: FriendRequestResponse } | { Err: ApiError };
export type Result_8 = { Ok: GroupResponse } | { Err: ApiError };
export type Result_9 = { Ok: ProfileResponse } | { Err: ApiError };
export interface RewardableActivityResponse {
  timestamp: bigint;
  activity: Activity;
}
export interface Role {
  permissions: Array<Permission>;
  name: string;
  color: string;
  protected: boolean;
  index: [] | [bigint];
}
export type SortDirection = { Asc: null } | { Desc: null };
export type Subject =
  | { Event: bigint }
  | { Group: bigint }
  | { Attendee: Principal }
  | { None: null }
  | { Member: Principal }
  | { Profile: Principal };
export type SubjectResponse =
  | { Event: [] | [[bigint, Event]] }
  | { Group: [] | [[bigint, Group]] }
  | { Attendee: [] | [[Principal, Attendee]] }
  | { None: null }
  | { Member: [] | [[Principal, Member]] }
  | { Profile: [] | [[Principal, Profile]] };
export type SubjectType =
  | { Event: null }
  | { Group: null }
  | { Attendee: null }
  | { None: null }
  | { Member: null }
  | { Profile: null };
export interface TokenGated {
  principal: Principal;
  name: string;
  description: string;
  amount: bigint;
  standard: string;
}
export interface Topic {
  id: bigint;
  value: string;
  kind: TopicKind;
}
export type TopicKind = { Tag: null } | { Skill: null } | { Category: null };
export interface TransactionCompleteData {
  metadata: Array<[string, MetadataValue]>;
  sender: Principal;
  total_amount_distributed: bigint;
  canister: Principal;
  receiver_count: bigint;
}
export interface TransactionData {
  fee: bigint;
  metadata: Array<[string, MetadataValue]>;
  memo: [] | [Uint8Array | number[]];
  sender: Principal;
  canister: Principal;
  amount: bigint;
  block_height: bigint;
  receiver: Principal;
}
export type TransactionNotificationType =
  | {
      SingleTransaction: TransactionData;
    }
  | { TransactionsComplete: TransactionCompleteData };
export interface UpdateEvent {
  banner_image: Asset;
  owner: Principal;
  metadata: [] | [string];
  date: DateRange;
  name: string;
  tags: Uint32Array | number[];
  description: string;
  website: string;
  privacy: Privacy;
  image: Asset;
  location: Location;
}
export interface UpdateGroup {
  banner_image: Asset;
  name: string;
  tags: Uint32Array | number[];
  description: string;
  website: string;
  privacy: Privacy;
  image: Asset;
  privacy_gated_type_amount: [] | [bigint];
  location: Location;
}
export interface UpdateProfile {
  profile_image: Asset;
  banner_image: Asset;
  about: string;
  country: string;
  interests: Uint32Array | number[];
  city: string;
  email: [] | [string];
  website: string;
  display_name: string;
  extra: string;
  privacy: ProfilePrivacy;
  state_or_province: string;
  first_name: string;
  last_name: string;
  causes: Uint32Array | number[];
  date_of_birth: bigint;
  skills: Uint32Array | number[];
}
export interface UserNotificationData {
  is_read: boolean;
  is_sender: boolean;
}
export interface ValidationResponse {
  field: string;
  message: string;
}
export type WSMessage =
  | { Error: ApiError }
  | { Notification: NotificationResponse }
  | { SendNotification: [Principal, NotificationResponse] }
  | { UnreadCount: bigint };
export interface Wallet {
  provider: string;
  is_primary: boolean;
}
export interface WalletResponse {
  principal: Principal;
  provider: string;
  is_primary: boolean;
}
export interface WebsocketMessage {
  sequence_num: bigint;
  content: Uint8Array | number[];
  client_key: ClientKey;
  timestamp: bigint;
  is_service_message: boolean;
}
export interface _SERVICE {
  __get_candid_interface_tmp_hack: ActorMethod<[], string>;
  _dev_check_attendees_sync: ActorMethod<[Principal, bigint], [[string, boolean], [string, boolean]]>;
  _dev_check_events_sync: ActorMethod<[bigint, bigint], [[string, boolean], [string, boolean]]>;
  _dev_check_member_sync: ActorMethod<[Principal, bigint], [[string, boolean], [string, boolean]]>;
  _dev_create_canister: ActorMethod<[Array<Principal>], Result>;
  _dev_get_history_canister: ActorMethod<[], Result_1>;
  _dev_get_reward_canister: ActorMethod<[], Result_1>;
  _dev_send_reward_data: ActorMethod<[], undefined>;
  _dev_set_history_canister: ActorMethod<[Principal], Result_1>;
  _dev_set_reward_canister: ActorMethod<[Principal], Result_1>;
  _expose: ActorMethod<[], [] | [WSMessage]>;
  accept_friend_request: ActorMethod<[bigint], Result_2>;
  accept_owner_request_event_invite: ActorMethod<[bigint], Result_3>;
  accept_owner_request_group_invite: ActorMethod<[bigint], Result_4>;
  accept_user_request_event_invite: ActorMethod<[bigint, bigint, Principal], Result_5>;
  accept_user_request_group_invite: ActorMethod<[bigint, Principal], Result_4>;
  add_event: ActorMethod<[PostEvent], Result_6>;
  add_friend_request: ActorMethod<[Principal, string], Result_7>;
  add_group: ActorMethod<[PostGroup, [] | [string]], Result_8>;
  add_pinned: ActorMethod<[Subject], Result_9>;
  add_profile: ActorMethod<[PostProfile], Result_9>;
  add_report: ActorMethod<[PostReport], Result_10>;
  add_role_to_group: ActorMethod<[bigint, string, string, bigint], Result_11>;
  add_starred: ActorMethod<[Subject], Result_9>;
  add_topic: ActorMethod<[TopicKind, string], Result_12>;
  add_topics: ActorMethod<[TopicKind, Array<string>], Array<Result_12>>;
  add_transaction_notification: ActorMethod<[TransactionData], boolean>;
  add_transactions_complete_notification: ActorMethod<[TransactionCompleteData], boolean>;
  add_wallet_to_group: ActorMethod<[bigint, Principal, string], Result_8>;
  add_wallet_to_profile: ActorMethod<[PostWallet], Result_9>;
  approve_code_of_conduct: ActorMethod<[bigint], Result_2>;
  approve_privacy_policy: ActorMethod<[bigint], Result_2>;
  approve_terms_of_service: ActorMethod<[bigint], Result_2>;
  assign_role: ActorMethod<[bigint, string, Principal], Result_4>;
  ban_group_member: ActorMethod<[bigint, Principal], Result_13>;
  block_user: ActorMethod<[Principal], Result_9>;
  boost: ActorMethod<[Subject, bigint], Result_14>;
  cancel_event: ActorMethod<[bigint, bigint, string], Result_13>;
  check_new_stores: ActorMethod<[], Array<string>>;
  decline_friend_request: ActorMethod<[bigint], Result_2>;
  decline_owner_request_event_invite: ActorMethod<[bigint], Result_3>;
  decline_owner_request_group_invite: ActorMethod<[bigint], Result_4>;
  decline_user_request_event_invite: ActorMethod<[bigint, bigint, Principal], Result_5>;
  decline_user_request_group_invite: ActorMethod<[bigint, Principal], Result_4>;
  delete_event: ActorMethod<[bigint, bigint], Result_13>;
  delete_group: ActorMethod<[bigint], Result_15>;
  edit_event: ActorMethod<[bigint, bigint, UpdateEvent], Result_6>;
  edit_group: ActorMethod<[bigint, UpdateGroup], Result_8>;
  edit_profile: ActorMethod<[UpdateProfile], Result_9>;
  edit_role_permissions: ActorMethod<[bigint, string, Array<PostPermission>], Result_2>;
  fill_buffer: ActorMethod<[], undefined>;
  get_all_topics: ActorMethod<[TopicKind], Result_16>;
  get_attending_from_principal: ActorMethod<[Principal], Result_17>;
  get_banned_group_members: ActorMethod<[bigint], Result_18>;
  get_boosted_events: ActorMethod<[], Array<EventResponse>>;
  get_boosted_groups: ActorMethod<[], Array<GroupResponse>>;
  get_connected_clients: ActorMethod<[], Array<Principal>>;
  get_e8s_per_day_boost_cost: ActorMethod<[], bigint>;
  get_event: ActorMethod<[bigint], Result_6>;
  get_event_attendees: ActorMethod<[bigint], Result_17>;
  get_event_attendees_profiles_and_roles: ActorMethod<[bigint], Result_19>;
  get_event_count: ActorMethod<[[] | [BigUint64Array | bigint[]], [] | [string]], EventsCount>;
  get_event_invites: ActorMethod<[bigint, bigint], Result_20>;
  get_event_invites_with_profiles: ActorMethod<[bigint], Result_21>;
  get_events: ActorMethod<[bigint, bigint, EventSort, Array<EventFilter>], Result_22>;
  get_group: ActorMethod<[bigint], Result_8>;
  get_group_by_name: ActorMethod<[string], Result_8>;
  get_group_invites: ActorMethod<[bigint], Result_23>;
  get_group_invites_with_profiles: ActorMethod<[bigint], Result_24>;
  get_group_member: ActorMethod<[bigint, Principal], Result_25>;
  get_group_member_with_profile: ActorMethod<[bigint, Principal], Result_26>;
  get_group_members: ActorMethod<[bigint], Result_27>;
  get_group_members_with_profiles: ActorMethod<[bigint], Result_28>;
  get_group_roles: ActorMethod<[bigint], Result_29>;
  get_groups: ActorMethod<[bigint, bigint, Array<GroupFilter>, GroupSort], Result_30>;
  get_groups_by_id: ActorMethod<[BigUint64Array | bigint[]], Array<GroupResponse>>;
  get_groups_count: ActorMethod<[[] | [string]], GroupsCount>;
  get_groups_for_members: ActorMethod<[Array<Principal>], Array<JoinedMemberResponse>>;
  get_history_point: ActorMethod<[], Result_14>;
  get_incoming_friend_requests: ActorMethod<[], Array<FriendRequestResponse>>;
  get_incoming_friend_requests_with_profile: ActorMethod<[], Array<[FriendRequestResponse, ProfileResponse]>>;
  get_latest_logs: ActorMethod<[bigint], Array<Logger>>;
  get_member_roles: ActorMethod<[bigint, Principal], Result_31>;
  get_notifications: ActorMethod<[], Array<NotificationResponse>>;
  get_outgoing_friend_requests: ActorMethod<[], Array<FriendRequestResponse>>;
  get_outgoing_friend_requests_with_profile: ActorMethod<[], Array<[FriendRequestResponse, ProfileResponse]>>;
  get_pinned_by_subject_type: ActorMethod<[SubjectType], Array<SubjectResponse>>;
  get_profile: ActorMethod<[Principal], Result_9>;
  get_profiles: ActorMethod<[Array<Principal>], Array<ProfileResponse>>;
  get_relations: ActorMethod<[RelationType], Array<Principal>>;
  get_relations_count: ActorMethod<[RelationType], bigint>;
  get_relations_with_profiles: ActorMethod<[RelationType], Array<ProfileResponse>>;
  get_remaining_boost_time_in_seconds: ActorMethod<[Subject], Result_14>;
  get_report: ActorMethod<[bigint, bigint], Result_10>;
  get_reports: ActorMethod<[bigint, bigint, ReportSort, Array<ReportFilter>, bigint], Result_32>;
  get_self_attendee: ActorMethod<[], Result_3>;
  get_self_events: ActorMethod<[], Array<EventResponse>>;
  get_self_groups: ActorMethod<[], Array<GroupResponse>>;
  get_self_member: ActorMethod<[], Result_4>;
  get_starred_by_subject_type: ActorMethod<[SubjectType], BigUint64Array | bigint[]>;
  get_topic: ActorMethod<[TopicKind, bigint], Result_12>;
  get_topics: ActorMethod<[TopicKind, BigUint64Array | bigint[]], Result_16>;
  get_unread_notifications: ActorMethod<[], Array<NotificationResponse>>;
  http_request: ActorMethod<[HttpRequest], HttpResponse>;
  invite_to_event: ActorMethod<[bigint, bigint, Principal], Result_33>;
  invite_to_group: ActorMethod<[bigint, Principal], Result_4>;
  join_event: ActorMethod<[bigint], Result_5>;
  join_group: ActorMethod<[bigint, [] | [string]], Result_25>;
  leave_event: ActorMethod<[bigint], Result_13>;
  leave_group: ActorMethod<[bigint], Result_13>;
  log: ActorMethod<[PostLog], Result_34>;
  log_login: ActorMethod<[], Result_34>;
  log_size: ActorMethod<[], bigint>;
  log_with_caller: ActorMethod<[PostLog], Result_34>;
  mark_notifications_as_read: ActorMethod<[BigUint64Array | bigint[], boolean], Result_35>;
  migrate: ActorMethod<[], Array<Array<string>>>;
  multisig_new_proposal_notification: ActorMethod<[Array<Principal>, Principal, bigint, bigint], boolean>;
  multisig_proposal_accept_notification: ActorMethod<[Array<Principal>, Principal, bigint, bigint], boolean>;
  multisig_proposal_decline_notification: ActorMethod<[Array<Principal>, Principal, bigint, bigint], boolean>;
  multisig_proposal_status_update_notification: ActorMethod<[Array<Principal>, Principal, bigint, bigint], boolean>;
  multisig_whitelist_notice_notification: ActorMethod<[Array<Principal>, Principal, bigint], boolean>;
  read_reward_buffer: ActorMethod<[], Array<RewardableActivityResponse>>;
  remove_all_notifications: ActorMethod<[], Array<[bigint, UserNotificationData]>>;
  remove_attendee_from_event: ActorMethod<[bigint, bigint, Principal], Result_13>;
  remove_attendee_invite_from_event: ActorMethod<[bigint, bigint, Principal], Result_13>;
  remove_ban_from_group_member: ActorMethod<[bigint, Principal], Result_13>;
  remove_event_invite: ActorMethod<[bigint], Result_13>;
  remove_friend: ActorMethod<[Principal], Result_9>;
  remove_friend_request: ActorMethod<[bigint], Result_2>;
  remove_group_role: ActorMethod<[bigint, string], Result_2>;
  remove_invite: ActorMethod<[bigint], Result_13>;
  remove_member_from_group: ActorMethod<[bigint, Principal], Result_13>;
  remove_member_invite_from_group: ActorMethod<[bigint, Principal], Result_13>;
  remove_member_role: ActorMethod<[bigint, string, Principal], Result_4>;
  remove_notifications: ActorMethod<[BigUint64Array | bigint[]], Array<[bigint, UserNotificationData]>>;
  remove_pinned: ActorMethod<[Subject], Result_9>;
  remove_starred: ActorMethod<[Subject], Result_9>;
  remove_topic: ActorMethod<[TopicKind, bigint], boolean>;
  remove_wallet_from_group: ActorMethod<[bigint, Principal], Result_8>;
  remove_wallet_from_profile: ActorMethod<[Principal], Result_9>;
  reward_timer_next_trigger: ActorMethod<[], [] | [bigint]>;
  set_wallet_as_primary: ActorMethod<[Principal], Result_9>;
  store_stats: ActorMethod<[], Array<string>>;
  test_log: ActorMethod<[], undefined>;
  unblock_user: ActorMethod<[Principal], Result_9>;
  ws_close: ActorMethod<[CanisterWsCloseArguments], Result_36>;
  ws_get_messages: ActorMethod<[CanisterWsGetMessagesArguments], Result_37>;
  ws_message: ActorMethod<[CanisterWsMessageArguments, [] | [WSMessage]], Result_36>;
  ws_open: ActorMethod<[CanisterWsOpenArguments], Result_36>;
}

export const idlFactory = ({ IDL }: any) => {
  const Result = IDL.Variant({ Ok: IDL.Principal, Err: IDL.Text });
  const ValidationResponse = IDL.Record({
    field: IDL.Text,
    message: IDL.Text,
  });
  const ApiErrorType = IDL.Variant({
    Duplicate: IDL.Null,
    SerializeError: IDL.Null,
    DeserializeError: IDL.Null,
    NotFound: IDL.Null,
    ValidationError: IDL.Vec(ValidationResponse),
    Unsupported: IDL.Null,
    Unauthorized: IDL.Null,
    Unexpected: IDL.Null,
    NotImplemented: IDL.Null,
    BadRequest: IDL.Null,
  });
  const ApiError = IDL.Record({
    tag: IDL.Opt(IDL.Text),
    info: IDL.Opt(IDL.Vec(IDL.Text)),
    method_name: IDL.Opt(IDL.Text),
    message: IDL.Opt(IDL.Text),
    timestamp: IDL.Nat64,
    error_type: ApiErrorType,
  });
  const Result_1 = IDL.Variant({ Ok: IDL.Principal, Err: ApiError });
  const UserNotificationData = IDL.Record({
    is_read: IDL.Bool,
    is_sender: IDL.Bool,
  });
  const InviteType = IDL.Variant({
    OwnerRequest: IDL.Null,
    UserRequest: IDL.Null,
  });
  const InviteAttendeeResponse = IDL.Record({
    principal: IDL.Principal,
    invite_type: InviteType,
    group_id: IDL.Nat64,
    event_id: IDL.Nat64,
  });
  const JoinedAttendeeResponse = IDL.Record({
    principal: IDL.Principal,
    group_id: IDL.Nat64,
    event_id: IDL.Nat64,
  });
  const EventNotificationType = IDL.Variant({
    UserJoinEvent: IDL.Tuple(IDL.Nat64, IDL.Nat64),
    JoinEventOwnerRequestDecline: InviteAttendeeResponse,
    RemoveAttendeeByOwner: JoinedAttendeeResponse,
    EventReminder: IDL.Nat64,
    JoinEventUserRequestAccept: InviteAttendeeResponse,
    RoleAssignByOwner: JoinedAttendeeResponse,
    JoinEventOwnerRequestAccept: InviteAttendeeResponse,
    JoinEventOwnerRequest: InviteAttendeeResponse,
    JoinEventUserRequest: InviteAttendeeResponse,
    JoinEventUserRequestDecline: InviteAttendeeResponse,
    UserLeaveEvent: IDL.Tuple(IDL.Nat64, IDL.Nat64),
    RemoveInviteByOwner: InviteAttendeeResponse,
  });
  const FriendRequestResponse = IDL.Record({
    id: IDL.Nat64,
    to: IDL.Principal,
    created_at: IDL.Nat64,
    requested_by: IDL.Principal,
    message: IDL.Text,
  });
  const RelationNotificationType = IDL.Variant({
    FriendRequest: FriendRequestResponse,
    FriendRequestDecline: FriendRequestResponse,
    FriendRemove: IDL.Principal,
    FriendRequestReminder: IDL.Nat64,
    BlockUser: IDL.Principal,
    FriendRequestRemove: IDL.Nat64,
    FriendRequestAccept: FriendRequestResponse,
  });
  const MemberInvite = IDL.Record({
    updated_at: IDL.Nat64,
    invite_type: InviteType,
    created_at: IDL.Nat64,
    notification_id: IDL.Opt(IDL.Nat64),
  });
  const InviteMemberResponse = IDL.Record({
    principal: IDL.Principal,
    group_id: IDL.Nat64,
    invite: IDL.Opt(MemberInvite),
  });
  const JoinedMemberResponse = IDL.Record({
    principal: IDL.Principal,
    group_id: IDL.Nat64,
    roles: IDL.Vec(IDL.Text),
  });
  const GroupNotificationType = IDL.Variant({
    UserLeaveGroup: IDL.Nat64,
    UserJoinGroup: IDL.Nat64,
    JoinGroupUserRequest: InviteMemberResponse,
    JoinGroupUserRequestDecline: InviteMemberResponse,
    RoleAssignByOwner: JoinedMemberResponse,
    JoinGroupOwnerRequest: InviteMemberResponse,
    RemoveMemberByOwner: JoinedMemberResponse,
    GroupReminder: IDL.Nat64,
    JoinGroupOwnerRequestDecline: InviteMemberResponse,
    JoinGroupUserRequestAccept: InviteMemberResponse,
    RemoveInviteByOwner: InviteMemberResponse,
    JoinGroupOwnerRequestAccept: InviteMemberResponse,
  });
  const MetadataValue = IDL.Variant({
    Int: IDL.Int,
    Nat: IDL.Nat,
    Blob: IDL.Vec(IDL.Nat8),
    Text: IDL.Text,
  });
  const TransactionData = IDL.Record({
    fee: IDL.Nat,
    metadata: IDL.Vec(IDL.Tuple(IDL.Text, MetadataValue)),
    memo: IDL.Opt(IDL.Vec(IDL.Nat8)),
    sender: IDL.Principal,
    canister: IDL.Principal,
    amount: IDL.Nat,
    block_height: IDL.Nat,
    receiver: IDL.Principal,
  });
  const TransactionCompleteData = IDL.Record({
    metadata: IDL.Vec(IDL.Tuple(IDL.Text, MetadataValue)),
    sender: IDL.Principal,
    total_amount_distributed: IDL.Nat,
    canister: IDL.Principal,
    receiver_count: IDL.Nat64,
  });
  const TransactionNotificationType = IDL.Variant({
    SingleTransaction: TransactionData,
    TransactionsComplete: TransactionCompleteData,
  });
  const MultisigNotificationType = IDL.Variant({
    ProposalDecline: IDL.Tuple(IDL.Principal, IDL.Nat64, IDL.Nat64),
    ProposalAccept: IDL.Tuple(IDL.Principal, IDL.Nat64, IDL.Nat64),
    WhitelistNotice: IDL.Tuple(IDL.Principal, IDL.Nat64),
    ProposalStatusUpdate: IDL.Tuple(IDL.Principal, IDL.Nat64, IDL.Nat64),
    NewProposal: IDL.Tuple(IDL.Principal, IDL.Nat64, IDL.Nat64),
  });
  const NotificationType = IDL.Variant({
    Event: EventNotificationType,
    Relation: RelationNotificationType,
    Group: GroupNotificationType,
    Transaction: TransactionNotificationType,
    Multisig: MultisigNotificationType,
  });
  const Notification = IDL.Record({
    updated_at: IDL.Nat64,
    metadata: IDL.Opt(IDL.Text),
    is_accepted: IDL.Opt(IDL.Bool),
    is_actionable: IDL.Bool,
    created_at: IDL.Nat64,
    sender: IDL.Principal,
    notification_type: NotificationType,
    processed_by: IDL.Opt(IDL.Principal),
  });
  const NotificationResponse = IDL.Record({
    id: IDL.Opt(IDL.Nat64),
    user_data: IDL.Opt(UserNotificationData),
    notification: Notification,
  });
  const WSMessage = IDL.Variant({
    Error: ApiError,
    Notification: NotificationResponse,
    SendNotification: IDL.Tuple(IDL.Principal, NotificationResponse),
    UnreadCount: IDL.Nat64,
  });
  const Result_2 = IDL.Variant({ Ok: IDL.Bool, Err: ApiError });
  const AttendeeInvite = IDL.Record({
    updated_at: IDL.Nat64,
    invite_type: InviteType,
    created_at: IDL.Nat64,
    notification_id: IDL.Opt(IDL.Nat64),
    group_id: IDL.Nat64,
  });
  const AttendeeJoin = IDL.Record({
    updated_at: IDL.Nat64,
    created_at: IDL.Nat64,
    group_id: IDL.Nat64,
  });
  const Attendee = IDL.Record({
    invites: IDL.Vec(IDL.Tuple(IDL.Nat64, AttendeeInvite)),
    joined: IDL.Vec(IDL.Tuple(IDL.Nat64, AttendeeJoin)),
  });
  const Result_3 = IDL.Variant({ Ok: Attendee, Err: ApiError });
  const Join = IDL.Record({
    updated_at: IDL.Nat64,
    created_at: IDL.Nat64,
    roles: IDL.Vec(IDL.Text),
  });
  const Member = IDL.Record({
    invites: IDL.Vec(IDL.Tuple(IDL.Nat64, MemberInvite)),
    joined: IDL.Vec(IDL.Tuple(IDL.Nat64, Join)),
  });
  const Result_4 = IDL.Variant({ Ok: Member, Err: ApiError });
  const Result_5 = IDL.Variant({
    Ok: JoinedAttendeeResponse,
    Err: ApiError,
  });
  const ChunkData = IDL.Record({
    chunk_id: IDL.Nat64,
    canister: IDL.Principal,
    index: IDL.Nat64,
  });
  const Manifest = IDL.Record({ entries: IDL.Vec(ChunkData) });
  const CanisterStorage = IDL.Variant({
    None: IDL.Null,
    Manifest: Manifest,
    Chunk: ChunkData,
  });
  const Asset = IDL.Variant({
    Url: IDL.Text,
    None: IDL.Null,
    CanisterStorage: CanisterStorage,
  });
  const DateRange = IDL.Record({
    end_date: IDL.Nat64,
    start_date: IDL.Nat64,
  });
  const NeuronGatedRules = IDL.Variant({
    IsDisolving: IDL.Bool,
    MinStake: IDL.Nat64,
    MinAge: IDL.Nat64,
    MinDissolveDelay: IDL.Nat64,
  });
  const NeuronGated = IDL.Record({
    governance_canister: IDL.Principal,
    name: IDL.Text,
    description: IDL.Text,
    ledger_canister: IDL.Principal,
    rules: IDL.Vec(NeuronGatedRules),
  });
  const TokenGated = IDL.Record({
    principal: IDL.Principal,
    name: IDL.Text,
    description: IDL.Text,
    amount: IDL.Nat64,
    standard: IDL.Text,
  });
  const GatedType = IDL.Variant({
    Neuron: IDL.Vec(NeuronGated),
    Token: IDL.Vec(TokenGated),
  });
  const Privacy = IDL.Variant({
    Gated: GatedType,
    Private: IDL.Null,
    Public: IDL.Null,
    InviteOnly: IDL.Null,
  });
  const Address = IDL.Record({
    street: IDL.Text,
    country: IDL.Text,
    city: IDL.Text,
    postal_code: IDL.Text,
    label: IDL.Text,
    state_or_province: IDL.Text,
    house_number: IDL.Text,
    house_number_addition: IDL.Text,
  });
  const PhysicalLocation = IDL.Record({
    longtitude: IDL.Float32,
    address: Address,
    lattitude: IDL.Float32,
  });
  const MultiLocation = IDL.Record({
    physical: PhysicalLocation,
    digital: IDL.Text,
  });
  const Location = IDL.Variant({
    None: IDL.Null,
    Digital: IDL.Text,
    Physical: PhysicalLocation,
    MultiLocation: MultiLocation,
  });
  const PostEvent = IDL.Record({
    banner_image: Asset,
    metadata: IDL.Opt(IDL.Text),
    date: DateRange,
    name: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    website: IDL.Text,
    privacy: Privacy,
    group_id: IDL.Nat64,
    image: Asset,
    location: Location,
  });
  const Subject = IDL.Variant({
    Event: IDL.Nat64,
    Group: IDL.Nat64,
    Attendee: IDL.Principal,
    None: IDL.Null,
    Member: IDL.Principal,
    Profile: IDL.Principal,
  });
  const Boost = IDL.Record({
    updated_at: IDL.Nat64,
    subject: Subject,
    owner: IDL.Principal,
    seconds: IDL.Nat64,
    created_at: IDL.Nat64,
    blockheight: IDL.Nat64,
    notification_id: IDL.Opt(IDL.Nat64),
  });
  const EventCallerData = IDL.Record({
    is_starred: IDL.Bool,
    joined: IDL.Opt(JoinedAttendeeResponse),
    invite: IDL.Opt(InviteAttendeeResponse),
  });
  const EventResponse = IDL.Record({
    id: IDL.Nat64,
    updated_on: IDL.Nat64,
    banner_image: Asset,
    owner: IDL.Principal,
    metadata: IDL.Opt(IDL.Text),
    date: DateRange,
    attendee_count: IDL.Nat64,
    name: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    created_by: IDL.Principal,
    created_on: IDL.Nat64,
    website: IDL.Text,
    boosted: IDL.Opt(Boost),
    privacy: Privacy,
    group_id: IDL.Nat64,
    is_canceled: IDL.Tuple(IDL.Bool, IDL.Text),
    image: Asset,
    caller_data: IDL.Opt(EventCallerData),
    location: Location,
    is_deleted: IDL.Bool,
  });
  const Result_6 = IDL.Variant({ Ok: EventResponse, Err: ApiError });
  const Result_7 = IDL.Variant({
    Ok: FriendRequestResponse,
    Err: ApiError,
  });
  const PostGroup = IDL.Record({
    banner_image: Asset,
    name: IDL.Text,
    matrix_space_id: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    website: IDL.Text,
    privacy: Privacy,
    image: Asset,
    privacy_gated_type_amount: IDL.Opt(IDL.Nat64),
    location: Location,
  });
  const GroupCallerData = IDL.Record({
    is_starred: IDL.Bool,
    is_pinned: IDL.Bool,
    joined: IDL.Opt(JoinedMemberResponse),
    invite: IDL.Opt(InviteMemberResponse),
  });
  const PermissionActions = IDL.Record({
    edit: IDL.Bool,
    read: IDL.Bool,
    delete: IDL.Bool,
    write: IDL.Bool,
  });
  const Permission = IDL.Record({
    name: IDL.Text,
    actions: PermissionActions,
    protected: IDL.Bool,
  });
  const Role = IDL.Record({
    permissions: IDL.Vec(Permission),
    name: IDL.Text,
    color: IDL.Text,
    protected: IDL.Bool,
    index: IDL.Opt(IDL.Nat64),
  });
  const GroupResponse = IDL.Record({
    id: IDL.Nat64,
    updated_on: IDL.Nat64,
    banner_image: Asset,
    owner: IDL.Principal,
    name: IDL.Text,
    matrix_space_id: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    created_by: IDL.Principal,
    created_on: IDL.Nat64,
    website: IDL.Text,
    boosted: IDL.Opt(Boost),
    privacy: Privacy,
    wallets: IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text)),
    events_count: IDL.Nat64,
    image: Asset,
    caller_data: IDL.Opt(GroupCallerData),
    members_count: IDL.Nat64,
    privacy_gated_type_amount: IDL.Opt(IDL.Nat64),
    location: Location,
    roles: IDL.Vec(Role),
    is_deleted: IDL.Bool,
  });
  const Result_8 = IDL.Variant({ Ok: GroupResponse, Err: ApiError });
  const DocumentDetails = IDL.Record({
    approved_date: IDL.Nat64,
    approved_version: IDL.Nat64,
  });
  const ProfilePrivacy = IDL.Variant({
    Private: IDL.Null,
    Public: IDL.Null,
  });
  const WalletResponse = IDL.Record({
    principal: IDL.Principal,
    provider: IDL.Text,
    is_primary: IDL.Bool,
  });
  const ApplicationRole = IDL.Variant({
    Blocked: IDL.Null,
    Guest: IDL.Null,
    Member: IDL.Null,
    Banned: IDL.Null,
    Admin: IDL.Null,
    Moderator: IDL.Null,
    Leader: IDL.Null,
    Owner: IDL.Null,
    Watcher: IDL.Null,
  });
  const ProfileResponse = IDL.Record({
    updated_on: IDL.Nat64,
    profile_image: Asset,
    principal: IDL.Principal,
    banner_image: Asset,
    about: IDL.Text,
    country: IDL.Text,
    username: IDL.Text,
    starred: IDL.Vec(Subject),
    interests: IDL.Vec(IDL.Nat32),
    city: IDL.Text,
    created_on: IDL.Nat64,
    email: IDL.Text,
    website: IDL.Text,
    terms_of_service: IDL.Opt(DocumentDetails),
    display_name: IDL.Text,
    extra: IDL.Text,
    privacy_policy: IDL.Opt(DocumentDetails),
    pinned: IDL.Vec(Subject),
    privacy: ProfilePrivacy,
    wallets: IDL.Vec(WalletResponse),
    state_or_province: IDL.Text,
    first_name: IDL.Text,
    last_name: IDL.Text,
    causes: IDL.Vec(IDL.Nat32),
    code_of_conduct: IDL.Opt(DocumentDetails),
    date_of_birth: IDL.Nat64,
    skills: IDL.Vec(IDL.Nat32),
    application_role: ApplicationRole,
  });
  const Result_9 = IDL.Variant({ Ok: ProfileResponse, Err: ApiError });
  const PostProfile = IDL.Record({
    username: IDL.Text,
    display_name: IDL.Text,
    extra: IDL.Text,
    privacy: ProfilePrivacy,
    first_name: IDL.Text,
    last_name: IDL.Text,
  });
  const PostReport = IDL.Record({
    subject: Subject,
    group_id: IDL.Nat64,
    message: IDL.Text,
  });
  const Event = IDL.Record({
    updated_on: IDL.Nat64,
    banner_image: Asset,
    owner: IDL.Principal,
    metadata: IDL.Opt(IDL.Text),
    date: DateRange,
    name: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    created_by: IDL.Principal,
    created_on: IDL.Nat64,
    website: IDL.Text,
    privacy: Privacy,
    group_id: IDL.Nat64,
    is_canceled: IDL.Tuple(IDL.Bool, IDL.Text),
    image: Asset,
    location: Location,
    is_deleted: IDL.Bool,
  });
  const Group = IDL.Record({
    updated_on: IDL.Nat64,
    banner_image: Asset,
    special_members: IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text)),
    owner: IDL.Principal,
    name: IDL.Text,
    matrix_space_id: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    created_by: IDL.Principal,
    created_on: IDL.Nat64,
    website: IDL.Text,
    notification_id: IDL.Opt(IDL.Nat64),
    privacy: Privacy,
    wallets: IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text)),
    image: Asset,
    privacy_gated_type_amount: IDL.Opt(IDL.Nat64),
    location: Location,
    roles: IDL.Vec(Role),
    is_deleted: IDL.Bool,
  });
  const Wallet = IDL.Record({ provider: IDL.Text, is_primary: IDL.Bool });
  const Profile = IDL.Record({
    updated_on: IDL.Nat64,
    profile_image: Asset,
    banner_image: Asset,
    about: IDL.Text,
    country: IDL.Text,
    username: IDL.Text,
    starred: IDL.Vec(Subject),
    interests: IDL.Vec(IDL.Nat32),
    city: IDL.Text,
    created_on: IDL.Nat64,
    email: IDL.Text,
    website: IDL.Text,
    terms_of_service: IDL.Opt(DocumentDetails),
    display_name: IDL.Text,
    extra: IDL.Text,
    privacy_policy: IDL.Opt(DocumentDetails),
    notification_id: IDL.Opt(IDL.Nat64),
    pinned: IDL.Vec(Subject),
    privacy: ProfilePrivacy,
    wallets: IDL.Vec(IDL.Tuple(IDL.Principal, Wallet)),
    state_or_province: IDL.Text,
    first_name: IDL.Text,
    last_name: IDL.Text,
    causes: IDL.Vec(IDL.Nat32),
    code_of_conduct: IDL.Opt(DocumentDetails),
    date_of_birth: IDL.Nat64,
    skills: IDL.Vec(IDL.Nat32),
    relations: IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text)),
    application_role: ApplicationRole,
  });
  const SubjectResponse = IDL.Variant({
    Event: IDL.Opt(IDL.Tuple(IDL.Nat64, Event)),
    Group: IDL.Opt(IDL.Tuple(IDL.Nat64, Group)),
    Attendee: IDL.Opt(IDL.Tuple(IDL.Principal, Attendee)),
    None: IDL.Null,
    Member: IDL.Opt(IDL.Tuple(IDL.Principal, Member)),
    Profile: IDL.Opt(IDL.Tuple(IDL.Principal, Profile)),
  });
  const ReportResponse = IDL.Record({
    id: IDL.Nat64,
    subject: SubjectResponse,
    created_on: IDL.Nat64,
    message: IDL.Text,
    reported_by: IDL.Principal,
  });
  const Result_10 = IDL.Variant({ Ok: ReportResponse, Err: ApiError });
  const Result_11 = IDL.Variant({ Ok: Role, Err: ApiError });
  const TopicKind = IDL.Variant({
    Tag: IDL.Null,
    Skill: IDL.Null,
    Category: IDL.Null,
  });
  const Topic = IDL.Record({
    id: IDL.Nat64,
    value: IDL.Text,
    kind: TopicKind,
  });
  const Result_12 = IDL.Variant({ Ok: Topic, Err: ApiError });
  const PostWallet = IDL.Record({
    principal: IDL.Principal,
    provider: IDL.Text,
  });
  const Result_13 = IDL.Variant({ Ok: IDL.Null, Err: ApiError });
  const Result_14 = IDL.Variant({ Ok: IDL.Nat64, Err: ApiError });
  const Result_15 = IDL.Variant({
    Ok: IDL.Tuple(IDL.Bool, IDL.Bool, IDL.Bool),
    Err: ApiError,
  });
  const UpdateEvent = IDL.Record({
    banner_image: Asset,
    owner: IDL.Principal,
    metadata: IDL.Opt(IDL.Text),
    date: DateRange,
    name: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    website: IDL.Text,
    privacy: Privacy,
    image: Asset,
    location: Location,
  });
  const UpdateGroup = IDL.Record({
    banner_image: Asset,
    name: IDL.Text,
    tags: IDL.Vec(IDL.Nat32),
    description: IDL.Text,
    website: IDL.Text,
    privacy: Privacy,
    image: Asset,
    privacy_gated_type_amount: IDL.Opt(IDL.Nat64),
    location: Location,
  });
  const UpdateProfile = IDL.Record({
    profile_image: Asset,
    banner_image: Asset,
    about: IDL.Text,
    country: IDL.Text,
    interests: IDL.Vec(IDL.Nat32),
    city: IDL.Text,
    email: IDL.Opt(IDL.Text),
    website: IDL.Text,
    display_name: IDL.Text,
    extra: IDL.Text,
    privacy: ProfilePrivacy,
    state_or_province: IDL.Text,
    first_name: IDL.Text,
    last_name: IDL.Text,
    causes: IDL.Vec(IDL.Nat32),
    date_of_birth: IDL.Nat64,
    skills: IDL.Vec(IDL.Nat32),
  });
  const PostPermission = IDL.Record({
    name: IDL.Text,
    actions: PermissionActions,
  });
  const Result_16 = IDL.Variant({ Ok: IDL.Vec(Topic), Err: ApiError });
  const Result_17 = IDL.Variant({
    Ok: IDL.Vec(JoinedAttendeeResponse),
    Err: ApiError,
  });
  const Result_18 = IDL.Variant({
    Ok: IDL.Vec(IDL.Principal),
    Err: ApiError,
  });
  const Result_19 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(ProfileResponse, IDL.Vec(IDL.Text))),
    Err: ApiError,
  });
  const EventsCount = IDL.Record({
    new: IDL.Nat64,
    total: IDL.Nat64,
    starred: IDL.Nat64,
    invited: IDL.Nat64,
    past: IDL.Nat64,
    future: IDL.Nat64,
    attending: IDL.Nat64,
  });
  const Result_20 = IDL.Variant({
    Ok: IDL.Vec(InviteAttendeeResponse),
    Err: ApiError,
  });
  const Result_21 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(ProfileResponse, InviteAttendeeResponse)),
    Err: ApiError,
  });
  const SortDirection = IDL.Variant({ Asc: IDL.Null, Desc: IDL.Null });
  const EventSort = IDL.Variant({
    UpdatedOn: SortDirection,
    StartDate: SortDirection,
    CreatedOn: SortDirection,
    EndDate: SortDirection,
  });
  const EventFilter = IDL.Variant({
    Ids: IDL.Vec(IDL.Nat64),
    Tag: IDL.Nat32,
    UpdatedOn: DateRange,
    Name: IDL.Text,
    None: IDL.Null,
    Groups: IDL.Vec(IDL.Nat64),
    IsCanceled: IDL.Bool,
    StartDate: DateRange,
    Owner: IDL.Principal,
    CreatedOn: DateRange,
    EndDate: DateRange,
  });
  const PagedResponse = IDL.Record({
    total: IDL.Nat64,
    data: IDL.Vec(EventResponse),
    page: IDL.Nat64,
    limit: IDL.Nat64,
    number_of_pages: IDL.Nat64,
  });
  const Result_22 = IDL.Variant({ Ok: PagedResponse, Err: ApiError });
  const Result_23 = IDL.Variant({
    Ok: IDL.Vec(InviteMemberResponse),
    Err: ApiError,
  });
  const Result_24 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(InviteMemberResponse, ProfileResponse)),
    Err: ApiError,
  });
  const Result_25 = IDL.Variant({
    Ok: JoinedMemberResponse,
    Err: ApiError,
  });
  const Result_26 = IDL.Variant({
    Ok: IDL.Tuple(JoinedMemberResponse, ProfileResponse),
    Err: ApiError,
  });
  const Result_27 = IDL.Variant({
    Ok: IDL.Vec(JoinedMemberResponse),
    Err: ApiError,
  });
  const Result_28 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(JoinedMemberResponse, ProfileResponse)),
    Err: ApiError,
  });
  const Result_29 = IDL.Variant({ Ok: IDL.Vec(Role), Err: ApiError });
  const GroupFilter = IDL.Variant({
    Ids: IDL.Vec(IDL.Nat64),
    Tag: IDL.Nat32,
    UpdatedOn: DateRange,
    Name: IDL.Text,
    None: IDL.Null,
    Owner: IDL.Principal,
    CreatedOn: DateRange,
  });
  const GroupSort = IDL.Variant({
    UpdatedOn: SortDirection,
    MemberCount: SortDirection,
    Name: SortDirection,
    CreatedOn: SortDirection,
  });
  const PagedResponse_1 = IDL.Record({
    total: IDL.Nat64,
    data: IDL.Vec(GroupResponse),
    page: IDL.Nat64,
    limit: IDL.Nat64,
    number_of_pages: IDL.Nat64,
  });
  const Result_30 = IDL.Variant({ Ok: PagedResponse_1, Err: ApiError });
  const GroupsCount = IDL.Record({
    new: IDL.Nat64,
    total: IDL.Nat64,
    starred: IDL.Nat64,
    invited: IDL.Nat64,
    joined: IDL.Nat64,
  });
  const Logger = IDL.Record({
    principal: IDL.Opt(IDL.Principal),
    source: IDL.Opt(IDL.Text),
    data: IDL.Opt(IDL.Text),
    description: IDL.Text,
    created_on: IDL.Nat64,
  });
  const Result_31 = IDL.Variant({ Ok: IDL.Vec(IDL.Text), Err: ApiError });
  const SubjectType = IDL.Variant({
    Event: IDL.Null,
    Group: IDL.Null,
    Attendee: IDL.Null,
    None: IDL.Null,
    Member: IDL.Null,
    Profile: IDL.Null,
  });
  const RelationType = IDL.Variant({
    Blocked: IDL.Null,
    Friend: IDL.Null,
  });
  const ReportSort = IDL.Variant({
    SubjectType: SortDirection,
    ReportedBy: SortDirection,
    CreatedOn: SortDirection,
    Subject: SortDirection,
  });
  const ReportFilter = IDL.Variant({
    SubjectType: SubjectType,
    None: IDL.Null,
    GroupId: IDL.Nat64,
    ReportedBy: IDL.Principal,
    CreatedOn: DateRange,
    Subject: Subject,
  });
  const PagedResponse_2 = IDL.Record({
    total: IDL.Nat64,
    data: IDL.Vec(ReportResponse),
    page: IDL.Nat64,
    limit: IDL.Nat64,
    number_of_pages: IDL.Nat64,
  });
  const Result_32 = IDL.Variant({ Ok: PagedResponse_2, Err: ApiError });
  const HttpRequest = IDL.Record({
    url: IDL.Text,
    method: IDL.Text,
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const HttpHeader = IDL.Record({ value: IDL.Text, name: IDL.Text });
  const HttpResponse = IDL.Record({
    status: IDL.Nat,
    body: IDL.Vec(IDL.Nat8),
    headers: IDL.Vec(HttpHeader),
  });
  const Result_33 = IDL.Variant({
    Ok: InviteAttendeeResponse,
    Err: ApiError,
  });
  const LogType = IDL.Variant({
    Error: IDL.Null,
    Info: IDL.Null,
    Warning: IDL.Null,
  });
  const PostLog = IDL.Record({
    source: IDL.Opt(IDL.Text),
    data: IDL.Opt(IDL.Text),
    log_type: LogType,
    description: IDL.Text,
  });
  const Result_34 = IDL.Variant({
    Ok: IDL.Tuple(IDL.Nat64, Logger),
    Err: ApiError,
  });
  const Result_35 = IDL.Variant({
    Ok: IDL.Vec(IDL.Tuple(IDL.Nat64, UserNotificationData)),
    Err: ApiError,
  });
  const Activity = IDL.Variant({
    UserActivity: IDL.Principal,
    GroupMemberCount: IDL.Nat64,
  });
  const RewardableActivityResponse = IDL.Record({
    timestamp: IDL.Nat64,
    activity: Activity,
  });
  const ClientKey = IDL.Record({
    client_principal: IDL.Principal,
    client_nonce: IDL.Nat64,
  });
  const CanisterWsCloseArguments = IDL.Record({ client_key: ClientKey });
  const Result_36 = IDL.Variant({ Ok: IDL.Null, Err: IDL.Text });
  const CanisterWsGetMessagesArguments = IDL.Record({ nonce: IDL.Nat64 });
  const CanisterOutputMessage = IDL.Record({
    key: IDL.Text,
    content: IDL.Vec(IDL.Nat8),
    client_key: ClientKey,
  });
  const CanisterOutputCertifiedMessages = IDL.Record({
    messages: IDL.Vec(CanisterOutputMessage),
    cert: IDL.Vec(IDL.Nat8),
    tree: IDL.Vec(IDL.Nat8),
    is_end_of_queue: IDL.Bool,
  });
  const Result_37 = IDL.Variant({
    Ok: CanisterOutputCertifiedMessages,
    Err: IDL.Text,
  });
  const WebsocketMessage = IDL.Record({
    sequence_num: IDL.Nat64,
    content: IDL.Vec(IDL.Nat8),
    client_key: ClientKey,
    timestamp: IDL.Nat64,
    is_service_message: IDL.Bool,
  });
  const CanisterWsMessageArguments = IDL.Record({ msg: WebsocketMessage });
  const CanisterWsOpenArguments = IDL.Record({
    gateway_principal: IDL.Principal,
    client_nonce: IDL.Nat64,
  });
  return IDL.Service({
    __get_candid_interface_tmp_hack: IDL.Func([], [IDL.Text], ["query"]),
    _dev_check_attendees_sync: IDL.Func(
      [IDL.Principal, IDL.Nat64],
      [IDL.Tuple(IDL.Text, IDL.Bool), IDL.Tuple(IDL.Text, IDL.Bool)],
      ["query"]
    ),
    _dev_check_events_sync: IDL.Func(
      [IDL.Nat64, IDL.Nat64],
      [IDL.Tuple(IDL.Text, IDL.Bool), IDL.Tuple(IDL.Text, IDL.Bool)],
      ["query"]
    ),
    _dev_check_member_sync: IDL.Func(
      [IDL.Principal, IDL.Nat64],
      [IDL.Tuple(IDL.Text, IDL.Bool), IDL.Tuple(IDL.Text, IDL.Bool)],
      ["query"]
    ),
    _dev_create_canister: IDL.Func([IDL.Vec(IDL.Principal)], [Result], []),
    _dev_get_history_canister: IDL.Func([], [Result_1], ["query"]),
    _dev_get_reward_canister: IDL.Func([], [Result_1], ["query"]),
    _dev_send_reward_data: IDL.Func([], [], []),
    _dev_set_history_canister: IDL.Func([IDL.Principal], [Result_1], []),
    _dev_set_reward_canister: IDL.Func([IDL.Principal], [Result_1], []),
    _expose: IDL.Func([], [IDL.Opt(WSMessage)], ["query"]),
    accept_friend_request: IDL.Func([IDL.Nat64], [Result_2], []),
    accept_owner_request_event_invite: IDL.Func([IDL.Nat64], [Result_3], []),
    accept_owner_request_group_invite: IDL.Func([IDL.Nat64], [Result_4], []),
    accept_user_request_event_invite: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Principal], [Result_5], []),
    accept_user_request_group_invite: IDL.Func([IDL.Nat64, IDL.Principal], [Result_4], []),
    add_event: IDL.Func([PostEvent], [Result_6], []),
    add_friend_request: IDL.Func([IDL.Principal, IDL.Text], [Result_7], []),
    add_group: IDL.Func([PostGroup, IDL.Opt(IDL.Text)], [Result_8], []),
    add_pinned: IDL.Func([Subject], [Result_9], []),
    add_profile: IDL.Func([PostProfile], [Result_9], []),
    add_report: IDL.Func([PostReport], [Result_10], []),
    add_role_to_group: IDL.Func([IDL.Nat64, IDL.Text, IDL.Text, IDL.Nat64], [Result_11], []),
    add_starred: IDL.Func([Subject], [Result_9], []),
    add_topic: IDL.Func([TopicKind, IDL.Text], [Result_12], []),
    add_topics: IDL.Func([TopicKind, IDL.Vec(IDL.Text)], [IDL.Vec(Result_12)], []),
    add_transaction_notification: IDL.Func([TransactionData], [IDL.Bool], []),
    add_transactions_complete_notification: IDL.Func([TransactionCompleteData], [IDL.Bool], []),
    add_wallet_to_group: IDL.Func([IDL.Nat64, IDL.Principal, IDL.Text], [Result_8], []),
    add_wallet_to_profile: IDL.Func([PostWallet], [Result_9], []),
    approve_code_of_conduct: IDL.Func([IDL.Nat64], [Result_2], []),
    approve_privacy_policy: IDL.Func([IDL.Nat64], [Result_2], []),
    approve_terms_of_service: IDL.Func([IDL.Nat64], [Result_2], []),
    assign_role: IDL.Func([IDL.Nat64, IDL.Text, IDL.Principal], [Result_4], []),
    ban_group_member: IDL.Func([IDL.Nat64, IDL.Principal], [Result_13], []),
    block_user: IDL.Func([IDL.Principal], [Result_9], []),
    boost: IDL.Func([Subject, IDL.Nat64], [Result_14], []),
    cancel_event: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Text], [Result_13], []),
    check_new_stores: IDL.Func([], [IDL.Vec(IDL.Text)], ["query"]),
    decline_friend_request: IDL.Func([IDL.Nat64], [Result_2], []),
    decline_owner_request_event_invite: IDL.Func([IDL.Nat64], [Result_3], []),
    decline_owner_request_group_invite: IDL.Func([IDL.Nat64], [Result_4], []),
    decline_user_request_event_invite: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Principal], [Result_5], []),
    decline_user_request_group_invite: IDL.Func([IDL.Nat64, IDL.Principal], [Result_4], []),
    delete_event: IDL.Func([IDL.Nat64, IDL.Nat64], [Result_13], []),
    delete_group: IDL.Func([IDL.Nat64], [Result_15], []),
    edit_event: IDL.Func([IDL.Nat64, IDL.Nat64, UpdateEvent], [Result_6], []),
    edit_group: IDL.Func([IDL.Nat64, UpdateGroup], [Result_8], []),
    edit_profile: IDL.Func([UpdateProfile], [Result_9], []),
    edit_role_permissions: IDL.Func([IDL.Nat64, IDL.Text, IDL.Vec(PostPermission)], [Result_2], []),
    fill_buffer: IDL.Func([], [], []),
    get_all_topics: IDL.Func([TopicKind], [Result_16], ["query"]),
    get_attending_from_principal: IDL.Func([IDL.Principal], [Result_17], ["query"]),
    get_banned_group_members: IDL.Func([IDL.Nat64], [Result_18], ["query"]),
    get_boosted_events: IDL.Func([], [IDL.Vec(EventResponse)], ["query"]),
    get_boosted_groups: IDL.Func([], [IDL.Vec(GroupResponse)], ["query"]),
    get_connected_clients: IDL.Func([], [IDL.Vec(IDL.Principal)], ["query"]),
    get_e8s_per_day_boost_cost: IDL.Func([], [IDL.Nat64], ["query"]),
    get_event: IDL.Func([IDL.Nat64], [Result_6], ["query"]),
    get_event_attendees: IDL.Func([IDL.Nat64], [Result_17], ["query"]),
    get_event_attendees_profiles_and_roles: IDL.Func([IDL.Nat64], [Result_19], ["query"]),
    get_event_count: IDL.Func([IDL.Opt(IDL.Vec(IDL.Nat64)), IDL.Opt(IDL.Text)], [EventsCount], ["query"]),
    get_event_invites: IDL.Func([IDL.Nat64, IDL.Nat64], [Result_20], ["query"]),
    get_event_invites_with_profiles: IDL.Func([IDL.Nat64], [Result_21], ["query"]),
    get_events: IDL.Func([IDL.Nat64, IDL.Nat64, EventSort, IDL.Vec(EventFilter)], [Result_22], ["query"]),
    get_group: IDL.Func([IDL.Nat64], [Result_8], ["query"]),
    get_group_by_name: IDL.Func([IDL.Text], [Result_8], ["query"]),
    get_group_invites: IDL.Func([IDL.Nat64], [Result_23], ["query"]),
    get_group_invites_with_profiles: IDL.Func([IDL.Nat64], [Result_24], ["query"]),
    get_group_member: IDL.Func([IDL.Nat64, IDL.Principal], [Result_25], ["query"]),
    get_group_member_with_profile: IDL.Func([IDL.Nat64, IDL.Principal], [Result_26], ["query"]),
    get_group_members: IDL.Func([IDL.Nat64], [Result_27], ["query"]),
    get_group_members_with_profiles: IDL.Func([IDL.Nat64], [Result_28], ["query"]),
    get_group_roles: IDL.Func([IDL.Nat64], [Result_29], []),
    get_groups: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Vec(GroupFilter), GroupSort], [Result_30], ["query"]),
    get_groups_by_id: IDL.Func([IDL.Vec(IDL.Nat64)], [IDL.Vec(GroupResponse)], ["query"]),
    get_groups_count: IDL.Func([IDL.Opt(IDL.Text)], [GroupsCount], ["query"]),
    get_groups_for_members: IDL.Func([IDL.Vec(IDL.Principal)], [IDL.Vec(JoinedMemberResponse)], ["query"]),
    get_history_point: IDL.Func([], [Result_14], ["query"]),
    get_incoming_friend_requests: IDL.Func([], [IDL.Vec(FriendRequestResponse)], ["query"]),
    get_incoming_friend_requests_with_profile: IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(FriendRequestResponse, ProfileResponse))],
      ["query"]
    ),
    get_latest_logs: IDL.Func([IDL.Nat64], [IDL.Vec(Logger)], ["query"]),
    get_member_roles: IDL.Func([IDL.Nat64, IDL.Principal], [Result_31], ["query"]),
    get_notifications: IDL.Func([], [IDL.Vec(NotificationResponse)], ["query"]),
    get_outgoing_friend_requests: IDL.Func([], [IDL.Vec(FriendRequestResponse)], ["query"]),
    get_outgoing_friend_requests_with_profile: IDL.Func(
      [],
      [IDL.Vec(IDL.Tuple(FriendRequestResponse, ProfileResponse))],
      ["query"]
    ),
    get_pinned_by_subject_type: IDL.Func([SubjectType], [IDL.Vec(SubjectResponse)], ["query"]),
    get_profile: IDL.Func([IDL.Principal], [Result_9], ["query"]),
    get_profiles: IDL.Func([IDL.Vec(IDL.Principal)], [IDL.Vec(ProfileResponse)], ["query"]),
    get_relations: IDL.Func([RelationType], [IDL.Vec(IDL.Principal)], ["query"]),
    get_relations_count: IDL.Func([RelationType], [IDL.Nat64], ["query"]),
    get_relations_with_profiles: IDL.Func([RelationType], [IDL.Vec(ProfileResponse)], ["query"]),
    get_remaining_boost_time_in_seconds: IDL.Func([Subject], [Result_14], ["query"]),
    get_report: IDL.Func([IDL.Nat64, IDL.Nat64], [Result_10], ["query"]),
    get_reports: IDL.Func([IDL.Nat64, IDL.Nat64, ReportSort, IDL.Vec(ReportFilter), IDL.Nat64], [Result_32], ["query"]),
    get_self_attendee: IDL.Func([], [Result_3], ["query"]),
    get_self_events: IDL.Func([], [IDL.Vec(EventResponse)], ["query"]),
    get_self_groups: IDL.Func([], [IDL.Vec(GroupResponse)], ["query"]),
    get_self_member: IDL.Func([], [Result_4], ["query"]),
    get_starred_by_subject_type: IDL.Func([SubjectType], [IDL.Vec(IDL.Nat64)], ["query"]),
    get_topic: IDL.Func([TopicKind, IDL.Nat64], [Result_12], ["query"]),
    get_topics: IDL.Func([TopicKind, IDL.Vec(IDL.Nat64)], [Result_16], ["query"]),
    get_unread_notifications: IDL.Func([], [IDL.Vec(NotificationResponse)], ["query"]),
    http_request: IDL.Func([HttpRequest], [HttpResponse], ["query"]),
    invite_to_event: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Principal], [Result_33], []),
    invite_to_group: IDL.Func([IDL.Nat64, IDL.Principal], [Result_4], []),
    join_event: IDL.Func([IDL.Nat64], [Result_5], []),
    join_group: IDL.Func([IDL.Nat64, IDL.Opt(IDL.Text)], [Result_25], []),
    leave_event: IDL.Func([IDL.Nat64], [Result_13], []),
    leave_group: IDL.Func([IDL.Nat64], [Result_13], []),
    log: IDL.Func([PostLog], [Result_34], []),
    log_login: IDL.Func([], [Result_34], []),
    log_size: IDL.Func([], [IDL.Nat64], ["query"]),
    log_with_caller: IDL.Func([PostLog], [Result_34], []),
    mark_notifications_as_read: IDL.Func([IDL.Vec(IDL.Nat64), IDL.Bool], [Result_35], []),
    migrate: IDL.Func([], [IDL.Vec(IDL.Vec(IDL.Text))], []),
    multisig_new_proposal_notification: IDL.Func(
      [IDL.Vec(IDL.Principal), IDL.Principal, IDL.Nat64, IDL.Nat64],
      [IDL.Bool],
      []
    ),
    multisig_proposal_accept_notification: IDL.Func(
      [IDL.Vec(IDL.Principal), IDL.Principal, IDL.Nat64, IDL.Nat64],
      [IDL.Bool],
      []
    ),
    multisig_proposal_decline_notification: IDL.Func(
      [IDL.Vec(IDL.Principal), IDL.Principal, IDL.Nat64, IDL.Nat64],
      [IDL.Bool],
      []
    ),
    multisig_proposal_status_update_notification: IDL.Func(
      [IDL.Vec(IDL.Principal), IDL.Principal, IDL.Nat64, IDL.Nat64],
      [IDL.Bool],
      []
    ),
    multisig_whitelist_notice_notification: IDL.Func(
      [IDL.Vec(IDL.Principal), IDL.Principal, IDL.Nat64],
      [IDL.Bool],
      []
    ),
    read_reward_buffer: IDL.Func([], [IDL.Vec(RewardableActivityResponse)], ["query"]),
    remove_all_notifications: IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Nat64, UserNotificationData))], []),
    remove_attendee_from_event: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Principal], [Result_13], []),
    remove_attendee_invite_from_event: IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Principal], [Result_13], []),
    remove_ban_from_group_member: IDL.Func([IDL.Nat64, IDL.Principal], [Result_13], []),
    remove_event_invite: IDL.Func([IDL.Nat64], [Result_13], []),
    remove_friend: IDL.Func([IDL.Principal], [Result_9], []),
    remove_friend_request: IDL.Func([IDL.Nat64], [Result_2], []),
    remove_group_role: IDL.Func([IDL.Nat64, IDL.Text], [Result_2], []),
    remove_invite: IDL.Func([IDL.Nat64], [Result_13], []),
    remove_member_from_group: IDL.Func([IDL.Nat64, IDL.Principal], [Result_13], []),
    remove_member_invite_from_group: IDL.Func([IDL.Nat64, IDL.Principal], [Result_13], []),
    remove_member_role: IDL.Func([IDL.Nat64, IDL.Text, IDL.Principal], [Result_4], []),
    remove_notifications: IDL.Func([IDL.Vec(IDL.Nat64)], [IDL.Vec(IDL.Tuple(IDL.Nat64, UserNotificationData))], []),
    remove_pinned: IDL.Func([Subject], [Result_9], []),
    remove_starred: IDL.Func([Subject], [Result_9], []),
    remove_topic: IDL.Func([TopicKind, IDL.Nat64], [IDL.Bool], []),
    remove_wallet_from_group: IDL.Func([IDL.Nat64, IDL.Principal], [Result_8], []),
    remove_wallet_from_profile: IDL.Func([IDL.Principal], [Result_9], []),
    reward_timer_next_trigger: IDL.Func([], [IDL.Opt(IDL.Nat64)], ["query"]),
    set_wallet_as_primary: IDL.Func([IDL.Principal], [Result_9], []),
    store_stats: IDL.Func([], [IDL.Vec(IDL.Text)], ["query"]),
    test_log: IDL.Func([], [], []),
    unblock_user: IDL.Func([IDL.Principal], [Result_9], []),
    ws_close: IDL.Func([CanisterWsCloseArguments], [Result_36], []),
    ws_get_messages: IDL.Func([CanisterWsGetMessagesArguments], [Result_37], ["query"]),
    ws_message: IDL.Func([CanisterWsMessageArguments, IDL.Opt(WSMessage)], [Result_36], []),
    ws_open: IDL.Func([CanisterWsOpenArguments], [Result_36], []),
  });
};
