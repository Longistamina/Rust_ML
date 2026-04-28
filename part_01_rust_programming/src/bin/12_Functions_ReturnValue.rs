fn func_simple() {
    println!("Hello Wolrd!!!");
}

fn func_single_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn func_multi_parameters(value: f64, unit_label: char) { // Must always specify the types!!!
    let value = value*2.0;
    println!("The result of value*2 is: {value}{unit_label}");
}

fn func_return_pow(base: f64, exponent: f64) -> f64 {
    let base = base as f64; // casting to f64, ensure the right type
    let exponent = exponent as f64;
    
    base.powf(exponent) // The returned value, as scalar
}

fn func_multi_return(base: f64, exp: f64) -> (f64, f64, bool) {
    let result = base.powf(exp);
    let is_positive = result > 0.0;

    // To return multiple values, just wrap the variables in parentheses
    (result, base, is_positive) 
}

fn get_coordinates() -> [f64; 2] {
    let x = 10.5;
    let y = 20.2;

    [x, y] // Return as an array
}

fn main() {
    func_simple();
    func_single_parameter(53);
    func_multi_parameters(3.8, 'N');

    let result_pow = func_return_pow(2.0, 3.2);
    println!("Function returning one value: {result_pow}"); // 2^3.2

    let (val, original_base, status) = func_multi_return(2.0, 3.0);
    println!("Function returning multiple values: Reulst <-> {val}, Base <-> {original_base}, Positive <-> {status}");

    let coordinates = get_coordinates();
    println!("Function returning one array: {coordinates:?}");
}