use crate::error::{self, LibResult};
use std::convert::TryFrom;

/// `0xFF`: File Spec: All meta-events begin with FF, then have an event type byte (which is always
/// less than 128)
pub(crate) const FILE_META_EVENT: u8 = 0b1111_1111;

/// `0xF0`: File Spec: `F0 <length> <bytes to be transmitted after F0>`
pub(crate) const FILE_SYSEX_F0: u8 = 0b1111_0000;

/// `0xF7`: File Spec: `F7 <length> <all bytes to be transmitted>`
pub(crate) const FILE_SYSEX_F7: u8 = 0b1111_0111;

/// Represents the status byte types in Table I "Summary of Status Bytes" from the MIDI
/// specification.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum StatusType {
    /// `0x8`: a `Note Off` message.
    NoteOff = 0x8,

    /// `0x9`: a `Note On (a velocity of 0 = Note Off)` message.
    NoteOn = 0x9,

    /// `0xA`: a `Polyphonic key pressure/Aftertouch` message.
    PolyPressure = 0xA,

    /// `0xB`: a `Control change` message or a `Channel Mode` message. Channel Mode messages are
    /// sent under the same Status Byte as the Control Change messages (BnH). They are
    /// differentiated by the first data byte which will have a value from 121 to 127 for Channel
    /// Mode messages.
    ControlOrSelectChannelMode = 0xB,

    /// `0xC`: a `Program change` message.
    Program = 0xC,

    /// `0xD`: a `Channel pressure/After touch` message.
    ChannelPressure = 0xD,

    /// `0xE`: a `Pitch bend change` message.
    PitchBend = 0xE,

    /// `0xF`: a `System Message`.
    System = 0xF,
}

impl Default for StatusType {
    fn default() -> Self {
        StatusType::NoteOff
    }
}

impl StatusType {
    pub(crate) fn from_u8(value: u8) -> LibResult<Self> {
        match value {
            x if StatusType::NoteOff as u8 == x => Ok(StatusType::NoteOff),
            x if StatusType::NoteOn as u8 == x => Ok(StatusType::NoteOn),
            x if StatusType::PolyPressure as u8 == x => Ok(StatusType::PolyPressure),
            x if StatusType::ControlOrSelectChannelMode as u8 == x => {
                Ok(StatusType::ControlOrSelectChannelMode)
            }
            x if StatusType::Program as u8 == x => Ok(StatusType::Program),
            x if StatusType::ChannelPressure as u8 == x => Ok(StatusType::ChannelPressure),
            x if StatusType::PitchBend as u8 == x => Ok(StatusType::PitchBend),
            x if StatusType::System as u8 == x => Ok(StatusType::System),
            _ => error::Other { site: site!() }.fail(),
        }
    }
}

impl TryFrom<u8> for StatusType {
    type Error = crate::Error;

    fn try_from(value: u8) -> crate::Result<Self> {
        Ok(StatusType::from_u8(value)?)
    }
}