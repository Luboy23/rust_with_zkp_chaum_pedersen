use num_bigint::{BigUint, RandBigInt};
use rand;

pub struct ZKP {
    p: BigUint,
    q:BigUint,
    alpha: BigUint,
    beta: BigUint,
}

impl ZKP {

/// 计算 alpha^x mod p
/// 输出：output = n^exp mod p
/// 参数:
/// - `n`: 基数 (BigUint)
/// - `exponent`: 指数 (BigUint)
/// - `modulus`: 模数 (BigUint)
/// 
/// 返回:
/// - `BigUint`: 计算结果 n^exp mod p
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}

/// 计算公式：s = k - c * x mod q
/// 输出：s
/// 参数:
/// - `k`: 临时私钥 (BigUint)
/// - `c`: 哈希值或挑战值 (BigUint)
/// - `x`: 私钥 (BigUint)
/// - `q`: 模数 (通常为素数) (BigUint)
/// 
/// 返回:
/// - `BigUint`: 计算结果 s
pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
    if *k >= c * x {
       // 如果 k >= c * x，直接计算 (k - c * x) mod q
       return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
    } else {
       // 如果 k < c * x，则计算 q - (c * x - k) mod q
       return &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q);
    }
}

/// 验证两个条件:
/// 条件1: r1 = alpha^s * y1^c
/// 条件2: r2 = beta^s * y2^c
/// 
/// 参数:
/// - `r1`: 参数 r1 (BigUint)
/// - `r2`: 参数 r2 (BigUint)
/// - `y1`: 参数 y1 (BigUint)
/// - `y2`: 参数 y2 (BigUint)
/// - `alpha`: 基数 alpha (BigUint)
/// - `beta`: 基数 beta (BigUint)
/// - `c`: 哈希值或挑战值 (BigUint)
/// - `s`: 计算出的 s 值 (BigUint)
/// - `p`: 模数 p (通常为素数) (BigUint)
/// 
/// 返回:
/// - `bool`: 验证是否通过（即两个条件是否都成立）
pub fn verify(&self, r1: &BigUint, r2: &BigUint, y1: &BigUint, y2: &BigUint, c: &BigUint, s: &BigUint) -> bool {
    let cond1 = *r1 == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p)).modpow(&BigUint::from(1u32), &self.p);
    let cond2 = *r2 == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p)).modpow(&BigUint::from(1u32), &self.p);
    // 返回两个条件的与运算结果
    cond1 && cond2
}

pub fn generate_random_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint_below(bound)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_toy_example() {
        // 定义 alpha, beta, p, q
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);  // 模数
        let q = BigUint::from(11u32);   // 模数 q
        let zkp = ZKP {p:p.clone(), q:q.clone(), alpha: alpha.clone(), beta:beta.clone()};

        let x = BigUint::from(6u32);   // 私钥 x
        let k = BigUint::from(7u32);   // 临时私钥 k

        let c = BigUint::from(4u32);   // 挑战值 c

        // 计算 y1 和 y2
        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));  // 验证计算 y1 的结果
        assert_eq!(y2, BigUint::from(3u32));  // 验证计算 y2 的结果

        // 计算 r1 和 r2
        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);
        assert_eq!(r1, BigUint::from(8u32));  // 验证计算 r1 的结果
        assert_eq!(r2, BigUint::from(4u32));  // 验证计算 r2 的结果

        // 使用假设的私钥计算 s_fake
        let x_fake = BigUint::from(7u32);     // 假的私钥
        let s_fake = zkp.solve(&k, &c, &x_fake);  // 计算 s_fake
        // assert_eq!(s, BigUint::from(5u32));

        // 验证 s_fake 是否满足验证条件
        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result);  // 应该返回 false，因为 s_fake 使用了错误的私钥
    }
    
    #[test]
    fn test_example_with_random_numbers() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);  // 模数
        let q = BigUint::from(11u32);   // 模数 q
        let zkp = ZKP {p:p.clone(), q:q.clone(), alpha: alpha.clone(), beta:beta.clone()};

        let x = BigUint::from(6u32);   // 私钥 x
        let k = ZKP::generate_random_below(&q);

        let c = ZKP::generate_random_below(&q);
 
        // 计算 y1 和 y2
        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);
        assert_eq!(y1, BigUint::from(2u32));  // 验证计算 y1 的结果
        assert_eq!(y2, BigUint::from(3u32));  // 验证计算 y2 的结果

        // 计算 r1 和 r2
        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2,  &c, &s);
        assert!(result);
    }
}