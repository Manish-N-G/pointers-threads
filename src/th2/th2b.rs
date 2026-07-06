#[allow(non_snake_case)]
pub fn thread2b_RcWeak() {
    enum RcWeak {
        NRc(std::rc::Rc<Node>),
        NWeak(std::rc::Weak<Node>),
    }
    struct Node {
        a: u8,
        b: std::cell::RefCell<Vec<RcWeak>>,
    }

    // let x = std::rc::Rc::new(Node{a:3, b: std::cell::RefCell::new(vec![])});
    // let y = std::rc::Rc::new(Node{a:4, b: std::cell::RefCell::new(vec![RcWeak::NWeak(std::rc::Rc::downgrade(&x))])});
    // x.b.borrow_mut().push(RcWeak::NWeak(std::rc::Rc::downgrade(&y)));
    //
    // println!("strong x is {:?} and strong y is {}", std::rc::Rc::strong_count(&x), std::rc::Rc::strong_count(&y));
    // println!("weak x is {:?} and weak y is {}", std::rc::Rc::weak_count(&x), std::rc::Rc::weak_count(&y));
    //
    // drop(x);
    // println!("new strong y is {:?} and weak y is {}", std::rc::Rc::strong_count(&y), std::rc::Rc::strong_count(&y));
    // drop(y); // this should clear all the references

    // this doesnt work
    let x = std::rc::Rc::new(Node {
        a: 3,
        b: std::cell::RefCell::new(vec![]),
    });
    let y = std::rc::Rc::new(Node {
        a: 4,
        b: std::cell::RefCell::new(vec![RcWeak::NRc(std::rc::Rc::clone(&x))]),
    });
    x.b.borrow_mut().push(RcWeak::NRc(std::rc::Rc::clone(&y)));

    // let yb = x.b.borrow();
    // let RcWeak::NRc(yweak) = &yb[0] else {
    //     unreachable!("panic");
    // };
    // let y2 = std::rc::Rc::downgrade(yweak);
    // drop(yb);
    //
    // let mut yrc = x.b.borrow_mut();
    // let yy = yrc.get_mut(0).unwrap();
    // *yy = RcWeak::NWeak(y2);
    // drop(yrc);

    let mut yb = x.b.borrow_mut();
    if let RcWeak::NRc(yweak) = &yb[0] {
        let y2 = std::rc::Rc::downgrade(yweak);
        yb[0] = RcWeak::NWeak(y2);
    };
    drop(yb);

    let mut xb = y.b.borrow_mut();
    if let RcWeak::NRc(xweak) = &xb[0] {
        let x2 = std::rc::Rc::downgrade(xweak);
        xb[0] = RcWeak::NWeak(x2);
    };
    drop(xb);

    println!(
        "new strong x is {:?} and new strong y is {}",
        std::rc::Rc::strong_count(&x),
        std::rc::Rc::strong_count(&y)
    );
    println!(
        "weak x is {:?} and weak y is {}",
        std::rc::Rc::weak_count(&x),
        std::rc::Rc::weak_count(&y)
    );

    drop(x);
    drop(y);
}
