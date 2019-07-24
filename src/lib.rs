mod utils;

use wasm_bindgen::prelude::*;

extern crate rand;

use sha2::Sha256;
use sha2::Digest;
use sha2::digest::generic_array::typenum::U32;
use sha2::digest::generic_array::GenericArray;

//use std::io::Bytes;
//use digest::{Reset, Input};
use rand::{thread_rng, Rng, random};
use rand::distributions::Alphanumeric;
use std::str;
use std::marker::Copy;
use num_bigint::BigInt;
use num_traits::Num;
//use std::str::FromStr;
//use wasm_bindgen::__rt::core::fmt::rt::v1::Count::Param;
//use num_integer::Integer;

//use num_bigint::ParseBigIntError;
extern crate num_bigint;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

//#[wasm_bindgen]
fn _gen() -> GenericArray<u8, U32> {
    Sha256::digest(&thread_rng().sample_iter(&Alphanumeric).take(10).collect::<String>().into_bytes()[..])
}

//#[wasm_bindgen]
fn gen() -> String {
    format!("{:x}{:x}{:x}", _gen(), _gen(), _gen())
//    "sfa".to_string()
}

fn gen_big_int() -> BigInt {
    BigInt::from_str_radix(gen().as_str(), 16).unwrap()
}

fn hashing_BigInt(message: BigInt) -> BigInt {
    hashing_str(message.to_string().as_str())
//    let input = message.to_str_radix(10).as_bytes();
//    let hash_str = format!("{:x}", Sha256::digest(input));
//    let hash_str = BigInt::from_str_radix(hash_str.as_str(), 16).unwrap();
//    hash_str % BigInt::from(731499577)
}

fn hashing_str(message: &str) -> BigInt {
    let input = message.clone().as_bytes();
    let hash_str = format!("{:x}", Sha256::digest(input));
    let hash_str = BigInt::from_str_radix(hash_str.as_str(), 16).unwrap();
    hash_str % BigInt::from(731499577)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hashing(BigInt::from(1)), BigInt::from(149138774));
        assert_eq!(hashing(BigInt::from(2)), BigInt::from(190256123));
        assert_eq!(hashing(BigInt::from(3344477)), BigInt::from(125782615));
        assert_eq!(hashing(BigInt::from_str("3344477").unwrap()), BigInt::from(125782615));
    }
}

#[wasm_bindgen]
pub struct Test {
    a: u64,
    b: u64,
}

#[derive(Default)]
struct Priv {
    rs: BigInt,
    re: BigInt,
    rw: BigInt,
    rz: BigInt,
    rx: BigInt,
    r: BigInt,
    w: BigInt,
    r_: BigInt,
    rn: [BigInt; 20],
    z: BigInt,

    x: BigInt,
}

fn String2BigInt(s: &String) -> BigInt {
    str2BigInt(s.as_str())
}

fn str2BigInt(s: &str) -> BigInt {
    BigInt::from_str_radix(s, 16).unwrap()
}

impl Priv {
    fn new(e: &String) -> Priv {
        let mut p = Priv {
            rs: String2BigInt(&gen_random_32()),
            re: String2BigInt(&gen_random_32()),
            rw: String2BigInt(&gen_random_32()),
            rz: String2BigInt(&gen_random_32()),
            rx: String2BigInt(&gen_random_32()),
            w: String2BigInt(&gen_random_32()),
            r: String2BigInt(&gen_random_32()),
            rn: [Default::default(),Default::default(),Default::default(),Default::default(),Default::default(),
                Default::default(),Default::default(),Default::default(),Default::default(),Default::default(),
                Default::default(),Default::default(),Default::default(),Default::default(),Default::default(),
                Default::default(),Default::default(),Default::default(),Default::default(),Default::default(),],
            r_: Default::default(),
            z: Default::default(),

            x: Default::default(),
        };
        p.r_ = &p.rz - (String2BigInt(e) * &p.rw);
        p
    }

    fn fuzz(&self, secret: usize, rand_num: &BigInt) -> BigInt {
        &self.rn[secret] + rand_num * &self.x
    }
}

struct Rnym {
    exp: BigInt,
    //已经算好的
    gamma: BigInt,
    grnym: BigInt,
}

struct Base {
    g: BigInt,
    h: BigInt,
    m: BigInt,
}

