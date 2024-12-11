use anchor_lang::prelude::*;

declare_id!("rtg4oPj1cYrTKqXxR2Pjj5Ty28yHbBsGAWPi5ehvtXy");

#[program]
pub mod voting_program {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        _poll_id: u64,
        poll_start: u64,
        poll_end: u64,
        name: String,
        description: String
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;

        poll.poll_name = name;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_end;

        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        _poll_id: u64,
        candidate: String,
    ) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let poll = &mut ctx.accounts.poll_account;

        candidate_account.candidate_name = candidate;
        poll.poll_option_index += 1;

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>,
        _poll_id: u64,
        _candidate: String,
    ) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > (ctx.accounts.poll_account.poll_end as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (ctx.accounts.poll_account.poll_start as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate_account.candidate_votes += 1;
    
        msg!("Voted for candidate: {}", candidate_account.candidate_name);
        msg!("Votes: {}", candidate_account.candidate_votes);
    
        Ok(())
    }
}



#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
} 


#[derive(Accounts)]
#[instruction( poll_id: u64, candidate: String )]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump,
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction( poll_id: u64, candidate: String,)]
pub struct Vote<'info> {
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub poll_option_index: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}
