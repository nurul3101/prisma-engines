use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use crate::{CompositeType, CompositeTypeId, FieldId, Model, ModelId, ScalarField};

#[derive(Default)]
pub struct PrismaSchema<'a> {
    models: Vec<Model<'a>>,
    types: Vec<CompositeType<'a>>,
    model_fields: Vec<(ModelId, ScalarField<'a>)>,
}

impl<'a> PrismaSchema<'a> {
    pub fn push_model(&mut self, model: Model<'a>) -> ModelId {
        self.models.push(model);

        ModelId(self.models.len() - 1)
    }

    pub fn push_type(&mut self, r#type: CompositeType<'a>) -> CompositeTypeId {
        self.types.push(r#type);

        CompositeTypeId(self.types.len() - 1)
    }

    pub fn push_model_field(&mut self, model_id: ModelId, field: ScalarField<'a>) {
        self.model_fields.push((model_id, field));
    }
}

impl<'a> Index<ModelId> for PrismaSchema<'a> {
    type Output = Model<'a>;

    fn index(&self, index: ModelId) -> &Self::Output {
        &self.models[index.0]
    }
}

impl<'a> IndexMut<ModelId> for PrismaSchema<'a> {
    fn index_mut(&mut self, index: ModelId) -> &mut Self::Output {
        &mut self.models[index.0]
    }
}