impl Base {
    fn new(g: &str, h: &str, m: BigInt) -> Base {
        Base {
            g: str2BigInt(g),
            h: str2BigInt(h),
            m: m,
        }
    }
    fn exp(&self, g_exp: &BigInt, h_exp: &BigInt) -> BigInt {
        _exp(&self.g, g_exp, &self.h, h_exp, &self.m)
//        (self.g.modpow(g_exp, m) * self.h.modpow(h_exp, m)) % m
    }
}

fn _exp(b1: &BigInt, e1: &BigInt, b2: &BigInt, e2: &BigInt, m: &BigInt) -> BigInt {
    (b1.modpow(e1, m) * b2.modpow(e2, m)) % m
}

impl Rnym {
    fn new(exp: &str, gamma: &str, id: &str) -> Rnym {
        let mut rnym = Rnym {
            exp: str2BigInt(exp),
            gamma: str2BigInt(gamma),
            grnym: Default::default(),
        };
        rnym.grnym = hashing_str(id).modpow(&rnym.exp, &rnym.gamma);
        rnym
    }
}

#[derive(Default)]
struct Params {
    Cs: BigInt,
    Ce: BigInt,
    Cv: BigInt,
    Cw: BigInt,
    C: BigInt,
    Cx: BigInt,
    Cz: BigInt,
    y1: BigInt,
    z1: BigInt,
    x: BigInt,
    y2: BigInt,
    z2: BigInt,
    z3: BigInt,

    y3: BigInt,
    y4: BigInt,
    y5: BigInt,
    y6: BigInt,
    y7: BigInt,
    y8: BigInt,
    y13: BigInt,

    z4: BigInt,
    z5: BigInt,
    z6: BigInt,
    z7: BigInt,
    z8: BigInt,
    z9: BigInt,
    z10: BigInt,
    z11: BigInt,
    z12: BigInt,

    y9: BigInt,
    y10: BigInt,
    y11: BigInt,
    y12: BigInt,

    z13: BigInt,
    z14: BigInt,
    z15: BigInt,
    z16: BigInt,
    z17: BigInt,
    z18: BigInt,
    z19: BigInt,

    rnym: BigInt,
}

use std::collections::HashMap;
//use serde_json;

fn export(params: Params) -> String {
    // jsonify the credential

    let mut credential = HashMap::new();
    credential.insert("Cs",params.Cs.to_string());
    credential.insert("Ce",params.Ce.to_string());
    credential.insert("Cw", params.Cw.to_string());
    credential.insert("Cx", params.Cx.to_string());
    credential.insert("Cv", params.Cv.to_string());
    credential.insert("Cz",params.Cz.to_string());
    credential.insert("y1",params.y1.to_string());
    credential.insert("y2",params.y2.to_string());
    credential.insert("y3",params.y3.to_string());
    credential.insert("y4",params.y4.to_string());
    credential.insert("y5",params.y5.to_string());
    credential.insert("y6",params.y6.to_string());
    credential.insert("y7",params.y7.to_string());
    credential.insert("y8",params.y8.to_string());
    credential.insert("y9",params.y9.to_string());
    credential.insert("y10",params.y10.to_string());
    credential.insert("y11",params.y11.to_string());
    credential.insert("y12",params.y12.to_string());
    credential.insert("y13",params.y13.to_string());
    credential.insert("z1",params.z1.to_string());
    credential.insert("z2",params.z2.to_string());
    credential.insert("z3",params.z3.to_string());
    credential.insert("z4",params.z4.to_string());
    credential.insert("z5",params.z5.to_string());
    credential.insert("z6",params.z6.to_string());
    credential.insert("z7",params.z7.to_string());
    credential.insert("z8",params.z8.to_string());
    credential.insert("z9",params.z9.to_string());
    credential.insert("z10",params.z10.to_string());
    credential.insert("z11",params.z11.to_string());
    credential.insert("z12",params.z12.to_string());
    credential.insert("z13",params.z13.to_string());
    credential.insert("z14",params.z14.to_string());
    credential.insert("z15",params.z15.to_string());
    credential.insert("z16",params.z16.to_string());
    credential.insert("z17",params.z17.to_string());
    credential.insert("z18",params.z18.to_string());
    credential.insert("z19",params.z19.to_string());
    credential.insert("rnym",params.rnym.to_string());
    serde_json::to_string(&credential).unwrap()
}

