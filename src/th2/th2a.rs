// #![allow(unused)]
use std::cell::UnsafeCell;
pub use self::modcell::MyCell;
pub use self::modrefcell::MyRefCell;
pub use self::modrc::MyRc;
// same as above
// use crate::th2::th2a::modcell::MyCell;
// use crate::th2::th2a::modrefcell::MyRefCell;
// use crate::th2::th2a::modrc::MyRc;
use std::ops::{ Deref, DerefMut };
// Just tells us that we point to something that is not a null value
use std::ptr::NonNull;
// PhantomData adds certain characteristics to struct that make it behave as a certain way
// think of when we need to consider auto traits to a struct type depending on is fields
// to where adhere to a traits requirements.
// This plays a bit role in raw pointers for example, variance of a type or for autotrait 
// requirements. More is said about PhantomData later
use std::marker::PhantomData;

// I using this test function. Not the same as a test module used for cargo test
pub fn testing_mycell() {
    // this variable doenst need to be mutable as it mutates through interior mutability
    let ce = MyCell::new(3u8);
    println!("MyCell u8 val get is {}", ce.get() );
    ce.set(8u8);
    println!("MyCell u8 after val set is {}", ce.get() );
    println!("MyCell replace {}", ce.replace(9u8) );
    println!("MyCell get new val is {}", ce.get() );

    let mut c2 = MyCell::new(String::from("hello there"));
    // since we used trait implementing copy only for get,
    // the print will not allow us to get the refernece back
    // println!("c2 is {:?}", c2.get());
    // This is why I feel we need to put this in the struct directly
    // However we can use mut value
    *c2.get_mut() = String::from("new"); // works but doesnt server the purpose of interior
                                         // mutability
    let mut c3 = std::cell::Cell::new(String::from("hello from cell"));
    // in std library, we cant use get() for non copy trait types too
    // c3.get();
    *c3.get_mut() = String::from("he"); // there is a method for get_mut
}

pub fn testing_myrefcell() {
    let rce = MyRefCell::new(11u8);
    // can have many borrows but only a single borrow_mut
    println!("MyCell u8 borrow is {:?}", rce.borrow() );
    // can have many borrows is its in the same thread
    println!("MyCell u8 borrow again is {:?}", rce.borrow() );
    // rce.borrow_mut(); // this doesnt work. It will compile but will panic during run time
    #[allow(unused)]
    let rce = MyRefCell::new(10u8);
    // drop(rce); // not required as this is already shadowed dropped.
    let rce = MyRefCell::new(10u8);
    let x = rce.borrow();
    drop(x);
    // drop(x); // Another drop x will cause this to throw an error
    println!("MyCell u8 try borrow is possible? {:?}", rce.try_borrow() );
    println!("MyCell u8 try borrow again is possible? {:?}", rce.try_borrow() );
    println!("MyCell u8 try borrow mut is possible? {:?}", rce.try_borrow_mut() );
    let mut x = rce.borrow_mut();
    *x+=4;
    *x+=9;
    println!("MyCell u8 try borrow after is possible? {:?}", rce.try_borrow() );
    println!("MyCell u8 try borrow mut after is possible? {:?}", rce.try_borrow_mut() );
    println!("MyCell u8 try borrow mut after is {:?}", *x );
}

pub fn testing_myrc_cell() {
    let myrcval = MyRc::new(33u8);
    let otherrc = MyRc::clone(&myrcval);
    let newotherrc = MyRc::clone(&myrcval);
    println!("MyRC strong count for val: {}", myrcval.get_strong() );
    println!("MyRC strong count for val cloned: {}", otherrc.get_strong() );
    drop(newotherrc);
    println!("MyRC strong count for val cloned after 1 cloned dropped: {}", otherrc.get_strong() );
    drop(otherrc);
    println!("MyRC strong count for val cloned after 2 cloned dropped: {}", myrcval.get_strong() );
    drop(myrcval);
    // println!("MyRC after all dropped for strong count: {}", myrcval.get_strong() ); // this cont
    // compile as value is already dropped
}

