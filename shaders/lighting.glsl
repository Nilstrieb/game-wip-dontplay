uniform sampler2D texture;
uniform float ambient;
uniform bool has_texture;

void main()
{
    // lookup the pixel in the texture
    vec4 pixel = texture2D(texture, gl_TexCoord[0].xy);

    // multiply it by the color
    vec4 col = gl_Color;
    if (has_texture) {
        col = gl_Color * pixel;
    }
    gl_FragColor = vec4(col.x * ambient, col.y * ambient, col.z * ambient, col.w);
}