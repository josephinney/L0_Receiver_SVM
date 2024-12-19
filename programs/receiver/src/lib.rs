use anchor_lang::prelude::*;
use anchor_lang::solana_program::msg;

declare_id!("Fg6PaFpoGXkY7U4g8F3LhsLfegJziKwDdd5ARfJjQYyD");

#[program]
pub mod destination_oapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let oapp_receiver = &mut ctx.accounts.oapp_receiver;
        oapp_receiver.data = "Nothing received yet".to_string();
        Ok(())
    }

    pub fn _lzReceive(
        ctx: Context<ReceiveMessage>,
        message: String,
        src_eid: u32,  // Assume this is the source EID of the message
        sender: Pubkey, // The sender's address
        nonce: u64,     // Nonce for tracking the message
    ) -> Result<()> {
        let oapp_receiver = &mut ctx.accounts.oapp_receiver;
        oapp_receiver.data = message.clone();
        
        // Emit log messages (simulates emitting an event)
        msg!("Message received: {}", message);
        msg!("Source EID: {}", src_eid);
        msg!("Sender: {:?}", sender);
        msg!("Nonce: {}", nonce);

        Ok(())
    }
}

#[account]
pub struct OAppReceiver {
    pub data: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub oapp_receiver: Account<'info, OAppReceiver>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReceiveMessage<'info> {
    #[account(mut)]
    pub oapp_receiver: Account<'info, OAppReceiver>,
}

