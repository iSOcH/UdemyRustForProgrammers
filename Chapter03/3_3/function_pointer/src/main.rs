fn my_function(inp: i32) -> i32 {
    inp * 2
}

fn takes_some_function_as_input(f: fn(i32) -> i32) {
    f(2);
}

fn main() {
    takes_some_function_as_input(my_function);

    let closure = |inp: i32| -> i32 { inp * 2 };

    // this call becomes invalid as soon as the closure captures any context
    // compiler: "closures can only be coerced to `fn` types if they do not capture any variables"
    takes_some_function_as_input(closure);
}
