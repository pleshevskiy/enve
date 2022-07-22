use enve::core::SepVec;

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

fn main() -> Result<(), enve::Error> {
    let res: f32 = enve::get::<PlusVec<MulVec<f32>>>("E")?
        .iter()
        .map(|m| m.iter().product::<f32>())
        .sum();

    println!("result: {}", res);

    Ok(())
}
