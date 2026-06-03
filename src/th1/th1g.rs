// When using State as a trait object, the trait doesn’t know what the concrete self will be
// exactly, so the return type isn’t known at compile time. (This is one of the dyn
// compatibility rules)
mod state1_oop_box_dyn {
    // This trait is meant to use OOP concepts to represnt how we can add functionality to the
    // differnt types as structs in Rust. There as many ways genreally for rust to go about a task
    // but this on focuses on OOP and could has been implemented in a different way that the one 
    // that is used here.
    trait State {
        // Here, we try to use the State trait to control how this is implemented
        // This triat modifies the states for our type ( in this case the Post struct)
        // and removed that implementation from that type. This trait will allow us to
        // process it this way, as we are using Box dyn of that state. As We cant directly
        // get the type of the dyn state without using other libraries, we implement it
        // this way ,and we stay true to using the OOP style operation.
        // NOTE: Remember, self cant be a return type
        fn review_state(self: Box<Self>) -> Box<dyn State>;
        fn approve_state(self: Box<Self>) -> Box<dyn State>;
        fn add_state(self: Box<Self>) -> Box<dyn State>;
        // this content could have been &post rather than &str
        // I through it was best to do it this way when possible
        fn content<'a>(&'a self, _content: &'a str) -> &'a str {
            ""
        }
    }

    #[derive(Debug)]
    struct Created;
    impl State for Created {
        fn review_state(self: Box<Self>) -> Box<dyn State>{
            self // state does not change
        }
        fn approve_state(self: Box<Self>) -> Box<dyn State>{
            self // state cant be apporved before its reviewed
        }
        fn add_state(self: Box<Self>) -> Box<dyn State>{
            Box::new(Added) 
        }
    }

    #[derive(Debug)]
    struct Added;
    impl State for Added {
        // this function takes ownership of the Box holding the type that implemements the trait
        // and returns a Box that we can create.
        fn review_state(self: Box<Self>) -> Box<dyn State>{
            Box::new(Reviewed)
        }
        fn approve_state(self: Box<Self>) -> Box<dyn State>{
            self
        }
        fn add_state(self: Box<Self>) -> Box<dyn State>{
            self
        }
    }

    #[derive(Debug)]
    struct Reviewed;
    impl State for Reviewed {
        fn review_state(self: Box<Self>) -> Box<dyn State>{
            self
        }
        fn approve_state(self: Box<Self>) -> Box<dyn State>{
            Box::new(Approved)
        }
        fn add_state(self: Box<Self>) -> Box<dyn State>{
            Box::new(Added)
        }
        fn content(&self, _content: & str) -> &str{
            ""
        }
    }

    #[derive(Debug)]
    struct Approved;
    impl State for Approved {
        fn review_state(self: Box<Self>) -> Box<dyn State>{
            Box::new(Reviewed)
        }
        fn approve_state(self: Box<Self>) -> Box<dyn State>{
            self
        }
        fn add_state(self: Box<Self>) -> Box<dyn State>{
            Box::new(Added)
        }
        fn content<'a>(&self, content: &'a str) -> &'a str{
            content
        }
    }

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Self {
            Self { state: Some(Box::new(Created)), content: "".into() }
        }

        pub fn insert(&mut self, content: impl Into<String>){
            self.content = content.into();
            self.state = Some(self.state.take().unwrap().add_state());
        }

        pub fn append(&mut self, content: &str){
            self.content.push_str(content);
            self.state = Some(self.state.take().unwrap().add_state());
        }

        pub fn review(&mut self){
            if let Some(s) = self.state.take() {
                self.state = Some(s.review_state())
            }
        }

        pub fn approve(&mut self){
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve_state())
            }
        }

        pub fn get(&self) -> &str {
            // The lifetime will become an issue if we are not careful here.
            // we convert &Option<Box<State>> to Option<&Box<State>> with as_ref()
            // This is quite important as unwrap takes self gives problems for &Option types
            // and we Option in order to get the &T value inside Option. Otherwise,
            // if this important step is not considered, we will will have a block at
            // the unwrap step and could face issues in the &self.content step down the line.
            self.state.as_ref().unwrap().content(&self.content)
        }
    }

}


