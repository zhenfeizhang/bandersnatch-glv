use ark_ff::field_new;
use ark_ff::One;
use bandersnatch::Fq;
use bandersnatch::{EdwardsAffine, EdwardsProjective, FrParameters};

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

pub fn poor_man_glv(base: EdwardsAffine, scalar: FrParameters) -> EdwardsProjective {
    todo!()
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
  
