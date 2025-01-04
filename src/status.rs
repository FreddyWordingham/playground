use bevy::prelude::*;

use crate::health::Health;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_healthbar);
    }
}

pub fn display_healthbar(mut children: Query<(&Parent, &mut Text2d)>, query: Query<&Health>) {
    for (parent, mut child_text) in &mut children {
        let parent_health = query.get(parent.get());
        if let Ok(health) = parent_health {
            child_text.0 = format!("health {}/{}", health.value, health.max_value);
        }
    }
}
