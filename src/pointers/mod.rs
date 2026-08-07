// Since this module is not pub, these docs comments are not
// going to be seen in the lib documentation
// No pointer putting //! here
// However if I did, /// here, it would work. But its best to add these
// in the document itself
pub mod lib_ptr_a;
pub mod lib_ptr_a2;
pub mod lib_ptr_b;

pub use lib_ptr_a::*;
pub use lib_ptr_a2::*;
pub use lib_ptr_b::*;
