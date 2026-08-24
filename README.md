# This Rust library exposes mock functions to understand Pointers, threads and Async Operations

The purpose of this library is meant to expose you to how the different types in rust work with regards to Pointers, threads and Async

## For Pointers:

We start from simple implementation of how simple pointers work to actually creating our own pointers and getting into some deep topics in order to understand how they work under the hood. In this part, we cover
- `Rc`
- RefCell
- Arc
- `Mutex`
- Cow
- `RWLock`
- Locks

Even through the symbols exposed in this library can range from basic operations to fuzzy logical outcomes, we try to understand by looking the the source code, how they operate. Some of the move complicated topics we will cover here are

- Parking 
- `Unparking`
- PhantomData
- PhantomPinned
- Pinning
- Condvars
- Yielding

There are more to come in the future of this library.

## For Threads

For Threads, we cant truly isolate it by itself, without knowing how some pointers work. 

