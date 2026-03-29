#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CircularCooldownMaterial {
    color: vec4<f32>,
    progress: f32,
}

@group(1) @binding(0)
var<uniform> material: CircularCooldownMaterial;

@fragment
fn fragment(
    in: UiVertexOutput,
) -> @location(0) vec4<f32> {
    // UV is from 0 to 1. Center is 0.5.
    let center = vec2<f32>(0.5, 0.5);
    let uv = in.uv - center;
    
    // Calculate distance from center to handle rounding (circle mask)
    let dist = length(uv);
    if (dist > 0.5) {
        discard;
    }

    // Calculate angle: atan2(x, y) gives clockwise angle starting from top (0)
    let angle = atan2(uv.x, uv.y);
    
    // Convert angle from range [-PI, PI] to [0, 1]
    let normalized_angle = (angle / 6.28318530718) + 0.5;
    
    // We want the shroud to disappear in a circular motion.
    // If progress is 1.0 (ready), shroud is 0.
    // If progress is 0.0 (just started), shroud is 1.0.
    // In the update system, we pass (1.0 - cooldown_fraction).
    
    if (normalized_angle < material.progress) {
        return material.color;
    } else {
        discard;
    }
}
