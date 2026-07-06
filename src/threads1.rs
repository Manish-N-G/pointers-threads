// Note: This is again not the way we need to write the comments in
// the best way.
// Note: If we have doc comments in the lib and well as the module
// file, for the same module, then rust will insert the comments in
// order. I have left it on for a reason, but this is not the way
// we should be doing this.
// Also. Here //! will be for threads1, and we know that this file
// is not pub, and is the root file.
// This is the reason, why we have to use /// as it used for the actual
// lib th1 file written(tagged) just before we create lib_th1a module.

/// Lib th1a will have only simple functions to play with.
pub mod lib_th1a;