#[wasm_bindgen]
pub fn greet(s: &str, e: &str, v: &str, uk: &str, a: &str, b: &str, g: &str, h: &str, n: &str, exp: &str, gamma: &str, course_id: &str) -> String {
    let a = str2BigInt(a);
    let b = str2BigInt(b);
    let n = str2BigInt(n);
    let e = str2BigInt(e);
    let v = str2BigInt(v);
    let uk = str2BigInt(uk);
    let s = str2BigInt(s);

    let mut pr = Priv::new(&e.to_string());
    let mut rnym = Rnym::new(exp, gamma, course_id);
    let mut params = Params::default();
    let base = Base::new(g, h, n.clone());

    params.Cs = base.exp(&s, &pr.rs);
    params.Ce = base.exp(&e, &pr.re);
    params.Cw = base.exp(&pr.w, &pr.rw);
    params.Cx = base.exp(&uk, &pr.rx);
    params.Cv = v * base.g.modpow(&pr.w, &n) % &n;
    let z = &e * &pr.w;
    params.Cz = base.exp(&z, &pr.rz);


    params.y1 = _exp(&params.Cv, &pr.rn[1], &base.h, &pr.rn[2], &n);
    params.y2 = base.exp(&pr.rn[1], &pr.rn[3]);
    params.x = hashing_BigInt(&base.g * &base.h * &params.C * &params.Cv * &params.Cs *
        &params.Ce * &params.Cx * &params.Cz * &params.Cw);

    // update Priv.x
    pr.x = params.x.clone();


//    params.z1 = &pr.rn[1] + &params.x * &e;
//    params.z2 = &pr.rn[2] + &params.x * &pr.r;
//    params.z3 = &pr.rn[3] + &params.x * &pr.re;
    params.z1 = pr.fuzz(1, &e);
    params.z2 = pr.fuzz(2, &pr.r);
    params.z3 = pr.fuzz(3, &pr.re);

    params.y3 = (_exp(&a, &pr.rn[4], &b, &pr.rn[5], &n) *
        base.exp(&pr.rn[6], &pr.rn[7])) % &n;
    params.y4 = base.exp(&pr.rn[4], &pr.rn[8]);
    params.y5 = base.exp(&pr.rn[5], &pr.rn[9]);
    params.y6 = base.exp(&pr.rn[10], &pr.rn[11]);
    params.y7 = base.exp(&pr.rn[6], &pr.rn[12]);
    params.y8 = _exp(&params.Cv, &pr.rn[10], &base.h, &pr.rn[12], &n);
    params.y13 = rnym.grnym.modpow(&pr.rn[4], &rnym.gamma);

    params.z4 = pr.fuzz(4, &uk);
    params.z5 = pr.fuzz(5, &s);
    params.z6 = pr.fuzz(6, &pr.z);
    params.z7 = pr.fuzz(7, &pr.r);
    params.z8 = pr.fuzz(8, &pr.rx);
    params.z9 = pr.fuzz(9, &pr.rs);
    params.z10 = pr.fuzz(10, &e);
    params.z11 = pr.fuzz(11, &pr.re);
    params.z12 = pr.fuzz(12, &pr.rz);

    pr.r_ = &pr.rz - &e * &pr.rw;
    params.y9 = base.exp(&pr.rn[13], &pr.rn[14]);
    params.y10 = base.exp(&pr.rn[15], &pr.rn[16]);
    params.y11 = base.exp(&pr.rn[17], &pr.rn[18]);
    params.y12 = _exp(&params.Cw, &pr.rn[17], &base.h, &pr.rn[19], &n);

    params.z13 = pr.fuzz(13, &z);
    params.z14 = pr.fuzz(14, &pr.rz);
    params.z15 = pr.fuzz(15, &pr.w);
    params.z16 = pr.fuzz(16, &pr.rw);
    params.z17 = pr.fuzz(17, &e);
    params.z18 = pr.fuzz(18, &pr.re);
    params.z19 = pr.fuzz(19, &pr.r_);

    params.rnym = rnym.grnym.modpow(&uk, &rnym.gamma);

    export(params)
}


fn gen_random_32() -> String {
// tested!!
    let input: &[u8] = &[random()];
    format!("{:x}", Sha256::digest(input)).chars().take(8).collect()
}


//fn generate(s:)