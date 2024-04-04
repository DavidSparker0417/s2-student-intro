use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum StudentIntroInstruction {
    AddIntro { name: String, message: String },
    UpdateIntro { name: String, message: String },
}

#[derive(BorshDeserialize)]
struct StudentIntroPayload {
    name: String,
    message: String,
}

impl StudentIntroInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&cmd, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let payload = StudentIntroPayload::try_from_slice(rest).unwrap();
        Ok(match cmd {
            0 => StudentIntroInstruction::AddIntro {
                name: payload.name,
                message: payload.message,
            },
            1 => StudentIntroInstruction::UpdateIntro {
                name: payload.name,
                message: payload.message,
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        })
    }
}
