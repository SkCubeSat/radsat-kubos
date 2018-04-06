/*
 * Copyright (C) 2018 Kubos Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//TODO: remove before publishing
#![allow(unused)]

use byteorder::{LittleEndian, WriteBytesExt};
use crc16::*;
use failure::Error;
use messages::*;
use serial_comm::Connection;

//TODO: Create container trait? Or just for the serial connection part...

pub struct MAI400 {
    conn: Connection,
}

impl MAI400 {
    /// Constructor for MAI400 structure
    pub fn new(conn: Connection) -> MAI400 {
        MAI400 { conn }
    }

    pub fn init_tx() {}

    pub fn init_rx() {}

    pub fn terminate() {

        //close(tx);
        //close(rx);
    }

    pub fn reset() {
        //REQUEST_RESET
        //CONFIRM_RESET
    }

    pub fn set_mode() {}

    pub fn get_info(&self) -> MAIResult<()> {
        //Create packet
        let mut packet = GetInfoMessage::default().serialize();

        let crc = State::<AUG_CCITT>::calculate(&packet[1..]);
        packet.write_u16::<LittleEndian>(crc).unwrap();

        //send packet
        self.conn.write(packet.as_slice())?;
        Ok(())
    }
    //Option 2
    //Don't actually merge this. Need to figure out which way is preferable
    pub fn get_info_alt(&self) -> MAIResult<()> {
        //Create packet
        let packet = GetInfoMessage::default();
        let slice = unsafe {
            ::std::mem::transmute::<GetInfoMessage, [u8; ::std::mem::size_of::<GetInfoMessage>()]>(
                packet,
            )
        };

        //send packet
        self.conn.write(&slice)?;
        Ok(())
    }

    fn send_message() {}
}

/// Common Error for MAI Actions
#[derive(Fail, Display, Debug)]
pub enum MAIError {
    #[display(fmt = "Parse error: {}", message)]
    /// There was a problem parsing the result data
    ParseError {
        /// The message from original error
        message: String,
    },
}

/// Custom error type for MAI400 operations.
pub type MAIResult<T> = Result<T, Error>;
