#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 tc;

uniform mat4 Model;
uniform mat4 View;
uniform mat4 Projection;

out VS_OUTPUT {
    vec2 tc;
} OUT;

void main()
{
    // gl_Position = Projection * View * vec4(Position, 1.0);
    gl_Position = Projection * Model * vec4(Position, 1.0);
    // gl_Position = Projection * View * vec4(Position, 1.0) * Model;
    OUT.tc = tc;
}