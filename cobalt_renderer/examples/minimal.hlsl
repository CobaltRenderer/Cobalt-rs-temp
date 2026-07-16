// Copyright (c) 2026 Maptek Pty Ltd
// Licensed under the MIT License

// Basic shader to draw triangles

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

// Vertex shader main function
VSOutput vertex(VSInput IN)
{
    VSOutput OUT;
    OUT.position = float4(IN.position, 0.5f, 1.0f);
    OUT.color = IN.color;
    return OUT;
}

// Fragment shader main function
float4 fragment(VSOutput IN) : SV_TARGET0
{
    return float4(IN.color, 1.0f);
}
