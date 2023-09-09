use super::input::Input;

#[derive(Debug)]
pub struct ParserError<'input> {
    pub(super) input: Input<'input>,
}

impl<'input> ParserError<'input> {
    pub fn new(input: Input<'input>) -> Self {
        Self { input }
    }
}
