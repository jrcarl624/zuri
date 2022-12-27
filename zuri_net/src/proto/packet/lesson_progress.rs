use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use crate::proto::packet::PacketType;
use crate::proto::io::{Reader, Writer};

#[derive(Debug, Clone, FromPrimitive, ToPrimitive)]
pub enum LessonAction {
    Start,
    Complete,
    Restart,
}

#[derive(Debug, Clone)]
pub struct LessonProgress {
    pub action: LessonAction,
    pub score: i32,
    pub identifier: String,
}

impl PacketType for LessonProgress {
    fn write(&self, writer: &mut Writer) {
        writer.u8(self.action.to_u8().unwrap());
        writer.var_i32(self.score);
        writer.string(self.identifier.as_str());
    }

    fn read(reader: &mut Reader) -> Self {
        Self {
            action: LessonAction::from_u8(reader.u8()).unwrap(),
            score: reader.var_i32(),
            identifier: reader.string(),
        }
    }
}
