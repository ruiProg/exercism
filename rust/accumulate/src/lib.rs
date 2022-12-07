pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut output = Vec::with_capacity(input.len());

    for val in input {
        output.push(function(val));
    }

    output
}
