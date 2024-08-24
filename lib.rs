#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod game_score {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct GameScore {
        owner: AccountId,
        score: Mapping<String, Vec<(AccountId, u32)>>,
        ranking: Mapping<String, Vec<AccountId>>,
    }


    impl GameScore {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                score: Mapping::default(),
                ranking: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn update_score(&mut self, room_id: String, player: AccountId, new_score: u32) {
            if self.env().caller() != self.owner {
                ink::env::debug_println!("Not owner");
                return;
            }

            let mut scores = self.score.get(&room_id).unwrap_or_default();
            
            if let Some(score) = scores.iter_mut().find(|(p, _)| *p == player) {
                score.1 = new_score;
            } else {
                scores.push((player, new_score));
            }

            scores.sort_by(|a, b| b.1.cmp(&a.1));
            self.score.insert(&room_id, &scores);

            let ranking: Vec<AccountId> = scores.iter().map(|(p, _)| *p).collect();
            self.ranking.insert(&room_id, &ranking);
        }

        #[ink(message)]
        pub fn get_room_score(&self, room_id: String) -> Vec<(AccountId, u32)> {
            if self.env().caller() != self.owner {
                ink::env::debug_println!("Not owner");
                return Vec::new();
            }
            self.score.get(&room_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_room_ranking(&self, room_id: String) -> Vec<AccountId> {
            if self.env().caller() != self.owner {
                ink::env::debug_println!("Not owner");
                return Vec::new();
            }
            self.ranking.get(&room_id).unwrap_or_default()
        }
    }
}