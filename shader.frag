#version 450

layout(location = 0) out vec4 outColor;
layout(location = 0) in vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform texture2D t_Color;
layout(set = 0, binding = 1) uniform sampler s_Color;

void main() {
    vec2 uv = v_TexCoord.xy;
    uv.x *= 4.0 / 3.0; // this is the aspect ratio
    outColor = vec4(texture(sampler2D(t_Color, s_Color), uv).rgb, 1.0);
}