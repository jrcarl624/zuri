use crate::proto::io::{Reader, Writer};
use crate::proto::packet::PacketType;
use crate::proto::types::game_rule::GameRule;

#[derive(Debug, Clone)]
pub struct GameRulesChanged {
    pub game_rules: Vec<GameRule>,
}

impl PacketType for GameRulesChanged {
    fn write(&self, writer: &mut Writer) {
        writer.var_u32(self.game_rules.len() as u32);
        self.game_rules.iter().for_each(|game_rule| game_rule.write(writer));
    }

    fn read(reader: &mut Reader) -> Self {
        Self { game_rules: (0..reader.var_u32()).map(|_| GameRule::read(reader)).collect() }
    }
}
