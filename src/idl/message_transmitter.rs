use anchor_lang::{AnchorDeserialize, Discriminator};
use message_transmitter::instructions as Instructions;

pub trait Instruction: Sized {
    fn match_log(data: &Vec<u8>) -> bool;
    fn decode(data: &Vec<u8>) -> Result<Self, String>;
    fn match_and_decode(data: &Vec<u8>) -> Option<Self> {
        if Self::match_log(data) {
            Self::decode(data).ok()
        } else {
            None
        }
    }
}

// Assuming SendMessage implements Default
impl Instruction for message_transmitter::instruction::SendMessage {
    fn match_log(data: &Vec<u8>) -> bool {
        data.starts_with(&Self::DISCRIMINATOR)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, String> {

        if data.len() >= 8 {
            
            let decode = self::Instructions::SendMessageParams::try_from_slice(&data[8..]);

            match decode  {
                Ok(d) => {
                    return Ok(
                        Self{
                            params : d
                        }
                    )
                },
                _ => { 
                    Err("Unable to decode".to_string()) }
            }
        }
        else {
            Err("Invalid data length".to_string())
        }
    }
}

impl Instruction for message_transmitter::instruction::SendMessageWithCaller {
    fn match_log(data: &Vec<u8>) -> bool {
        data.starts_with(&Self::DISCRIMINATOR)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, String> {

        if data.len() >= 8 {
            
            let decode = self::Instructions::SendMessageWithCallerParams::try_from_slice(&data[8..]);

            match decode  {
                Ok(d) => {
                    return Ok(
                        Self{
                            params : d
                        }
                    )
                },
                _ => { Err("Unable to decode".to_string()) }
            }
        }
        else {
            Err("Invalid data length".to_string())
        }
    }
}

impl Instruction for message_transmitter::instruction::ReceiveMessage {
    fn match_log(data: &Vec<u8>) -> bool {
        data.starts_with(&Self::DISCRIMINATOR)
    }

    fn decode(data: &Vec<u8>) -> Result<Self, String> {

        if data.len() >= 8 {
            
            let decode = self::Instructions::ReceiveMessageParams::try_from_slice(&data[8..]);

            match decode  {
                Ok(d) => {
                    return Ok(
                        Self{
                            params : d
                        }
                    )
                },
                _ => { Err("Unable to decode".to_string()) }
            }
        }
        else {
            Err("Invalid data length".to_string())
        }
    }
}