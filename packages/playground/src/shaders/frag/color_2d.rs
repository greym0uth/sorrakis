pub const SHADER: &str = r#"
    precision mediump float;

    uniform vec4 uColor;

    void main() {
        gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.b, uColor.a);
    }
"#;
