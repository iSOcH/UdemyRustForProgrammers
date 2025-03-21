fn my_function(inp: i32) -> i32 {
    inp * 2
}

fn my_function2() {
    let num = 2;

    // this is impl Fn(i32) -> i32, but we cannot add the type annotation
    let closure = |inp: i32| -> i32 { inp * 2 };

    let mut data_for_mut = 5;

    // unexpected: this closure (of type impl FnMut(i32)) cannot be called unless it is mut
    // compiler: "calling `closure_mut` requires mutable binding due to mutable borrow of `data_for_mut`"
    let mut closure_mut = |inp: i32| {
        data_for_mut += inp;
    };

    let ret = closure(num);
    closure_mut(ret);

    println!("{}", data_for_mut);
}

fn main() {
    let num = 2;
    let ret = my_function(num);
    println!("{}", ret);

    my_function2();
}
