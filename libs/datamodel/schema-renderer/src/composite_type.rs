use std::borrow::Cow;

#[derive(Default)]
pub struct CompositeType<'a> {
    name: Cow<'a, str>,
}
