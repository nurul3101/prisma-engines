mod composite_type;
mod model;
mod prisma_schema;
mod scalar_field;

pub use composite_type::CompositeType;
pub use model::Model;
pub use prisma_schema::PrismaSchema;
pub use scalar_field::{PrismaType, ScalarField, ScalarFieldType};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone, Copy, PartialEq)]
pub struct ModelId(usize);

#[derive(Clone, Copy, PartialEq)]
pub struct FieldId(usize);

#[derive(Clone, Copy, PartialEq)]
pub struct CompositeTypeId(usize);

fn sanitize_string<'a>(s: &'a str) -> Option<String> {
    static RE_START: Lazy<Regex> = Lazy::new(|| Regex::new("^[^a-zA-Z]+").unwrap());
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("[^_a-zA-Z0-9]").unwrap());

    let needs_sanitation = RE_START.is_match(s) || RE.is_match(s);

    if needs_sanitation {
        let start_cleaned: String = RE_START.replace_all(s, "").parse().unwrap();
        let sanitized: String = RE.replace_all(start_cleaned.as_str(), "_").parse().unwrap();

        Some(sanitized)
    } else {
        None
    }
}
