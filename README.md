# Matchmaking

Hardcoded constants:
- referee_authority: Keypair which has privilaged access to conclude_match function. This is a regular wallet address, whose private key is held by your game server.

PDAs:
- vault_authority: PDA which controls smart contract vaults
- token_vault: wSOL vault, which is an associated token account of vault_authority

Accounts:
1. struct match_data: store player public keys, match_seed, winner_address (null value means match running) and prize_settled(boolean). Each new match gets a new match_data struct as a PDA of player addresses and match seeds. match_data PDA is a unique identifier for each match.

1. new_match(player_1, player_2, match_seed): Transfer wSOL to token_vault then write data to match_data
2. conclude_match(winner_pubkey): Must be signed by referee_authority. Set winner_address and prize_settled = 0 in match_data struct.
3. settle_prize(): Signed  by winner. Check if winner matches and prize is not settled. Transfer winnings to user.

