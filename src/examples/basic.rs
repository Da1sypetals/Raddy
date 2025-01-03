use approx::assert_abs_diff_eq;
use nalgebra::{Const, SVector};
use raddy::make;
use raddy::Ad;
use rand::{thread_rng, Rng};

const EPS: f64 = 1e-10;

fn main() {
    // 1.
    // ################ scalar ################
    let mut rng = thread_rng();
    let val = rng.gen_range(0.0..10.0);

    let var = make::ad(val);
    let var = &var;
    let y = var.sin() * var + var.ln();
    let g = val * val.cos() + val.sin() + val.recip();
    let h = -val * val.sin() + 2.0 * val.cos() - val.powi(-2);

    assert_abs_diff_eq!(y.grad()[(0, 0)], g, epsilon = EPS);
    assert_abs_diff_eq!(y.hess()[(0, 0)], h, epsilon = EPS);

    // 2.
    // ############################# Matrix #############################

    const N_TEST_MAT_4: usize = 4;
    type NaConst = Const<N_TEST_MAT_4>;
    const N_VEC_4: usize = N_TEST_MAT_4 * N_TEST_MAT_4;

    let vals: &[f64] = &(0..N_VEC_4)
        .map(|_| rng.gen_range(-4.0..4.0))
        .collect::<Vec<_>>();

    let s: SVector<Ad<N_VEC_4>, N_VEC_4> = make::vec(vals);
    let z = s
        .clone()
        // This reshape is COL MAJOR!!!!!!!!!!!!!
        .reshape_generic(NaConst {}, NaConst {})
        .transpose();

    let det = z.determinant();
    let _grad = det.grad();
    let _hess = det.hess();
    // core logic ends ####################################################

    // correctness
    // let expected_grad = grad_det4(
    //     vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7], vals[8], vals[9],
    //     vals[10], vals[11], vals[12], vals[13], vals[14], vals[15],
    // );
    // let g_diff = (expected_grad - det.grad()).norm_squared();
    // assert_abs_diff_eq!(g_diff, 0.0, epsilon = EPS);

    // let expected_hess = hess_det4(
    //     vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7], vals[8], vals[9],
    //     vals[10], vals[11], vals[12], vals[13], vals[14], vals[15],
    // );
    // let h_diff = (det.hess() - expected_hess).norm_squared();
    // assert_abs_diff_eq!(h_diff, 0.0, epsilon = EPS);
}
