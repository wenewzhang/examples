
struct Foo;

impl Foo{
    fn print(&self){
        println!("print foo");
    }

    //为了方便就用了immutable borrow，否则main最后面调用f.drop()过不去，需要其他办法来绕
    fn drop(&self){
        println!("drop foo");
    }
}

thread_local!(static F:Foo = Foo);


fn main(){
    F.with(move|f:&Foo|{
        f.print();
    });

    F.with(|f|{
        f.drop();
    });

    println!("exit");
}
