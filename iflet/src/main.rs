enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn iflet() {
    println!("---Start if let---");

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }
    println!("---End if let---");
}

fn whilelet() {
    println!("---Start while let---");

    let mut optional = Some(0);
    
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9 quit");
            optional = None;
        } else {
            println!("i is {:?}", i);
            optional = Some(i+1);
        }
    }

    println!("---End while let---");
}

fn main() {
    println!("---Start---");

    iflet();

    whilelet();
    
    println!("---End---");
}
