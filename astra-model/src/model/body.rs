use super::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
#[cfg_attr(feature = "godot", derive(gdnative::ToVariant))]
pub struct Body {
    pub features: BodyFeatures,
    pub center_of_mass: Vector3,
    #[cfg(not(feature = "godot"))]
    pub joints: HashMap<JointType, Joint>,
    #[cfg(feature = "godot")]
    pub joints: gdnative::Dictionary,
    pub hand_poses: u32,
    pub id: u8,
    pub status: BodyStatus,
}

impl<'a> From<&'a sys::_astra_body> for Body {
    fn from(body: &sys::_astra_body) -> Self {
        Body {
            features: body.features.into(),
            center_of_mass: body.centerOfMass.into(),
            joints: convert_joints(body.joints),
            hand_poses: 0,
            id: body.id,
            status: body.status.into(),
        }
    }
}
