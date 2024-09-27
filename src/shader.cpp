void plot_point(__global unsigned char* buffer, uint width, uint height, uint x, uint y, unsigned char r, unsigned char g, unsigned char b) {
    // Ensure the coordinates are within bounds
    if (x >= width || y >= height) return;

    // Calculate the index in the buffer
    uint index = (y * width + x) * 4; // Each pixel has 4 components (RGBA)

    // Set the RGB values and alpha to 255
    buffer[index] = r;     // R
    buffer[index + 1] = g; // G
    buffer[index + 2] = b; // B
    buffer[index + 3] = 255; // A (Alpha)
}

__kernel void generate(__global unsigned char* buffer, unsigned char scalar) {
    const uint width = 1000;
    const uint height = 1080;

    uint gid = get_global_id(0);
    uint col = gid % width;
    uint line = gid / width;
    uint result = gid;

    // Set alpha to 255
    if ((gid+1) % 4 == 0) {
        buffer[get_global_id(0)] = 255;
        return;
    }

    float f_gid = (float) gid;
    float pixel_x = (unsigned char) (sin((float) col / 40) * 255);
    float pixel_y = (unsigned char) (sin((float) line / 40) * 255);

    plot_point(buffer, width, height, col + scalar, line, (unsigned char) col, 0, (unsigned char) line);
}
