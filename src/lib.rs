use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, msg, pubkey::Pubkey};
#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}
#[derive(BorshSerialize, BorshDeserialize)]

struct Counter {
    count: u32
}

entrypoint!(counter_p);

pub  fn counter_p(programme_id: &Pubkey,accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("Increment counter");
            counter.count = counter.count.saturating_add(value);
        },
        InstructionType::Decrement(value) => {
            msg!("Decrement counter");
            counter.count = counter.count.saturating_sub(value);
        };
    }

    counter.serialize(&mut &mut acc.data.borrow_mut()[..])?;
    msg!("Counter updated: {}", counter.count);

    Ok(())
    




}