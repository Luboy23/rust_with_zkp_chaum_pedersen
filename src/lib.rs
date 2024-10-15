use num_bigint::{BigUint, RandBigInt};
use rand::{self, Rng};



pub struct ZKP {
    pub p: BigUint,
    pub q:BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
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

pub fn generate_random_number_below(bound: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint_below(bound)
    }

pub fn generate_random_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be( &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(), );
        let alpha = BigUint::from_bytes_be( &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(), );


        let exp = BigUint::from_bytes_be( &hex::decode("5C3FD564B7747F9E2742A4").unwrap(), );
        // beta = alpha^x is also a generator
        let beta = alpha.modpow(&exp, &p);

        (alpha, beta, p ,q)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hex;


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
        let k = ZKP::generate_random_number_below(&zkp.q);

        let c = ZKP::generate_random_number_below(&zkp.q);

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

    #[test]
    fn test_1024_bits_constants() {
               //
        //    Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        //    The hexadecimal value of the prime is:
        //
        //    p = B10B8F96 A080E01D DE92DE5E AE5D54EC 52C99FBC FB06A3C6
        //        9A6A9DCA 52D23B61 6073E286 75A23D18 9838EF1E 2EE652C0
        //        13ECB4AE A9061123 24975C3C D49B83BF ACCBDD7D 90C4BD70
        //        98488E9C 219A7372 4EFFD6FA E5644738 FAA31A4F F55BCCC0
        //        A151AF5F 0DC8B4BD 45BF37DF 365C1A65 E68CFDA7 6D4DA708
        //        DF1FB2BC 2E4A4371
        //
        //    The hexadecimal value of the generator is:
        //
        //    g = A4D1CBD5 C3FD3412 6765A442 EFB99905 F8104DD2 58AC507F
        //        D6406CFF 14266D31 266FEA1E 5C41564B 777E690F 5504F213
        //        160217B4 B01B886A 5E91547F 9E2749F4 D7FBD7D3 B9A92EE1
        //        909D0D22 63F80A76 A6A24C08 7A091F53 1DBF0A01 69B6A28A
        //        D662A4D1 8E73AFA3 2D779D59 18D08BC8 858F4DCE F97C2A24
        //        855E6EEB 22B3B2E5
        //    q = F518AA87 81A8DF27 8ABA4E7D 64B7CB9D 49462353
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be( &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(), );
        let alpha = BigUint::from_bytes_be( &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(), );

        // beta = alpha^x is also a generator
        let beta = alpha.modpow(&ZKP::generate_random_number_below(&q), &p);

        let zkp = ZKP {p:p.clone(), q:q.clone(), alpha: alpha.clone(), beta:beta.clone()};

        let x = ZKP::generate_random_number_below(&zkp.q);
        let k = ZKP::generate_random_number_below(&zkp.q);        let c = ZKP::generate_random_number_below(&zkp.q);

        // 计算 y1 和 y2
        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        // 计算 r1 和 r2
        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result = zkp.verify(&r1, &r2, &y1, &y2,  &c, &s);
        assert!(result);
    }   
}