use ark_ec::AffineCurve;
use ark_ec::ProjectiveCurve;
use ark_ff::field_new;
use ark_ff::BigInteger;
use ark_ff::BigInteger256;
use ark_ff::BigInteger384;
use ark_ff::One;
use ark_std::Zero;
use bandersnatch::{EdwardsAffine, EdwardsProjective, FrParameters};
use bandersnatch::{Fq, Fr};
use num_bigint::BigUint;
use std::convert::TryFrom;
use ark_std::cmp::max;
#[rustfmt::skip]
const COEFF_A1: Fq = field_new!(Fq, "16179988757916560824577558193084210236647645729299773892093730683504906651604");
#[rustfmt::skip]
const COEFF_A2: Fq = field_new!(Fq, "37446463827641770816307242315180085052603635617490163568005256780843403514036");
#[rustfmt::skip]
const COEFF_A3: Fq = field_new!(Fq, "14989411347484419663140498193005880785086916883037474254598401919095177670477");

#[rustfmt::skip]
const COEFF_B1: Fq = field_new!(Fq, "37446463827641770816307242315180085052603635617490163568005256780843403514036");
#[rustfmt::skip]
const COEFF_B2: Fq = field_new!(Fq, "36553259151239542273674161596529768046449890757310263666255995151154432137034");
#[rustfmt::skip]
const COEFF_B3: Fq = field_new!(Fq, "15882616023886648205773578911656197791240661743217374156347663548784149047479");

#[rustfmt::skip]
const COEFF_C1: Fq = field_new!(Fq, "42910309089382041158038545419309140955400939872179826051492616687477682993077");
#[rustfmt::skip]
const COEFF_C2: Fq = field_new!(Fq, "9525566085744149321409195088876824882289612628347811771111042012460898191436");

/// The modulus of the field.
/// MODULUS = 13108968793781547619861935127046491459309155893440570251786403306729687672801.
const MODULUS: BigInteger256 = BigInteger256([
    0x74fd06b52876e7e1,
    0xff8f870074190471,
    0x0cce760202687600,
    0x1cfb69d4ca675f52,
]);

/// (Self::MODULUS - 1) / 2
/// 6554484396890773809930967563523245729654577946720285125893201653364843836400
const MODULUS_MINUS_ONE_DIV_TWO: BigInteger256 = BigInteger256([
    0xba7e835a943b73f0,
    0x7fc7c3803a0c8238,
    0x06673b0101343b00,
    0xe7db4ea6533afa9,
]);

// N = Matrix(
// [[113482231691339203864511368254957623327,  10741319382058138887739339959866629956],
// [21482638764116277775478679919733259912, -113482231691339203864511368254957623327]])
#[rustfmt::skip]
const COEFF_N11: Fr = field_new!(Fr, "113482231691339203864511368254957623327");
#[rustfmt::skip]
const COEFF_N12: Fr = field_new!(Fr, "10741319382058138887739339959866629956");
#[rustfmt::skip]
const COEFF_N21: Fr = field_new!(Fr, "21482638764116277775478679919733259912");
#[rustfmt::skip]
const COEFF_N22: Fr = field_new!(Fr, "-113482231691339203864511368254957623327");

pub fn poor_man_glv(base: EdwardsAffine, scalar: Fr) -> EdwardsProjective {
    let psi_base = psi(&base);
    let (k1, k2) = get_decomposition(scalar);

    multi_scalar_mul(&base, &k1, &psi_base, &k2)
}

pub fn psi(base: &EdwardsAffine) -> EdwardsProjective {
    let mut x = base.x;
    let mut y = base.y;
    let mut z = y;

    // z = y;
    let fy = COEFF_A1 * (y + COEFF_A2) * (y + COEFF_A3);
    let gy = COEFF_B1 * (y + COEFF_B2) * (y + COEFF_B3);
    let hy = (y + COEFF_C1) * (y + COEFF_C2);

    x = x * fy * hy;
    y = gy * z;
    z = hy * z;

    EdwardsProjective::new(x, y, Fq::one(), z)
}

pub fn get_decomposition(scalar: Fr) -> (Fr, Fr) {
    let tmp: BigInteger256 = scalar.into();
    let scalar_z: BigUint = tmp.into();

    let tmp: BigInteger256 = COEFF_N11.into();
    let n11: BigUint = tmp.into();

    let tmp: BigInteger256 = COEFF_N12.into();
    let n12: BigUint = tmp.into();

    let r: BigUint = MODULUS.into();

    // beta = vector([n,0]) * self.curve.N_inv
    let beta_1 = scalar_z.clone() * n11;
    let beta_2 = scalar_z * n12;

    let beta_1 = beta_1 / r.clone();
    let beta_2 = beta_2 / r;

    // b = vector([int(beta[0]), int(beta[1])]) * self.curve.N
    let beta_1 = Fr::from(beta_1);
    let beta_2 = Fr::from(beta_2);
    let b1 = beta_1 * COEFF_N11 + beta_2 * COEFF_N21;
    let b2 = beta_1 * COEFF_N12 + beta_2 * COEFF_N22;

    let k1 = scalar - b1;
    let k2 = -b2;
    (k1, k2)
}

