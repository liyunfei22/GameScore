#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod game_score {
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct GameScore {
        owner: AccountId,
        score: ink::storage::Mapping<AccountId, u32>,
        ranking: Vec<AccountId>,
    }

    impl GameScore {
        #[ink(constructor)]
        pub fn new() -> Self {
            let score = Mapping::default();
            let ranking = Vec::default();
            let owner = Self::env().caller();
            Self {
                owner,
                score,
                ranking
            }
        }

        #[ink(message)]
        pub fn update_score(&mut self, player: AccountId, new_score: u32) -> Result<(), Error> {
            self.ensure_owner()?;
            self.score.insert(player, &new_score);
            self.update_ranking(player, new_score);
            Ok(())
        }

        fn update_ranking(&mut self, player: AccountId, score: u32) {
            self.ranking.retain(|&p| p != player);
    
            let pos = self.ranking.iter()
                                   .position(|&p| self.score.get(&p).unwrap_or(0) < score)
                                   .unwrap_or(self.ranking.len());
            self.ranking.insert(pos, player);
        }

        #[ink(message)]
        pub fn get_score(&self, player: AccountId) -> Result<Option<u32>, Error> {
            self.ensure_owner()?;
            Ok(self.score.get(player))
        }

        #[ink(message)]
        pub fn get_ranking(&self) -> Result<Vec<(AccountId, u32)>, Error> {
            self.ensure_owner()?;
            Ok(self.ranking.iter()
                        .map(|&player| (player, self.score.get(&player).unwrap_or(0)))
                        .collect())
        }

        fn ensure_owner(&self) -> Result<(), Error> {
            if self.env().caller() == self.owner {
                Ok(())
            } else {
                Err(Error::NotOwner)
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn test_new() {
            let game_score = GameScore::new();
            assert_eq!(game_score.get_ranking().unwrap(), Vec::new());
        }

        #[ink::test]
        fn test_update_score() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(game_score.update_score(accounts.alice, 100), Ok(()));
            assert_eq!(game_score.get_score(accounts.alice), Ok(Some(100)));

            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(game_score.update_score(accounts.bob, 200), Err(Error::NotOwner));
        }

        #[ink::test]
        fn test_ranking() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(game_score.update_score(accounts.alice, 100), Ok(()));
            assert_eq!(game_score.update_score(accounts.bob, 200), Ok(()));
            assert_eq!(game_score.update_score(accounts.charlie, 150), Ok(()));

            let ranking = game_score.get_ranking().unwrap();
            assert_eq!(ranking.len(), 3);
            assert_eq!(ranking[0], (accounts.bob, 200));
            assert_eq!(ranking[1], (accounts.charlie, 150));
            assert_eq!(ranking[2], (accounts.alice, 100));
        }

        #[ink::test]
        fn test_update_existing_score() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(game_score.update_score(accounts.alice, 100), Ok(()));
            assert_eq!(game_score.update_score(accounts.alice, 150), Ok(()));

            assert_eq!(game_score.get_score(accounts.alice), Ok(Some(150)));
            assert_eq!(game_score.get_ranking().unwrap(), vec![(accounts.alice, 150)]);
        }

        #[ink::test]
        fn test_permissions() {
            let mut game_score = GameScore::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            // 非所有者不能更新分数
            test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(game_score.update_score(accounts.alice, 100), Err(Error::NotOwner));

            // 非所有者不能获取分数
            assert_eq!(game_score.get_score(accounts.alice), Err(Error::NotOwner));

            // 非所有者不能获取排名
            assert_eq!(game_score.get_ranking(), Err(Error::NotOwner));
        }
    }
}