use super::*;
#[cfg(feature = "godot")]
use gdnative::ToVariant;
#[cfg(not(feature = "godot"))]
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "godot", derive(gdnative::ToVariant))]
pub struct Joint {
    pub joint_type: JointType,
    pub status: JointStatus,
    pub depth_position: Vector2,
    pub world_position: Vector3,
}

impl From<sys::_astra_joint> for Joint {
    fn from(joint: sys::_astra_joint) -> Self {
        Self {
            joint_type: joint.type_.into(),
            status: joint.status.into(),
            depth_position: joint.depthPosition.into(),
            world_position: joint.worldPosition.into(),
        }
    }
}
#[cfg(not(feature = "godot"))]
pub fn convert_joints(astra_joints: [sys::astra_joint_t; 19usize]) -> HashMap<JointType, Joint> {
    astra_joints
        .iter()
        .enumerate()
        .map(|(joint_type, joint)| (JointType::from(joint_type as u8), Joint::from(*joint)))
        .collect()
}
#[cfg(feature = "godot")]
pub fn convert_joints(astra_joints: [sys::astra_joint_t; 19usize]) -> gdnative::Dictionary {
    let mut dict = gdnative::Dictionary::new();
    for (joint_type, joint) in astra_joints.iter().enumerate() {
        dict.set(
            &JointType::from(joint_type as u8).to_variant(),
            &Joint::from(*joint).to_variant(),
        );
    }
    dict
}
