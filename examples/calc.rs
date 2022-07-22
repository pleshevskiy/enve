use enve::core::SepVec;

type MinusVec<T> = SepVec<T, '-'>;
type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

const HELP_MESSAGE: &str = "
USAGE:
E=10+10*2+4 cargo run --example calc --all-features
";

fn main() -> Result<(), enve::Error> {
    let res: f32 = enve::get::<PlusVec<MinusVec<MulVec<f32>>>>("E")
        .map_err(|err| {
            match err {
                enve::Error::NotPresent => eprintln!("The expression was not found"),
                rest => eprintln!("ERROR: {}", rest),
            }

            eprintln!("{}", HELP_MESSAGE);
            std::process::exit(0);
        })
        .unwrap()
        .iter()
        .map(|p| {
            p.iter()
                .map(|m| m.iter().product::<f32>())
                .reduce(|acc, v| acc - v)
                .unwrap_or_default()
        })
        .sum::<f32>();

    println!("result: {}", res);

    Ok(())
}
