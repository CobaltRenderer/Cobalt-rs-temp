// Copyright (c) 2026 Maptek Pty Ltd
// Licensed under the MIT License

// Basic shader to draw triangles, with an option to 2D rotate around origin
// and linear -> SRGB colorspace conversion

// State value that sets triangle rotation
uniform float rotation;

// Input vertex attributes, semantic name is what renderer uses 
struct VSInput {
    float2 position : position;
    float3 color : color;
};

// Output vertex attributes, also input for fragment shader
struct VSOutput {
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

// Function to convert color space and have prettier triangle
float3 linearToSrgb(float3 lin) {
    float3 srgb;
    srgb.x = (lin.x <= 0.0031308) ? lin.x * 12.92 : 1.055 * pow(lin.x, 1.0 / 2.4) - 0.055;
    srgb.y = (lin.y <= 0.0031308) ? lin.y * 12.92 : 1.055 * pow(lin.y, 1.0 / 2.4) - 0.055;
    srgb.z = (lin.z <= 0.0031308) ? lin.z * 12.92 : 1.055 * pow(lin.z, 1.0 / 2.4) - 0.055;
    return srgb;
}

// Vertex shader main function
VSOutput vertex(VSInput IN)
{
    // Rotate triangle
	float s, c;
	sincos(rotation, s, c);
	float2x2 rotMatrix = float2x2(c, -s, s, c);
	IN.position.xy = mul(rotMatrix, IN.position);

    VSOutput OUT;
    OUT.position = float4(IN.position, 0.5f, 1.0f);
    OUT.color = IN.color;
    return OUT;
}

// Fragment shader main function
float4 fragment(VSOutput IN) : SV_TARGET0
{
    return float4(linearToSrgb(IN.color), 1.0f);
}
