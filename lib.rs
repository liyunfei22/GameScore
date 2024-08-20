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

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn test_new() {
            let game_score = GameScore::new();
            assert_eq!(game_score.get_ranking(), Vec::new());
        }

        #[ink::test]
        fn test_update_score() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            game_score.update_score(100);
            assert_eq!(game_score.get_score(), Some(100));

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            game_score.update_score(200);
            assert_eq!(game_score.get_score(), Some(200));
        }

        #[ink::test]
        fn test_ranking() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            game_score.update_score(100);

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            game_score.update_score(200);

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            game_score.update_score(150);

            let ranking = game_score.get_ranking();
            assert_eq!(ranking.len(), 3);
            assert_eq!(ranking[0], (accounts.bob, 200));
            assert_eq!(ranking[1], (accounts.charlie, 150));
            assert_eq!(ranking[2], (accounts.alice, 100));
        }

        #[ink::test]
        fn test_update_existing_score() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            game_score.update_score(100);
            game_score.update_score(150);

            assert_eq!(game_score.get_score(), Some(150));
            assert_eq!(game_score.get_ranking(), vec![(accounts.alice, 150)]);
        }
    }

}