// I am able to work on it cause the test functions are public
// Just my version of the Rc Type
pub mod modrc {
    use super::*;
    // We know that if we have to drop the MyRc type, if we have all the fields inside
    // it, it would mean that we have to drop the whole struct. The point of having an 
    // RC type is to be able to have many shared reference to a value. This reference could 
    // increases when we clone the value. For this sake, its better to create a inner type that
    // hold this values that doesnt get affected when we call the drop value on RC. This way
    // the drop will look at the num of references in inner and then decide whether we need to
    // remove this rc type.
    // This is done be having a pointer that points to the Inner type that has
    // all the actual values. In this case NonNull from MyRC that points to MyRcInner
    struct MyRcInner<T> {
        value: T,
        // initially was Unsafecell, but change to MyCell cause 
        // there were too many unsafe calls in the code.
        strong_count: MyCell<usize>,
    }
    pub struct MyRc<T> {
        // cant use & here cause we will have to deal with
        // multiple lifetimes which would make this a headache
        // to even try to implement. Initially we 1st try to use a raw const ptr
        // As the *const ptr needs to be dropped appropriately, we
        // had to find a way to do this. We cant use Box::from_raw here
        // as it requires a *mut ptr. But having a * mut ptr
        // gives us lesser functionality for our data type.
        // inner: *const MyRcInner<T>, // complications to learn how to drop *const ptr
        // inner: *mut MyRcInner<T>, // possilbe, but we have fewer possibilities as we progress
        // also we need to avoid direct mut access to a an inner type I feel
        // The way to get around this is by using a NonNull ptr type
        // that allows to handle this. However, its important to know that
        // NonNull data type is also more dangerous. Read more about how NonNull
        // and Option of NonNull work. Also read how PhantomData work.
        // NonNull gives better optimization over *mut for the compiler.
        // This touches the concepts how covarience/varience work.
        // Old: inner: *const MyRcInner<T>, // inner: *mut MyRcInner<T>,
        // Also, NonNull is !Send and !Sync so it doesnt allow for
        // mutli threading. Also MyCell which uses UnsafeCell does the same.
        inner: NonNull<MyRcInner<T>>,
        // NEXT: IMPORTANT:
        // We implement a PhantomData type of MyRc that points to MyRcInner.
        // The reason we do this is because of how phantom data works.
        // For example, when we have MyRc that only has a pointer (NonNull)
        // to the value we want, the rust compiler doesnt know what to do when
        // MyRc is dropped. In general, we have to provide a way to handle that when
        // rust drops MyRc, so in a sence, the dropchek throught phantomdata 
        // also checks and tried to drop (treats it as if it could) MyRcInner
        // (even when not possible directly as phantomdata doesnt really hold any data)
        // This is important when we have generic type with references which
        // have lifetimes associated with them. The reference types will make sure
        // that rust compiler checks if the value T: which holds a reference,
        // is not dropped before MyRc gets dropped. This is a protection we have
        // in Dropck to ensure that this does not happen. Rust makes this possible
        // with the use of PhantomData. We can think of PhantomData as a type that
        // owns a value or reference to a value when in actuality is doesnt use
        // any space for allocation. This is read by the compiler as; huh, okay 
        // you have a owned (which it doesnt but seems to for the compiler)
        // instance of T, so I will also check inside MyRcInner if that value
        // still holds good during dropck for example and or lifetimes and references.
        // This is a big topic to get into, but we can read about it to get a better
        // idea of what this does.
        _markr: PhantomData<MyRcInner<T>>,
    }
    
