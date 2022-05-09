use std::{borrow::Cow, fmt};

use crate::FieldId;

static COMMENTED_OUT_FIELD: &str = "This field was commented out because of an invalid name. Please provide a valid one that matches [a-zA-Z][a-zA-Z0-9_]*";
static EMPTY_TYPE_DETECTED: &str = "Nested objects had no data in the sample dataset to introspect a nested type.";

pub enum PrismaType<'a> {
    Int,
    BigInt,
    Float,
    Boolean,
    String,
    DateTime,
    Json,
    Bytes,
    Decimal,
    Composite(Cow<'a, str>),
    Unsupported(Cow<'a, str>),
}

impl<'a> PrismaType<'a> {
    pub fn composite(name: impl Into<Cow<'a, str>>) -> Self {
        Self::Composite(name.into())
    }

    pub fn unsupported(name: impl Into<Cow<'a, str>>) -> Self {
        Self::Unsupported(name.into())
    }
}

pub struct ScalarFieldType<'a> {
    prisma: PrismaType<'a>,
    native: Option<Cow<'a, str>>,
}

impl<'a> ScalarFieldType<'a> {
    pub fn new(prisma: PrismaType<'a>) -> Self {
        Self { prisma, native: None }
    }

    pub fn native_type(&mut self, native: impl Into<Cow<'a, str>>) {
        self.native = Some(native.into());
    }
}

pub struct ScalarField<'a> {
    name: Cow<'a, str>,
    r#type: ScalarFieldType<'a>,
    database_name: Option<Cow<'a, str>>,
    id_field: Option<FieldId>,
    documentation: Option<Cow<'a, str>>,
    default_value: Option<Cow<'a, str>>,
    is_optional: bool,
    is_array: bool,
    is_commented_out: bool,
}

impl<'a> ScalarField<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, r#type: ScalarFieldType<'a>) -> Self {
        let name = name.into();

        let (name, database_name) = match super::sanitize_string(&name) {
            Some(sanitized) => (Cow::from(sanitized), Some(Cow::from(name))),
            None => (name, None),
        };

        let documentation = match database_name.as_ref() {
            Some(name) if name.is_empty() => Some(Cow::from(COMMENTED_OUT_FIELD)),
            _ => None,
        };

        let is_commented_out = documentation.is_some();

        Self {
            name,
            r#type,
            database_name,
            documentation,
            is_optional: false,
            is_array: false,
            is_commented_out,
            id_field: None,
            default_value: None,
        }
    }

    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn database_name(&'a self) -> Option<&'a str> {
        self.database_name.as_deref()
    }

    pub fn set_optional(&mut self, is_optional: bool) {
        self.is_optional = is_optional;
    }

    pub fn set_array(&mut self, is_array: bool) {
        self.is_array = is_array;
    }

    pub fn set_name(&mut self, name: impl Into<Cow<'a, str>>) {
        self.name = name.into();
    }

    pub fn set_database_name(&mut self, name: impl Into<Cow<'a, str>>) {
        self.database_name = Some(name.into());
    }

    pub fn set_default_value(&mut self, value: impl Into<Cow<'a, str>>) {
        self.default_value = Some(value.into())
    }

    pub fn push_docs(&mut self, docs: impl Into<Cow<'a, str>>) {
        let docs = docs.into();

        match self.documentation.as_mut() {
            Some(existing) => {
                let existing = existing.to_mut();
                existing.push('\n');
                existing.push_str(&docs);
            }
            None => {
                self.documentation = Some(docs);
            }
        }
    }
}
