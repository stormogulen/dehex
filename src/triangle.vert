#version 330 core

layout (location = 0) in Vec3 Position;

void main() {

     gl_Position = vec4(Position, 1.0);
}