    impl<T> MyRc<T> {
        pub fn new(val: T) -> Self {
            // we 1st create the MyRcInner type
            let inner = Box::new(MyRcInner {value: val, strong_count: MyCell::new(1)});
            // here, into_raw gives us a raw pointer to the value.
            // this value is consumed when we get the raw pointer.
            // in such a situation, this value does not get deallocated
            // as is not affected when this scope ends, else the box 
            // could have been dropped at the end of this scope.
            // Self { inner: Box::into_raw(inner) }
            // Self { inner: NonNull::new( Box::into_raw(inner) ).unwrap() } // Works OR
            // Safety arguement: Box does not give us a null pointer
            unsafe { 
                Self { 
                    // inner: NonNull::new( Box::into_raw(inner) ), // gives option type 
                    inner: NonNull::new_unchecked( Box::into_raw(inner) ), // doesnt give option
                    _markr: PhantomData,
                }
                
            }
        }
        // this could be changed in the way its written
        pub fn get_strong(&self) -> usize {
            // unsafe { (*self.inner).strong_count.get() } // doesnt not work

            // this compiles, but this takes self as inner and not reference directly.
            // but since we take a reference of self for get_strong, I imagine that
            // it just takes self of &self. Also inner is just a wrapper what holds
            // a raw pointer. This works, but I didnt see any issues when testing
            // unsafe { (*self.inner.as_ptr()).strong_count.get() }

            unsafe { self.inner.as_ref().strong_count.get() }
        }
    }
    impl<T> Clone for MyRc<T> {
        fn clone(&self) -> Self {
            //let inner = unsafe { &*self.inner };
            let inner = unsafe { self.inner.as_ref() };
            let val:usize = inner.strong_count.get();
            inner.strong_count.set ( val.wrapping_add(1) );
            Self { inner: self.inner, _markr: PhantomData, }
        }
    }
    impl<T> Deref for MyRc<T> {
        type Target = T;
        fn deref(&self) ->&Self::Target {
            //unsafe { &(*self.inner).value }
            unsafe { &self.inner.as_ref().value }
        }
    }
    impl<T> Drop for MyRc<T> {
        fn drop(&mut self) {
            //let inner = unsafe { &(*self.inner) };
            let inner = unsafe { self.inner.as_ref() };
            match inner.strong_count.get() {
                x if x>1 => inner.strong_count.set( x-1 ),
                // drop value here
                _ => { // x 
                    // drop(inner);
                    unsafe { drop(Box::from_raw(self.inner.as_ptr())) }
                }, 
            }
        }
    }
}

// Simple implementation of RefCell type. This does similar things like
// RefCell and works more in compile time than run time.
pub mod modrefcell {
    use super::*;
    use RefState::*; // to have values locally in this module.
    #[derive(Debug, Copy, Clone, PartialEq)]
    enum RefState {
        Unshared,
        Shared(usize),
        Exclusive,
    }
    #[derive(Debug)]
    pub struct MyRefCell<T> {
        value: UnsafeCell<T>,
        references: MyCell<RefState>,
    }
    impl<T> MyRefCell<T> {
        pub fn new(val:T) -> Self {
            Self {
                value: UnsafeCell::new(val),
                references: MyCell::new(Unshared),
            }
        }
        // they all works but for mutable values. we have to correct this.
        // We could have implemented it though Cell of isize, in order to
        // counter the mutable value of MyRefCell type.
        // pub struct MyRefCell<T> {
        //     value: UnsafeCell<T>,
        //     references: isize,
        // }
        // pub fn test_borrow(&mut self) -> Option<&T> {
        //     if self.references >= 0 {
        //         self.references += 1;
        //         unsafe {
        //             Some( &*self.value.get() )
        //         }
        //     }
        //     else {
        //         panic!("cant borrow as already borrowed as mutable");
        //     }PartialEq
        // }
        // pub fn test_borrow_mut(&mut self) -> Option<&mut T> {
        //     if self.references > 0 {
        //         panic!("cant borrow mut as borrowed as imutable");
        //     } else if self.references < 0 {
        //         panic!("cant borrow mut as allready borrowed as mutable");
        //     } else {
        //         self.references = -1;
        //         unsafe {
        //             Some( &mut *self.value.get() )
        //         }
        //     }
        // }
        // pub fn other_borrow(&mut self) -> &T {
        //     if self.references >= 0 {
        //         self.references += 1;
        //         unsafe {
        //             &*self.value.get()
        //         }
        //     }
        //     else {
        //         panic!("cant borrow as already borrowed as mutable");
        //     }
        // }
        // pub fn other_borrow_mut(&mut self) -> &mut T {
        //     if self.references > 0 {
        //         panic!("cant borrow mut as borrowed as imutable");
        //     } else if self.references < 0 {
        //         panic!("cant borrow mut as allready borrowed as mutable");
        //     } else {
        //         self.references = -1;
        //         unsafe {
        //             &mut *self.value.get()
        //         }
        //     }
        // }
        
