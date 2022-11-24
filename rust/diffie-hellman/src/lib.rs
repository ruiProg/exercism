use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.modpow(a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.modpow(a, p)
}

trait ModuleOfPower {
    fn modpow(&self, exponent: Self, modulus: Self) -> Self;
}

impl ModuleOfPower for u64 {
    fn modpow(&self, exponent: u64, modulus: u64) -> u64 {
        use num_bigint::BigUint;

        let base = BigUint::from(*self);
        let exponent = BigUint::from(exponent);
        let modulus = BigUint::from(modulus);

        base.modpow(&exponent, &modulus)
            .iter_u64_digits()
            .next()
            .unwrap()
    }
}
