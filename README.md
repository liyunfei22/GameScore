# GameScore Smart Contract

GameScore is a smart contract written in ink! for managing game players' scores and rankings.

## Features

- Record player scores
- Automatically update player rankings
- Query player scores
- Retrieve a ranked list of all players

## Contract Structure

The contract primarily consists of two storage items:

1. `score`: A mapping that stores each player's (AccountId) corresponding score.
2. `ranking`: A vector that stores player AccountIds sorted by score in descending order.

## Main Functions

### Constructor

- `new()`: Initializes the contract, creating empty score mapping and ranking list.

### Messages

- `update_score(new_score: u32)`: Updates the caller's score and adjusts the ranking accordingly.
- `get_score() -> Option<u32>`: Retrieves the current score of the caller.
- `get_ranking() -> Vec<(AccountId, u32)>`: Retrieves the ranking and scores of all players.

## Usage Examples

1. Deploy the contract:
   Call the `new()` constructor to deploy the contract.

2. Update score:
   Players can call `update_score(new_score)` to update their own score.

3. Query score:
   Players can call `get_score()` to query their current score.

4. Get ranking:
   Anyone can call `get_ranking()` to get the ranking and scores of all players.
