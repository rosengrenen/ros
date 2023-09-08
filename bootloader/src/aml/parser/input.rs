#[derive(Clone, Copy, Debug)]
pub struct Input<'input> {
    bytes: &'input [u8],
}

impl<'input> Input<'input> {
    pub(super) fn new(bytes: &'input [u8]) -> Self {
        Self { bytes }
    }

    pub(super) fn as_slice(&self) -> &[u8] {
        self.bytes
    }

    pub(super) fn take_split(&self, index: usize) -> (Self, Self) {
        let (left, right) = self.bytes.split_at(index);
        (Self::new(left), Self::new(right))
    }
}
