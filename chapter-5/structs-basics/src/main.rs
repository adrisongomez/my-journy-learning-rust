struct Foo {
    bar: u32,
    foobar: String,
}

struct Bar(i32,i32,String);

fn create_foo(bar: u32, foobar: String) -> Foo {
    Foo {
        bar,
        foobar,
    }
}

struct Unit;

fn main() {
    let foo = create_foo(35, String::from("Hello world"));

    println!("{} {}", foo.bar, foo.foobar);
    let foo1 = Foo {
        bar: 60,
        ..foo
    };
    println!("{} {}", foo1.bar, foo1.foobar);

    let bar = Bar(1,1, String::from("0"));
    println!("{} {} {}", bar.0, bar.1, bar.2);

    let _unit_struck = Unit{};

}