        // Here, we take only a shared reference of self.
        // However, using our MyCell, which uses unsafe code under the hood,
        // we are able to use the set and get methods so that we are able 
        // be make it work. We use the MyCell along with UnsafeCell in order to
        // make this work. Perhaps we could have even used MyCell for the value field.

        // Instead of us giving out the shared reference and mutable
        // reference respectively for borrow and borrow_mut, its better to give
        // out this new type Ref and RefMut type. They are to make sure that we
        // can find a way in which it allows a type to be dropped.
        // As we are dropping values, this can then change its counting states,
        // which can influence the way in which the program runs.
        // This can then allow us to find a way to take the reference of a value
        // again and not have the compiler throw an error again.
        // This implementation takes place in run time as shown below.
        // Without the Ref Type/RefMut type, we will not have a clear way to
        // drop a value for a borrow/borrow_mut variable. This is important as we
        // Need to know how to monitor, update the reference states, drop the values
        // when necessary and Deref/Deref_mut that value. Calling this on the
        // Original MyRefCell will not be able to handle this as we cant call 
        // drop directly for an instance of borrow/borrow_mut for example.
        // This intermediate Struct helps us handles the states and values
        // available in the MyRefCell struct type.
        // NOTE: If something affects types, lifetimes and memory layout, its compile time
        // NOTE: If something allocates, mutates memory or executes logic, its run time
        pub fn borrow(&self) -> Ref<'_, T> {
            match self.references.get() {
                Unshared => self.references.set( Shared(1) ),
                Shared(x) => self.references.set( Shared(x.saturating_add(2)) ),
                _ => panic!("cant borrow as already borrowed as mutable"),
            }
            Ref { refcell: self }
        }
        pub fn borrow_mut(&self) -> RefMut<'_, T> {
            match self.references.get() {
                Unshared => self.references.set( Exclusive ),
                Shared(_) => panic!("cant borrow as mut as borrowed as immutable"),
                _ => panic!("cant borrow as mut as already borrowed as mutable"),
            }
            RefMut { refcell: self }
            // unsafe {
            //     &mut *self.value.get()
            // }
        }
        pub fn try_borrow(&self) -> bool {
            // match self.references.get() 
            //     Exclusive => false,
            //     _ => true,
            // }
            // Same as above
            // matches!( self.references.get(), x if x != Exclusive ) // I had to use PartialEq for this
                                                                      // to be accepted
            // is better and more direct
            self.references.get() != Exclusive
        }
        pub fn try_borrow_mut(&self) -> bool {
            // match self.references.get() {
            //     Unshared => true,
            //     _ => false,
            // }
            // // Same as above
            // matches!( self.references.get(), Unshared)
            // cleaner
            self.references.get() == Unshared
        }
    }
    // NOTE: we have to implement a way to drop values and then 
    // make our value decrement for shared references.
    #[derive(Debug)]
    pub struct Ref<'refc, T> {
        refcell: &'refc MyRefCell<T>,
    } 
    // by implementing the Drop and Deref, DerefMut traits on a type,
    // we are able to adjuct the count of the references for the origial MyRefCell. This
    // way, we know or lets say have an idea how this can be implemented
    // at runtime. The reason this is done is by looking at the borrow
    // and borrow_mut calls during the program.
    impl<'refc, T> Drop for Ref<'refc, T> {
        // calling drop on Ref affects the fields of the MyRefCell through interior mutability
        fn drop(&mut self) {
            match self.refcell.references.get() {
                // should not be possilbe as just creating a borrow will change it to shared value
                // If we call drop again on this type, it will panic through unreachable
                Unshared | Exclusive => unreachable!(),
                Shared(1) => self.refcell.references.set( Unshared ),
                Shared(x) => self.refcell.references.set( Shared(x-1) ), 
            }
        }
    }

    impl<'refc, T> Deref for Ref<'refc, T> {
        type Target = T; // not a trait here
        // Self as Deref looks as the Trait fields for this type
        // We have an associated type that tells us that the
        // Target has value T
        // The return type just tells that we get reference of that value type
        fn deref(&self) -> &<Self as Deref>::Target {
            // get() here gives us *mut T, so change it to &T
            unsafe { &*self.refcell.value.get() }
        }
    }

    // This does the same as Ref, except, that we get the mut references
    // for the value field of the MyRefCell. I does this as in intermediary type
    // so we can implement drop, deref and derefmut.
    pub struct RefMut<'refc, T> {
        refcell: &'refc MyRefCell<T>,
    } 

    // this drop will have to handle states for just borrow_mut as borrow_mut
    // is only the one capable to produce RefMut. So when dropping a borrow_mut reference 
    // We have to handle just the Exclusive types as the others are not possible
    impl<'refc, T> Drop for RefMut<'refc, T> {
        fn drop(&mut self) {
            match self.refcell.references.get() {
                // these states are not possible
                // if we manuall drop the again, they they will panic through unreachable
                Unshared | Shared(_) => unreachable!(),
                Exclusive => self.refcell.references.set( Unshared ), 
            }
        }
    }

    // both deref and derefmut are possible for RefMut
    impl<'refc, T> Deref for RefMut<'refc, T> {
        type Target = T; // not a trait here
        // Calling deref mut allows us to use the Target for out type
        // As Self is of Trait Deref, we seems we can call directly Self::Target
        fn deref(&self) -> &Self::Target {
            unsafe { &*self.refcell.value.get() }
        }
    }

    // this if just for derefmut that gets the target from
    // implementing deref. We have to implement defer or else we will
    // not be able to implement defermut for our type
    impl<'refc, T> DerefMut for RefMut<'refc, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            // get() give us *mut T, so we convert it to &mut T
            unsafe { &mut *self.refcell.value.get() }
        }
    }
}

