#version 450

layout(location = 0) out vec4 outColor1;
layout(location = 1) out vec4 outColor2;
layout(location = 0) in vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform texture2D t_Color;
layout(set = 0, binding = 1) uniform sampler s_Color;

vec2 resolution = vec2(800., 600.);
vec4 live = vec4(0., 1., 0., 1.);
vec4 dead = vec4(0., 0., 0., 1.);
vec4 blue = vec4(0., 0., 1., 1.);

void main() {
    vec2 uv = v_TexCoord.xy / 2. + .5;
    vec2 pixel = vec2(1., 1.) / resolution;
    float sum = 0.;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(-1., -1.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(-1., 0.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(-1., 1.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(1., -1.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(1., 0.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(1., 1.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(0., -1.)).g;
    sum += texture(sampler2D(t_Color, s_Color), uv + pixel * vec2(0., 1.)).g;
    vec4 me = texture(sampler2D(t_Color, s_Color), uv);

    if (me.g <= .1) {
        if ((sum >= 2.9) && (sum <= 3.1)) {
            outColor1 = live;
        } else if (me.b > .01) {
            outColor1 = vec4(0., 0., max(me.b - .006, .001), 1.);
        } else {
            outColor1 = dead;
        }
    } else {
        if ((sum >= 1.9) && (sum <= 3.1)) {
            outColor1 = live;
        } else {
            outColor1 = blue;
        }
    }
    outColor2 = outColor1;
}