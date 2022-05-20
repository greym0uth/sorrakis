pub const VERT: &str = r#"
attribute vec4 position;

uniform vec4 u_Scale;
uniform vec4 u_Translation;
uniform float u_Rotation;

vec4 Z(vec4 v, float a)
{
    vec4 vo = v; float c = cos(a); float s = sin(a);
    v.x = c * vo.x - s * vo.y;
    v.y = s * vo.x + c * vo.y;
    return v;
}

void main() {
    vec4 scaled_position = position * u_Scale;

    float x = scaled_position.x;
    float y = scaled_position.y;

    vec4 rotated_position = Z(scaled_position, u_Rotation);


    gl_Position = rotated_position + u_Translation;
}
"#;

pub const FRAG: &str = r#"
precision mediump float;

uniform vec4 u_Color;

void main() {
    gl_FragColor = u_Color;
}
"#;

//vec4 rotation = vec4(cos(u_Rotation), sin(u_Rotation) * -1.0, sin(u_Rotation), cos(u_Rotation));
