trait Compressor {
    fn compress(self, fileName: String);
}
struct JpegCompressor;
impl Compressor for JpegCompressor {
    fn compress(self, fileName: String) {
        println!("JPEG compressed");
    }
}

trait Filter {
    fn filter(self, fileName: String);
}
struct BlackNWhiteFilter;
impl Filter for BlackNWhiteFilter {
    fn filter(self, fileName: String) {
        println!("Filter applied")
    }
}
struct ImageStorage<C, F> {
    compressor: C,
    filter: F,
}
impl<C: Compressor, F: Filter> ImageStorage<C, F> {
    pub fn new(compressor: C, filter: F) -> Self {
        Self {
            compressor: compressor,
            filter: filter,
        }
    }
    pub fn store(self, fileName: String) {
        self.compressor.compress(fileName.clone());
        self.filter.filter(fileName);
    }
}
fn main() {
    let image_storage = ImageStorage::new(JpegCompressor, BlackNWhiteFilter);
    image_storage.store("a file in C".to_string());
}
