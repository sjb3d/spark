#version 430 core

layout(location = 0) in vec2 a_pos;
layout(location = 1) in vec2 a_uv;
layout(location = 2) in vec4 a_col;

layout(push_constant) uniform BatchData {
    vec2 size_in_points_rcp;
} g_batch;

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) out vec2 v_uv;
layout(location = 1) out vec4 v_col;

void main() {
    gl_Position = vec4(a_pos*g_batch.size_in_points_rcp*2.0 - 1.0, 0, 1);
    v_uv = a_uv;
    v_col = a_col;
}
