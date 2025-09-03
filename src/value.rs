use super::capture::Epoch;

#[derive(PartialEq, Debug)]
pub enum Value {
    Epoch(Epoch),
    Name([u8; 64]),
}
