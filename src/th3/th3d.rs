// DYN type
// this module is mean to understand how to dyn dispatch works.
// This is similar to the "Type Erasure" concept in other languages.
// And we will see how this is handles in rust.
//

// more on type erasure soon.

pub trait Music{
    fn sound(&self) {
        println!("Do ray me");
    }

    fn instrument(&self) {}
}

// mirror of Music. Think of this as a Vtable in rust, and this is quite similar
// to the way rust does this.
pub struct MusicFunctions {
    sound_wrapper: unsafe fn(std::ptr::NonNull<()>),
    instrument_wrapper: unsafe fn(std::ptr::NonNull<()>),
    // This list will grow as the methods increase
}

pub struct Rock;
impl Music for Rock {
    fn sound(&self) {
        println!("Rock on!!!");
    }

    fn instrument(&self) {
        println!("Instrument - Guitar: \u{1F3B8}");
    }
}

pub struct Pop;
impl Music for Pop {
    fn sound(&self) {
        println!("Yeee heee..");
    }
}

pub struct Classic;
impl Music for Classic{}


// We try to make this as a kinda reference type
// we want this to have the same lifetime founds as 
// the struct itself.
pub struct AnyMusic<'a> {
    // This is just a raw pointers that pointers to something that is known
    // to not be null. We cant do NonNull<impl Music>. This is not supported
    // However, we can represent that using unit type.
    data: std::ptr::NonNull<()>, 

    // This is a function type that is meant to take in our data type.
    // If will have to be unsafe to avoid compiler safety checks.
    // We can use thunk or wrapper, do denote our type. This is the general
    // we should use our type. as we wrap our methods.
    // sound_wrapper: unsafe fn(std::ptr::NonNull<()>),
    // NOTE:
    // However, this would not be dynamic as we would have to rely on 
    // creating multiple wrapper for each method we will try to call
    // for that trait.Hence its better to optimize our field

    // this works, but we will change the & type
    // functions: &'a MusicFunctions,

    // We take &'static because of the way rust creates this values
    // during compile time. Meaning, in our situation, since rust knows
    // that the closures passed in the function field are not capturing
    // any variables in it, we can annotate this with the 'static
    // lifetime. This will then create a static memory value for this 
    // type during compile time. This is feature called constant promotion.
    functions: &'static MusicFunctions,
    
    // we are making sure that this function has some lifetime bounds
    _mkr: std::marker::PhantomData<&'a ()>,
}

// This is where we can define some conditions so that we are able to
// make sure we have all the necessary information that we need about the
// type we will be pointing to via our NonNull ptr.
impl<'a> AnyMusic<'a> {
    // val needs to be &'a T, as we pass a ref to from for null ptr
    pub fn new<T: Music>(val: &'a T) -> Self {
        // let v = std::ptr::NonNull::from(val); // will become NonNull<T>,
        // will become NonNull<()>
        // However, out NonNull of () via cast pointers to the &T referenct in memory.
        // let v2: std::ptr::NonNull<()> = std::ptr::NonNull::from(val).cast(); 
        Self {
            data: std::ptr::NonNull::from(val).cast(),

            // we know that closure can implicitily be coerced to function pointers
            // as long as they don't capture anything.
            // we can directly call data.music, because data will be NonNull<()>,
            // music_type: |data| { data.music(); },
            // However, using cast again, that become NonNull<T> and then call
            // if music function on it after we dereference to &T
            // This is also a unsafe, operation, which is why out struct field is unsafe.
            // sound_wrapper: |data| unsafe { data.cast::<T>().as_ref() }.sound(),
            // NOTE: However, in order to optimize our type to be able to point
            // to the list of functions available in the trait, we will change the
            // way the field is called.

            // Since rust has created this field in static memory for MusicFunctions
            // We can pass a reference to it.
            functions: &MusicFunctions {
                sound_wrapper: |data| unsafe { data.cast::<T>().as_ref() }.sound() ,
                instrument_wrapper: |data| unsafe { data.cast::<T>().as_ref() }.instrument(),
                // This list will grow as the methods increase
            },

            _mkr: std::marker::PhantomData,
        }
    }

    // Now, once we have created our AnyMusic type, we can call the sound function
    // as we defined that function for type of Music. 
    pub fn sound(&self) {
        // For older implementation, this was working, now we have to update the code
        // let fun = self.sound_wrapper;
        // unsafe { fun(self.data); }
        // unsafe { (self.sound_wrapper)( self.data) }
        unsafe { ( self.functions.sound_wrapper )( self.data ) }
    }

    // similarly we have this implementation for our instrument function
    pub fn instrument(&self) {
        unsafe { ( self.functions.instrument_wrapper )( self.data ) }
    }
}

pub fn normal_music_impl() {
    println!("Normal implementation--");
    let genre = Rock;
    genre.sound();
    Pop.sound();
    println!();
}


pub fn erasure_music_impl() {
    println!("Erasure implementation or our version of Dyn dispatch--");
    AnyMusic::new(&Rock).sound();

    let mut any_music = AnyMusic::new(&Rock);
    any_music.instrument();

    any_music = AnyMusic::new(&Pop);
    any_music.sound();
    any_music.instrument();

    any_music = AnyMusic::new(&Classic);
    any_music.sound();
    any_music.instrument();
    println!();
}