pub fn multi_scalar_mul(
    base: &EdwardsAffine,
    scalar_1: &Fr,
    endor_base: &EdwardsProjective,
    scalar_2: &Fr,
) -> EdwardsProjective {
    let mut b1 = (*base).into_projective();
    let mut s1 = *scalar_1;
    let mut b2 = *endor_base;
    let mut s2 = *scalar_2;

    if s1 > MODULUS_MINUS_ONE_DIV_TWO.into() {
        b1 = -b1;
        s1 = -s1;
    }
    if s2 > MODULUS_MINUS_ONE_DIV_TWO.into() {
        b2 = -b2;
        s2 = -s2;
    }
    let s1: BigInteger256 = s1.into();
    let s2: BigInteger256 = s2.into();

    let b1b2 = b1 + b2;

    let s1_bits = s1.to_bits_le();
    let s2_bits = s2.to_bits_le();
    let s1_len = get_bits(&s1_bits);
    let s2_len = get_bits(&s2_bits);
    // println!("{} {}", s1_len, s2_len);

    let mut res = EdwardsProjective::zero();
    for i in 0..max(s1_len, s2_len) as usize {
        res = res.double();
        // println!("{} {:?}", i, res);
        if s1_bits[i] && !s2_bits[i] {
            res += b1
        }
        if !s1_bits[i] && s2_bits[i] {
            res += b2
        }
        if s1_bits[i] && s2_bits[i] {
            res += b1b2
        }
    }
    res
}

fn get_bits(a: &[bool]) -> u16 {
    let mut res = 256;
    for e in a.iter().rev() {
        if !e {
            res -= 1;
        } else {
            return res;
        }
    }
    res
}

#[test]
fn test_psi() {
    use ark_ec::AffineCurve;
    use ark_ec::ProjectiveCurve;
    use ark_std::str::FromStr;

    let base_point = bandersnatch::EdwardsAffine::prime_subgroup_generator();
    let psi_point = bandersnatch::EdwardsAffine::from_str(
        "(3995099504672814451457646880854530097687530507181962222512229786736061793535, \
         33370049900732270411777328808452912493896532385897059012214433666611661340894)",
    )
    .unwrap();

    let t = psi(&base_point);
    assert_eq!(t.into_affine(), psi_point);
}

#[test]
fn test_decomp() {
    let scalar: Fr = field_new!(
        Fr,
        "4257185345094557079734489188109952172285839137338142340240392707284963971010"
    );
    let k1: Fr = field_new!(Fr, "30417741863887432744214758610616508258");
    let k2: Fr = field_new!(Fr, "-6406990765953933188067911864924578940");
    assert_eq!(get_decomposition(scalar), (k1, k2))
}

#[test]
fn test_msm() {
    use std::str::FromStr;
    let base_point = bandersnatch::EdwardsAffine::prime_subgroup_generator();
    let psi_point = bandersnatch::EdwardsAffine::from_str(
        "(3995099504672814451457646880854530097687530507181962222512229786736061793535, \
         33370049900732270411777328808452912493896532385897059012214433666611661340894)",
    )
    .unwrap();
    let t = psi(&base_point);
    assert_eq!(t.into_affine(), psi_point);

    let scalar: Fr = field_new!(
        Fr,
        "4257185345094557079734489188109952172285839137338142340240392707284963971010"
    );
    let k1: Fr = field_new!(Fr, "30417741863887432744214758610616508258");
    let k2: Fr = field_new!(Fr, "-6406990765953933188067911864924578940");
    assert_eq!(get_decomposition(scalar), (k1, k2));

    let res = bandersnatch::EdwardsAffine::from_str(
        "(6018810645516749504657411940673266094850700554607419759628157493373766067122, \
         13929928331741974885869757126422340790588975043986274897468601817898742989376)",
    )
    .unwrap();

    let tmp = base_point.mul(scalar);
    let res2 = multi_scalar_mul(&base_point, &k1, &psi_point.into_projective(), &k2).into_affine();

    assert_eq!(tmp.into_affine(), res);
    assert_eq!(res, res2);
}
