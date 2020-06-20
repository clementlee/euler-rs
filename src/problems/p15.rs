use crate::lib::FactorizedNum;

const VAL: u32 = 20;
pub fn solution() -> u64 {
    let mut base = FactorizedNum::create(1);

    for i in 2..=VAL {
        base = base * &FactorizedNum::create(i);
    }

    let mut next = base.clone();

    for i in (VAL + 1)..=(2 * VAL) {
        next = next * &FactorizedNum::create(i)
    }

    next = ((next / &base).unwrap() / &base).unwrap();

    next.materialize()
}
