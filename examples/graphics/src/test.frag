#version 430 core

layout(location = 0) out vec4 o_col;

layout(location = 0) in vec2 v_uv;

void main()
{
    o_col = vec4(v_uv, 1.f, 0.f);
}
