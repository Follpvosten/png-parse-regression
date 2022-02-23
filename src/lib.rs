#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn png017_works() {
        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
        let decoder = png017::Decoder::new(File::open("Asteroid_4.png").unwrap());
        let mut reader = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        reader.next_frame(&mut buf).unwrap();
    }

    #[test]
    fn png016_works() {
        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
        let decoder = png016::Decoder::new(File::open("Asteroid_4.png").unwrap());
        let (_info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        reader.next_frame(&mut buf).unwrap();
    }
}
