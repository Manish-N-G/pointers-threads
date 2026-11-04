pub trait FileMetadata {
    // pub is automatic
    fn is_readable(&self) -> bool;
    fn is_writable(&self) -> bool;
    fn exists(&self) -> bool;
} 

impl FileMetadata for std::path::Path {
    fn is_readable(&self) -> bool {
        std::fs::File::open(&self)
            .is_ok()
    }

    fn is_writable(&self) -> bool {
        std::fs::metadata(self)
            .map( |data| !data.permissions().readonly() )
            .unwrap_or(false)
    }

    fn exists(&self) -> bool {
        // std::fs::metadata(self)
        //     .map( |data| { data.is_file() })
        //     .unwrap_or(false)
        
        //file! is for this file
        self.exists()
    }
}
