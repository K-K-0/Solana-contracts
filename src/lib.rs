use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::next_account_info, entrypoint::{self, ProgramResult, __AccountInfo}, pubkey::Pubkey};
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

pub  fn counter_p(programme_id: &Pubkey,accounts: &[__AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let counter = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            counter.count += value;
        },
        InstructionType::Decrement(value) => {
            counter.count -= value;
        };
    }
    




}