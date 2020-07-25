#version 430 core

layout(push_constant) uniform TestData {
    float angle;
    float x_scale;
} g_test;

out gl_PerVertex {
    vec4 gl_Position;
};

layout(location = 0) out vec2 v_uv;

void main()
{
    vec2 v = vec2(
        gl_VertexIndex == 1 ? .5f : -.5f,
        gl_VertexIndex == 2 ? .5f : -.5f);
    v_uv = v + .5f;

    float ca = cos(g_test.angle);
    float sa = sin(g_test.angle);
    v = vec2(v.x*ca + v.y*sa, -v.x*sa + v.y*ca);

    gl_Position = vec4(v.x*g_test.x_scale, v.y, 0.f, 1.f);
}
