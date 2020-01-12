#version 450

layout(location = 0) out vec4 outColor1;
layout(location = 1) out vec4 outColor2;
layout(location = 0) in vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform texture2D t_Color;
layout(set = 0, binding = 1) uniform sampler s_Color;

void main() {
    vec2 uv = v_TexCoord.xy / 2.0 + 0.5;
    vec3 color = texture(sampler2D(t_Color, s_Color), uv-vec2(0.001,0.0)).rgb;
    outColor1 = vec4(color.rgb, 1.0);
    outColor2 = outColor1;
}