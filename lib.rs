#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod game_score {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

 
    #[ink(storage)]
    pub struct GameScore {
        score: ink::storage::Mapping<AccountId, u32>,
        ranking: Vec<AccountId>,
    }

    impl GameScore {
        #[ink(constructor)]
        pub fn new() -> Self {
            let score = Mapping::default();
            let ranking = Vec::default();
            Self {
                score,
                ranking
             }
        }


        #[ink(message)]
        pub fn update_score(&mut self, new_score: u32) {
            let caller = self.env().caller();
            self.score.insert(caller, &new_score);
            self.update_ranking(caller, new_score);
        }

        fn update_ranking(&mut self, player: AccountId, score: u32) {
            // 移除玩家在排行榜中的旧位置
            self.ranking.retain(|&p| p != player);
    
            // 查找玩家的新位置并插入玩家
            let pos = self.ranking.iter()
                                   .position(|&p| self.score.get(&p).unwrap_or(0) < score)
                                   .unwrap_or(self.ranking.len());
            self.ranking.insert(pos, player);
        }

        #[ink(message)]
        pub fn get_score(&self) -> Option<u32> {
            let caller = self.env().caller();
            self.score.get(caller)
        }

        #[ink(message)]
        pub fn get_ranking(&self) -> Vec<(AccountId, u32)> {
            self.ranking.iter()
                        .map(|&player| (player, self.score.get(&player).unwrap_or(0)))
                        .collect()
        }
    }

}
