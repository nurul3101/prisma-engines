use std::borrow::Cow;

use crate::FieldId;

static RESERVED_NAMES: &[&str] = &["PrismaClient"];
static RESERVED_NAME: &str = "This model has been renamed to 'RenamedPrismaClient' during introspection, because the original name 'PrismaClient' is reserved.";

#[derive(Default)]
pub struct Model<'a> {
    name: Cow<'a, str>,
    database_name: Option<Cow<'a, str>>,
    documentation: Option<Cow<'a, str>>,
    primary_key: Vec<FieldId>,
}

impl<'a> Model<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        let name = name.into();

        let (name, database_name, documentation) = match super::sanitize_string(&name) {
            Some(sanitized) => (Cow::from(sanitized), Some(name), None),
            None if RESERVED_NAMES.contains(&&*name) => (
                Cow::from(format!("Renamed{name}")),
                Some(name),
                Some(Cow::from(RESERVED_NAME)),
            ),
            None => (name, None, None),
        };

        Self {
            name,
            database_name,
            documentation,
            ..Default::default()
        }
    }

    pub fn set_primary_key(&mut self, ids: Vec<FieldId>) {
        self.primary_key = ids;
    }
}
