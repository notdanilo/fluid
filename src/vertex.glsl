#version 450

layout(location = 0) in vec3 position;
layout(location = 0) uniform mat4 projection;

void main(void) {
    gl_Position = projection * vec4(position, 1.0);
    gl_PointSize = 8.0;
}