mod state1_oop_dyn_any {
    use std::any::Any;
    // we can use the any type that allows us the get the concrete type in order to compare
    // when we dont get the value via just dyn traits. This is important to to how to
    // implement, cause we can use more that one way to get the solution when we want
    // to compare value for dyn trait objects and use them in like these semi oop principles
    // Here Any is used to enulate dynamic typing and this hopefully should be enougth
    // This works with Downcasting which we will use to get the value we need for the comparison
    trait State: Any {
        fn review_state(self: Box<Self>) -> Box<dyn Any>;
        fn approve_state(self: Box<Self>) -> Box<dyn Any>;
        fn add_state(self: Box<Self>) -> Box<dyn Any>;
    }

    // can have a helper function here that I can use to get the value forkkjj a type
    // that can get the trait
    // impl dyn State {
    //     fn get_struct(&self) -> Option<&str> {
    //
    //     }
    // }

    #[derive(Debug)]
    struct Created;
    impl State for Created {
        fn review_state(self: Box<Self>) -> Box<dyn Any>{
            self // state does not change
        }
        fn approve_state(self: Box<Self>) -> Box<dyn Any>{
            self // state cant be apporved before its reviewed
        }
        fn add_state(self: Box<Self>) -> Box<dyn Any>{
            Box::new(Added) 
        }
    }

    #[derive(Debug)]
    struct Added;
    impl State for Added {
        // this function takes ownership of the Box holding the type that implemements the trait
        // and returns a Box that we can create.
        fn review_state(self: Box<Self>) -> Box<dyn Any>{
            Box::new(Reviewed)
        }
        fn approve_state(self: Box<Self>) -> Box<dyn Any>{
            self
        }
        fn add_state(self: Box<Self>) -> Box<dyn Any>{
            self
        }
    }

    #[derive(Debug)]
    struct Reviewed;
    impl State for Reviewed {
        fn review_state(self: Box<Self>) -> Box<dyn Any>{
            self
        }
        fn approve_state(self: Box<Self>) -> Box<dyn Any>{
            Box::new(Approved)
        }
        fn add_state(self: Box<Self>) -> Box<dyn Any>{
            Box::new(Added)
        }
    }

    #[derive(Debug)]
    struct Approved;
    impl State for Approved {
        fn review_state(self: Box<Self>) -> Box<dyn Any>{
            Box::new(Reviewed)
        }
        fn approve_state(self: Box<Self>) -> Box<dyn Any>{
            self
        }
        fn add_state(self: Box<Self>) -> Box<dyn Any>{
            Box::new(Added)
        }
    }

    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Self {
            Self { state: Some(Box::new(Created)), content: "".into() }
        }

        pub fn insert(&mut self, content: impl Into<String>){
            self.content = content.into();
            self.state = Some(self.state.take().unwrap().add_state().downcast::<Added>().unwrap());
        }

        pub fn append(&mut self, content: &str){
            self.content.push_str(content);
            self.state = Some(self.state.take().unwrap().add_state().downcast::<Added>().unwrap());
        }

        pub fn review(&mut self){
            // approach one
            let concrete = (**self.state.as_ref().unwrap()).type_id();
            if concrete == std::any::TypeId::of::<Added>() {
                self.state = Some(self.state.take().unwrap().review_state().downcast::<Reviewed>().unwrap());
            }
        }

        pub fn approve(&mut self){
            // approach two
            if let Some(s) = self.state.take() {
                let concrete = (*s).type_id();
                if concrete == std::any::TypeId::of::<Reviewed>(){
                    self.state = Some(s.approve_state().downcast::<Approved>().unwrap())
                } else {
                    self.state = Some(s)
                }
            }
        }

        pub fn get(&self) -> &str {
            let concrete = (**self.state.as_ref().unwrap()).type_id();
            // this was quite painful to get. But I was able to get it only with (** ... of
            // &Box<dyn Trait>). cause direct * or &* or &** was causing me a lot of problems
            if concrete == std::any::TypeId::of::<Approved>() {
                return &self.content;
            }
            ""
        }
    }

}


