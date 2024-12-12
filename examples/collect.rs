use symbolica::{atom::Atom, fun, state::State};

fn main() {
    let input = Atom::parse("x*(1+a)+x*5*y+f(5,x)+2+y^2+x^2 + x^3").unwrap();
    let x = Atom::new_var(State::get_symbol("x"));
    let key = State::get_symbol("key");
    let coeff = State::get_symbol("val");

    let r = input.coefficient_list::<i8, _>(std::slice::from_ref(&x));

    println!("> Coefficient list:");
    for (key, val) in r {
        println!("\t{} {}", key, val);
    }

    println!("> Collect in x:");
    let out = input.collect::<i8, _>(
        &x,
        Some(Box::new(|x, out| {
            out.set_from_view(&x);
        })),
        None,
    );
    println!("\t{}", out);

    println!("> Collect in x with wrapping:");
    let out = input.collect::<i8, _>(
        &x,
        Some(Box::new(move |a, out| {
            out.set_from_view(&a);
            *out = fun!(key, out);
        })),
        Some(Box::new(move |a, out| {
            out.set_from_view(&a);
            *out = fun!(coeff, out);
        })),
    );
    println!("\t{}", out);
}
