#version 430 core

layout(location = 0) in vec2 v_uv;
layout(location = 1) in vec4 v_col;

layout(set = 0, binding = 0) uniform sampler2D g_tex;

layout(location = 0) out vec4 o_col;

float linear_from_gamma(float x)
{
    const float lo = x/12.92f;
    const float hi = pow((x + 0.055f)/1.055f, 2.4f);
    return (x < 0.04045f) ? lo : hi;
}
vec4 linear_from_gamma(vec4 c)
{
    return vec4(
        linear_from_gamma(c.x),
        linear_from_gamma(c.y),
        linear_from_gamma(c.z),
        c.w);
}

float gamma_from_linear(float x)
{
    const float lo = x*12.92f;
    const float hi = 1.055f*pow(x, 1.f/2.4f) - 0.055f;
    return (x < 0.0031308f) ? lo : hi;
}
vec4 gamma_from_linear(vec4 c)
{
    return vec4(
        gamma_from_linear(c.x),
        gamma_from_linear(c.y),
        gamma_from_linear(c.z),
        c.w);
}

void main() {
    o_col = v_col*gamma_from_linear(texture(g_tex, v_uv));
}