// The transformations between the states for this new approach are no longer encapsulated entirely
// within the Post implementation. However, our gain is that invalid states are now impossible
// because of the type system and the type checking that happens at compile time
mod state1_non_oop {
    #[derive(Debug)]
    pub struct Post{
        content: String,
    }
    impl Post {
        #[allow(clippy::new_ret_no_self)]
        pub fn new() -> DraftPost{
            DraftPost
        }
        pub fn get(&self) -> &str {
            &self.content
        }
        pub fn add_content(mut self, val: impl Into<String>) -> AddedPost {
            self.content = val.into();
            AddedPost { content: self.content }
        }
        pub fn append_content(mut self, val: &str) -> AddedPost {
            self.content.push_str(val);
            AddedPost { content: self.content }
        }
    }

    #[derive(Debug)]
    pub struct DraftPost;
    impl DraftPost {
        pub fn add_content(self, val: impl Into<String>) -> AddedPost {
            AddedPost { content: val.into() }
        }
    }
    
    #[derive(Debug)]
    pub struct AddedPost {
        content: String,
    }
    impl AddedPost {
        pub fn review(self) -> ReviewedPost {
            ReviewedPost { content: self.content }
        }
        pub fn append_content(&mut self, val: &str) {
            self.content.push_str(val);
        }
        pub fn add_content(&mut self, val: impl Into<String>) {
            self.content = val.into();
        }
    }

    #[derive(Debug)]
    pub struct ReviewedPost {
        content: String,
    }
    impl ReviewedPost {
        pub fn approve(self) -> Post {
            Post { content: self.content }
        }
        pub fn add_content(mut self, val: impl Into<String>) -> AddedPost {
            self.content = val.into();
            AddedPost { content: self.content }
        }
        pub fn append_content(mut self, val: &str) -> AddedPost {
            self.content.push_str(val);
            AddedPost { content: self.content }
        }
    }
}

pub mod state1_oop_non_oop {
    pub use super::state1_oop_box_dyn::Post as Post_OOP;
    pub use super::state1_non_oop::Post as Post_Non;
    pub use super::state1_oop_dyn_any::Post as Post_Any;

    pub fn thread1g_state_pattern() {
        println!("\n------------------------ state pattern ------------------------------");
        let mut p = Post_OOP::new();
        p.review();
        p.approve();
        println!("post_oop state created {:?}", p.get());
        p.insert("testing what we got");
        p.append(". I think we got it");
        println!("post_oop state added {:?}", p.get());
        p.review();
        println!("post_oop state reviewed {:?}", p.get());
        p.approve();
        println!("post_oop state approved {:?}", p.get());
        p.append(". Success!!! ");
        println!("post_oop approved to append {:?}", p.get());
        p.approve();
        println!("post_oop approved to approve before review {:?}", p.get());
        p.review();
        p.approve();
        println!("post_oop approved to approve after review.. {:?}", p.get());
    }

    pub fn thread1g_type_pattern() {
        println!("\n------------------------ type pattern ------------------------------");
        let p = Post_Non::new();
        println!("post_non as Draft {:?}", p);
        let mut p = p.add_content("Now type pattern testing to see what we got");
        println!("post_non Added {:?}", p);
        println!("post_non Added updated {:?}", p.append_content(". This is appended"));
        let p = p.review();
        let p = p.append_content(". This is again added after review");
        println!("post_non back to added {:?}", p );
        let p = p.review();
        println!("post_non Review {:?}", p );
        let p = p.approve();
        println!("post_non approved {:?}", p );
        println!("post_non approved {:?}", p.get() );
        let p = p.append_content(". After approved");
        let p = p.review();
        let p = p.approve();
        println!("post_non Finised {:?}", p.get() );
        println!("chain all post_non - {}", Post_Non::new()
            .add_content("Chain Val")
            .review()
            .append_content(". appended")
            .review()
            .approve().get());
    }

    pub fn thread1g_any_pattern() {
        println!("\n------------------------ any pattern -------------------------------");
        let mut p = Post_Any::new();
        p.review();
        p.approve();
        println!("post_any state created {:?}", p.get());
        p.insert("testing what we got");
        p.append(". I think we got it");
        println!("post_any state added {:?}", p.get());
        p.review();
        println!("post_any state reviewed {:?}", p.get());
        p.approve();
        println!("post_any state approved {:?}", p.get());
        p.append(". Success!!! ");
        println!("post_any approved to append {:?}", p.get());
        p.approve();
        println!("post_any approved to approve before review {:?}", p.get());
        p.review();
        p.approve();
        println!("post_any approved to approve after review.. {:?}", p.get());
    }
}


