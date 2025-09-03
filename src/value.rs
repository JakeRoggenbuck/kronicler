use super::capture::Epoch;

pub enum Value {
    Epoch(Epoch),
    Name([u8; 64]),
}
