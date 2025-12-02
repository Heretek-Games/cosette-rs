#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPlacement {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub item_master_id: i64,
    #[prost(string, tag = "3")]
    pub position: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub status: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub player_short_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub exp: i64,
    #[prost(string, tag = "3")]
    pub intro_text: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub db_profile_decks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "5")]
    pub profile_icon_id: i64,
    #[prost(int64, tag = "6")]
    pub profile_icon_frame: i64,
    #[prost(int64, repeated, tag = "7")]
    pub owned_icon_frames: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "8")]
    pub profile_background: i64,
    #[prost(int64, repeated, tag = "9")]
    pub owned_backgrounds: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "10")]
    pub profile_decks: ::prost::alloc::vec::Vec<ProfileDeck>,
    #[prost(int64, tag = "21")]
    pub stamina: i64,
    #[prost(sint64, tag = "22")]
    pub stamina_updated_time: i64,
    #[prost(sint64, tag = "23")]
    pub stamina_purchase_time: i64,
    #[prost(int64, tag = "24")]
    pub stamina_purchase_count: i64,
    #[prost(sint64, tag = "25")]
    pub stamina_purchase_time2: i64,
    #[prost(int64, tag = "26")]
    pub stamina_purchase_count2: i64,
    #[prost(string, tag = "31")]
    pub name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "32")]
    pub name_latest_change_at: i64,
    #[prost(string, tag = "33")]
    pub initial_affcode: ::prost::alloc::string::String,
    #[prost(int64, tag = "41")]
    pub explore_coin_received: i64,
    #[prost(int64, tag = "42")]
    pub explore_coin_limit: i64,
    #[prost(int64, tag = "44")]
    pub explore_coin_drop: i64,
    #[prost(sint64, tag = "43")]
    pub explore_coin_limit_updated_at: i64,
    #[prost(string, tag = "51")]
    pub location: ::prost::alloc::string::String,
    #[prost(int64, tag = "52")]
    pub recharge_reward_at: i64,
    #[prost(int64, repeated, tag = "61")]
    pub profile_badges: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "161")]
    pub db_profile_badge_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "162")]
    pub profile_badge_list: ::prost::alloc::vec::Vec<ProfileBadge>,
    #[prost(int64, repeated, tag = "62")]
    pub pop_gift_queue: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, tag = "100")]
    pub rank_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "101")]
    pub rank_last_reset_at: i64,
    #[prost(sint64, tag = "71")]
    pub latest_share_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileDeck {
    #[prost(int64, tag = "1")]
    pub battle_character_master_id: i64,
    #[prost(int32, tag = "2")]
    pub action_id: i32,
    #[prost(int32, tag = "3")]
    pub direction: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileBadge {
    #[prost(int64, tag = "1")]
    pub badge_id: i64,
    #[prost(string, tag = "2")]
    pub x: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub y: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerTutorialInfo {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub step: i64,
    #[prost(int64, repeated, tag = "3")]
    pub feature_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerWallet {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub gold: i64,
    #[prost(int64, tag = "12")]
    pub paid_gem: i64,
    #[prost(int64, tag = "13")]
    pub free_gem: i64,
    #[prost(int64, tag = "14")]
    pub gacha_ticket_1: i64,
    #[prost(int64, tag = "21")]
    pub paid_stone: i64,
    #[prost(int64, tag = "22")]
    pub free_stone: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerWalletResponse {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<player_wallet_response::ResourcesEntry>,
    #[prost(int64, tag = "11")]
    pub gold: i64,
    #[prost(int64, tag = "12")]
    pub paid_gem: i64,
    #[prost(int64, tag = "13")]
    pub free_gem: i64,
    #[prost(int64, tag = "21")]
    pub paid_stone: i64,
    #[prost(int64, tag = "22")]
    pub free_stone: i64,
    #[prost(int64, tag = "41")]
    pub gacha_ticket_1: i64,
}
/// Nested message and enum types in `PlayerWalletResponse`.
pub mod player_wallet_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourcesEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerWalletChangeLog {
    #[prost(string, tag = "1")]
    pub player_wallet_change_log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub takasho_transaction_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub paid_gem_diff: i64,
    #[prost(int64, tag = "12")]
    pub paid_gem_remain: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerVersion {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub version: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RechargeRebate {
    #[prost(string, tag = "10")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "20")]
    pub month_pass: i64,
    #[prost(string, repeated, tag = "30")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(sint64, tag = "90")]
    pub send_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerHistory {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub login_count: i64,
    #[prost(sint64, tag = "12")]
    pub latest_login_time: i64,
    #[prost(int64, tag = "13")]
    pub stamina_cost_count: i64,
    #[prost(int64, tag = "14")]
    pub accumulated_won_game_money: i64,
    #[prost(int64, tag = "15")]
    pub accumulated_lost_game_money: i64,
    #[prost(int64, tag = "16")]
    pub purchase_stamina_with_gem_count: i64,
    #[prost(int64, tag = "17")]
    pub continuous_login_count: i64,
    #[prost(int64, tag = "21")]
    pub battle_character_level_up_count: i64,
    #[prost(int64, tag = "31")]
    pub equipment_level_up_count: i64,
    #[prost(int64, repeated, tag = "32")]
    pub equipment_set: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "33")]
    pub deleted_equipments_level_counts: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "34")]
    pub deleted_equipments_rank_counts: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "35")]
    pub equipment_fitted_count: i64,
    #[prost(string, tag = "41")]
    pub sending_gift_count: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "42")]
    pub finished_puzzle_set: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "43")]
    pub puzzle_play_through_count: i64,
    #[prost(int64, tag = "44")]
    pub s_level_drink_making_count: i64,
    #[prost(string, tag = "45")]
    pub finished_explore_puzzle_count: ::prost::alloc::string::String,
    #[prost(int64, tag = "46")]
    pub drink_making_count: i64,
    #[prost(string, tag = "47")]
    pub draw_card_with_characters: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "48")]
    pub s_level_drink_making_type: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "51")]
    pub received_global_mail_ids: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(int64, tag = "61")]
    pub battle_win_count: i64,
    #[prost(int64, tag = "62")]
    pub play_through_memory_quest_count: i64,
    #[prost(int64, tag = "63")]
    pub play_through_daily_quest_count: i64,
    #[prost(int64, tag = "64")]
    pub monster_kill_count: i64,
    #[prost(int64, tag = "65")]
    pub melody_attack_count: i64,
    #[prost(int64, tag = "66")]
    pub beat_off_enemy_count: i64,
    #[prost(int64, tag = "67")]
    pub pull_enemy_count: i64,
    #[prost(int64, tag = "68")]
    pub beaten_off_by_enemy_count: i64,
    #[prost(int64, tag = "69")]
    pub pulled_by_enemy_count: i64,
    #[prost(string, tag = "70")]
    pub buffed_counts: ::prost::alloc::string::String,
    #[prost(int64, tag = "71")]
    pub battle_support_count: i64,
    #[prost(int64, tag = "72")]
    pub battle_lose_count: i64,
    #[prost(int64, repeated, tag = "81")]
    pub finished_daily_quests: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "82")]
    pub finished_newbie_activity: bool,
    #[prost(int64, repeated, tag = "83")]
    pub equipped_suits: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "84")]
    pub total_get_phonemes: i64,
    #[prost(int64, tag = "85")]
    pub total_levelup_phoneme_times: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
