use crate::{
    DamageType,
    MagicType,
    PhysicalType
};

pub struct Action {
    pub name: String,
    pub points_required: u32,
    pub damage_type: DamageType,
    pub magic_type: MagicType,
    pub physical_type: PhysicalType
}
