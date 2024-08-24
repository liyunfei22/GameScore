#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::env::Error as EnvError;

#[ink::contract]
mod kart_game {
    use super::*;

    #[ink(storage)]
    pub struct KartGame {
        owner: AccountId,
        token_contract: AccountId,
        game_pool: u128,
        players: Vec<AccountId>,
        game_active: bool,
    }

    #[ink(event)]
    pub struct PlayerJoined {
        #[ink(topic)]
        player: AccountId,
        #[ink(topic)]
        amount: u128,
    }

    #[ink(event)]
    pub struct GameStarted {
        #[ink(topic)]
        players: Vec<AccountId>,
    }

    #[ink(event)]
    pub struct GameEnded {
        #[ink(topic)]
        winner: AccountId,
        #[ink(topic)]
        reward: u128,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        GameAlreadyActive,
        GameNotActive,
        TransferFailed,
        InsufficientBalance,
        PlayerAlreadyJoined,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl KartGame {
        /// 构造函数，初始化游戏合约
        #[ink(constructor)]
        pub fn new(token_contract: AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                token_contract,
                game_pool: 0,
                players: Vec::new(),
                game_active: false,
            }
        }

        /// 玩家加入游戏，缴纳token
        #[ink(message, payable)]
        pub fn join_game(&mut self) -> Result<()> {
            let caller = self.env().caller();
            let amount = self.env().transferred_value();

            // 确保游戏没有在进行中
            if self.game_active {
                return Err(Error::GameAlreadyActive);
            }

            // 确保玩家没有已经加入
            if self.players.contains(&caller) {
                return Err(Error::PlayerAlreadyJoined);
            }

            // 转账token到合约
            self.env().transfer(self.env().account_id(), amount).map_err(|_| Error::TransferFailed)?;

            // 增加奖池
            self.game_pool += amount;
            self.players.push(caller);

            self.env().emit_event(PlayerJoined { player: caller, amount });

            Ok(())
        }

        /// 开始游戏，锁定玩家列表
        #[ink(message)]
        pub fn start_game(&mut self) -> Result<()> {
            self.ensure_owner()?;

            if self.game_active {
                return Err(Error::GameAlreadyActive);
            }

            self.game_active = true;

            self.env().emit_event(GameStarted { players: self.players.clone() });

            Ok(())
        }

        /// 结束游戏，分发奖励
        #[ink(message)]
        pub fn end_game(&mut self, winner: AccountId) -> Result<()> {
            self.ensure_owner()?;

            if !self.game_active {
                return Err(Error::GameNotActive);
            }

            let reward = self.game_pool * 80 / 100; // 80%给胜者
            let contract_share = self.game_pool - reward; // 20%给合约

            // 转账奖励给胜者
            self.env().transfer(winner, reward).map_err(|_| Error::TransferFailed)?;

            // 更新合约余额
            self.game_pool = contract_share;

            // 清空玩家列表并重置游戏状态
            self.players.clear();
            self.game_active = false;

            self.env().emit_event(GameEnded { winner, reward });

            Ok(())
        }

        /// 确保调用者是合约的所有者
        fn ensure_owner(&self) -> Result<()> {
            if self.env().caller() != self.owner {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }
}
