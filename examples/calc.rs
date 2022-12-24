use enve::{
    estring::{self, Aggregatable, Aggregate},
    Product, SepVec, Sum,
};

type PlusVec<T> = SepVec<T, '+'>;
type MulVec<T> = SepVec<T, '*'>;

const HELP_MESSAGE: &str = "
USAGE:
E=10+10*2-4 cargo run --example calc --all-features
";

fn main() {
    let res: f32 = enve::get::<Sum<PlusVec<MinusVec<Product<MulVec<f32>>>>>>("E")
        .map_err(|err| {
            match err.reason() {
                enve::Reason::NotPresent => eprintln!("The expression was not found"),
                rest => eprintln!("ERROR: {}", rest),
            }

            eprintln!("{}", HELP_MESSAGE);
            std::process::exit(0);
        })
        .unwrap()
        .agg();

    println!("result: {}", res);
}

struct MinusVec<T>(Vec<T>);

impl<T> estring::ParseFragment for MinusVec<T>
where
    T: estring::ParseFragment,
{
    fn parse_frag(es: estring::EString) -> estring::Result<Self> {
        let mut prev: Option<&str> = None;
        es.split('-')
            .map(str::trim)
            .map(|val| match prev.replace(val) {
                None => String::from(val),
                Some(_) => {
                    let mut s = val.to_owned();
                    s.insert(0, '-');
                    s
                }
            })
            .filter(|val| !val.is_empty())
            .map(estring::EString::from)
            .map(T::parse_frag)
            .collect::<estring::Result<Vec<T>>>()
            .map(Self)
    }
}

impl<T> estring::Aggregatable for MinusVec<T>
where
    T: Aggregatable,
{
    type Item = T::Item;

    fn items(self) -> Vec<Self::Item> {
        self.0.into_iter().flat_map(T::items).collect()
    }
}
