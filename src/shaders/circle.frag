#version 140
out vec4 color;
in vec2 color_attr;
in vec2 v_tex_coords;

uniform sampler2D tex;

void main() {
     color = texture(tex, v_tex_coords);
}

