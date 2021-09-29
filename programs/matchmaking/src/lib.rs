use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod matchmaking {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    //#[access_control(CreateMatch::accounts(&ctx, nonce))]
    pub fn create_match(ctx: Context<CreateMatch>, amount: u64, nonce: u8) -> ProgramResult {
        let MatchData = &mut ctx.accounts.matchdata;

        //emmiter here for indexing

        Ok(())
    }
    pub fn conclude_match(ctx: Context<ConcludeMatch>) -> ProgramResult {
        Ok(())
    }
    pub fn settle_prize(ctx: Context<SettlePrize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateMatch {
    #[account(init, payer = player , space= 8+8 )]
    pub matchdata: ProgramAccount<'info, MatchData>,

    #[account(mut)]
    pub player_one: AccountInfo<'info>,
    pub player_two: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ConcludeMatch {}

#[derive(Accounts)]
pub struct SettlePrize {}

#[account]
pub struct MatchData {
    pub player_one: Pubkey,
    pub player_two: Pubkey,
    pub prize_settled: bool,
    pub winner_address: Pubkey,
}
