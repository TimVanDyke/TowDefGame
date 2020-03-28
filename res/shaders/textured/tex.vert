#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 2) in vec2 Uv;

//uniform vec3 CameraPos;
uniform mat4 View;
uniform mat4 Projection;

out VS_OUTPUT {
    vec2 Uv;
} OUT;

void main()
{
    gl_Position = Projection * View * vec4(Position, 1.0);
    OUT.Uv = Uv;
}