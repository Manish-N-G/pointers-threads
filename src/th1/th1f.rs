// Cow => Clone on write
// use std::any::type_name;
// use std::any::TypeId;

pub fn thread1f_cow() {
    let c1 = std::borrow::Cow::Borrowed("Hello there");  // works
    println!( "make owned str {:?}", make_owned(c1) );

    let x = [1,2,3,4,5];
    let c1 = std::borrow::Cow::Borrowed(&x[..]).into_owned();  // this works
    println!( "into owned directly {:?}", c1 );

    let x = [1,2,3,4,5];
    let c1 = std::borrow::Cow::Borrowed(&x[..]);  // this needs to be hanled for make_owned
    let y:Vec<u8> = make_owned(c1); // rust makes this to u8 by setting x as u8
    println!( "make owned for u8{:?}", y );

    let x = 5;
    let c1 = std::borrow::Cow::Borrowed(&x);  // this needs to be hanled for make_owned
    println!( "make owned direct{:?}", make_owned(c1) );

    println!("---------------Create Cow ----------------");

    let c1 = std::borrow::Cow::Borrowed("Hello there");  // works
    let z = create_cow_owned(c1);
    println!( "create cow {} str {:?} for type {}", cow_type(&z), z, get_type(&z) );

    let x = [1,2,3,4,5];
    let c1 = std::borrow::Cow::Borrowed(&x[..]);  // this needs to be hanled for create_cow 0wned
    let z = create_cow_owned(c1);
    println!( "create cow {} for u8{:?} for type {}", cow_type(&z), z, get_type(&z) );

    let x = 5;
    let c1 = std::borrow::Cow::Borrowed(&x);  // this needs to be hanled for create_cow 0wned
    let z = create_cow_owned(c1); 
    println!( "create cow {} direct{:?} for type {}", cow_type(&z), z, get_type(&z) );


    let x = [1,2,3,4,5];
    let mut c1 = std::borrow::Cow::Borrowed(&x[..]);  // this needs to be hanled for create_cow 0wned
    println!( "cow borrowed before update u8 {:?} for type {} is type {}", c1, get_type(&c1), cow_type(&c1) );
    // [6,7,8].iter().for_each(|&val| {
    //     c1.to_mut().push(val)
    // }); does the same
    for x in [6,7,8] {
        c1.to_mut().push(x) } println!( "cow owned after update u8 {:?} for type {} is type {}", c1, get_type(&c1), cow_type(&c1) );
    if let std::borrow::Cow::Owned(x) = c1 {
        println!("c1 is {:?}", x);
    }

    println!("---------------Make Cow ----------------");

    let x = 8;
    let y = make_cow_for_u8(x);
    println!("cow make u8 for {} is {:?} and type {}, os is owned - {}", x, y, get_type(&y), cow_is_owned(x) );
    let x = 3;
    let y = make_cow_for_u8(x);
    println!("cow make u8 for {} is {:?} and type {}, os is owned - {}", x, y, get_type(&y), cow_is_owned(x) );


    println!("---------------Cow replace cow type ----------------");
    let x = "hello world";
    let y = "hello <world>";
    for val in [x,y] {
        println!("val is now {} and type {}", sanitize(val), cow_type(&sanitize(val)));
    }
}

fn sanitize(val: &'static str) -> std::borrow::Cow<'static, str> {
    if val.contains('<') || val.contains('>') {
        let mut owned = val.to_owned();
        owned = owned.replace('<', "<<<" );
        owned = owned.replace('>', ">>>" );
        std::borrow::Cow::Owned(owned)
    } else {
        std::borrow::Cow::Borrowed(val)
    }
}

#[allow(clippy::ptr_arg)]
fn cow_type<'a, T: ToOwned + ?Sized + 'a>(val: &std::borrow::Cow<'a, T>) -> &'static str {
    match val {
        std::borrow::Cow::Owned(_) => "Owned",
        std::borrow::Cow::Borrowed(_) => "Borrowed",
    }
}

fn make_owned<T:ToOwned + ?Sized>(val: std::borrow::Cow<'_, T>) -> <T as ToOwned>::Owned {
    val.into_owned()
}

fn create_cow_owned<T: ToOwned + ?Sized>(val: std:: borrow::Cow<'_,T>) -> std::borrow::Cow<'_,T> {
    match val {
        std::borrow::Cow::Borrowed(b) => {
            std::borrow::Cow::Owned(b.to_owned())
        },
        std::borrow::Cow::Owned(_) => {
            // val.clone()
            val
        },
    }
}

fn get_type<T>(_: &T) -> String{
    format!("{:?}", std::any::type_name::<T>() )
}

fn make_cow_for_u8<'a>(val: u8) -> std::borrow::Cow<'a, str> {
    match val%3 {
        0 => "Convert to borrowed Cow".into(),
        _ => String::from("Convert to Owned Cow").into()
    }
}

fn cow_is_owned(val: u8) -> bool {
    match make_cow_for_u8(val) {
        std::borrow::Cow::Owned(_) => true,
        std::borrow::Cow::Borrowed(_) => false,
    }
}