// simple implemention of Cell type. Meant for copy types.
pub mod modcell {
    use super::*;
    // Note: UnsafeCell implements !sync inheritantly
    // Hence this doesnt allow us to use this over threads parallely
    #[derive(Debug)]
    pub struct MyCell<T> {
        value: UnsafeCell<T>,
    }
    
    impl<T> MyCell<T> {
        pub fn new(val: T) -> MyCell<T> {
            Self { value: UnsafeCell::new(val) }
        }
        // Since we dont impl sync, this wont allow us to 
        // implement threads. And that will prevent us using references to
        // the value as we are not giving it out for multi threading.
        pub fn set(&self, val: T) {
            unsafe { *self.value.get() = val; } // get give us a mut pointer to address
        }
        // We get only the copy of the value, as we validate this
        // by using the copy trait. This way we dont get the
        // reference of the value.
        pub fn get(&self) -> T 
        where T: Copy {
            unsafe { *self.value.get() }
        }
        // This will give us a mut reference for a value
        // This doesnt server us for Interior mutability as mut it required for self
        pub fn get_mut(&mut self) -> &mut T {
            self.value.get_mut()
        }

        pub fn replace(&self, val: T) -> T 
        where T: Copy {
            let x = self.get();
            self.set(val);
            x
        }
    }
}
