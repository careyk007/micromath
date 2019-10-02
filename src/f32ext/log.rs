/// log_b(a) approximation for f32
use super::ln;

pub(super) fn log_ln_approx(x: f32, base: f32) -> f32 {
    //using change of base log_base(x) = ln(x)/ln(base)
    let fract_base_ln = 1.0 / ln::ln_1to2_series_approximation(base);
    let value_ln = ln::ln_1to2_series_approximation(x);
    value_ln * fract_base_ln
}

#[cfg(test)]
mod tests {
    use super::super::abs;
    use super::log_ln_approx;
    pub(crate) const MAX_ERROR: f32 = 0.001;
    /// log3(x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS_BASE3: &[(f32, f32)] = &[
        (1e-20, -41.91806433175008),
        (1e-19, -39.82216059431951),
        (1e-18, -37.726260329176036),
        (1e-17, -35.63035659174547),
        (1e-16, -33.5344528543149),
        (1e-15, -31.43854911688433),
        (1e-14, -29.342645379453764),
        (1e-13, -27.24674337816674),
        (1e-12, -25.150839640736177),
        (1e-11, -23.05493590330561),
        (1e-10, -20.95903216587504),
        (1e-09, -18.863130164588018),
        (1e-08, -16.76722642715745),
        (1e-07, -14.671322689726882),
        (1e-06, -12.575419820368088),
        (1e-05, -10.47951608293752),
        (1e-04, -8.383613213578725),
        (0.001, -6.287709910184044),
        (0.01, -4.1918066067893625),
        (0.1, -2.0959033033946812),
        (10.0, 2.0959033033946812),
        (100.0, 4.1918066067893625),
        (1000.0, 6.287709910184044),
        (10000.0, 8.383613213578725),
        (100000.0, 10.47951608293752),
        (1000000.0, 12.575419820368088),
        (10000000.0, 14.671322689726882),
        (100000000.0, 16.76722642715745),
        (1000000000.0, 18.863130164588018),
        (10000000000.0, 20.95903216587504),
        (100000000000.0, 23.05493590330561),
        (1000000000000.0, 25.150839640736177),
        (10000000000000.0, 27.24674337816674),
        (100000000000000.0, 29.342645379453764),
        (1000000000000000.0, 31.43854911688433),
        (1e+16, 33.5344528543149),
        (1e+17, 35.63035659174547),
        (1e+18, 37.726260329176036),
        (1e+19, 39.82216059431951),
    ];

    /// log5.5(x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS_BASE5_5: &[(f32, f32)] = &[
        (1e-20, -27.013786260685983),
        (1e-19, -25.663096611998228),
        (1e-18, -24.312409201000182),
        (1e-17, -22.961719552312427),
        (1e-16, -21.611029903624672),
        (1e-15, -20.260340254936917),
        (1e-14, -18.90965060624916),
        (1e-13, -17.55896207640626),
        (1e-12, -16.208272427718505),
        (1e-11, -14.857582779030748),
        (1e-10, -13.506893130342991),
        (1e-09, -12.156204600500091),
        (1e-08, -10.805514951812336),
        (1e-07, -9.45482530312458),
        (1e-06, -8.104136213859253),
        (1e-05, -6.753446565171496),
        (1e-04, -5.402757475906168),
        (0.001, -4.052068106929626),
        (0.01, -2.701378737953084),
        (0.1, -1.350689368976542),
        (10.0, 1.350689368976542),
        (100.0, 2.701378737953084),
        (1000.0, 4.052068106929626),
        (10000.0, 5.402757475906168),
        (100000.0, 6.753446565171496),
        (1000000.0, 8.104136213859253),
        (10000000.0, 9.45482530312458),
        (100000000.0, 10.805514951812336),
        (1000000000.0, 12.156204600500091),
        (10000000000.0, 13.506893130342991),
        (100000000000.0, 14.857582779030748),
        (1000000000000.0, 16.208272427718505),
        (10000000000000.0, 17.55896207640626),
        (100000000000000.0, 18.90965060624916),
        (1000000000000000.0, 20.260340254936917),
        (1e+16, 21.611029903624672),
        (1e+17, 22.961719552312427),
        (1e+18, 24.312409201000182),
        (1e+19, 25.663096611998228),
    ];
    /// log12.7(x) test vectors - `(input, output)`
    pub(crate) const TEST_VECTORS_BASE12_7: &[(f32, f32)] = &[
        (1e-20, -18.119162917899015),
        (1e-19, -17.213204546868663),
        (1e-18, -16.307247676740996),
        (1e-17, -15.401289305710643),
        (1e-16, -14.495330934680288),
        (1e-15, -13.589372563649935),
        (1e-14, -12.68341419261958),
        (1e-13, -11.77745657204057),
        (1e-12, -10.871498201010217),
        (1e-11, -9.965539829979862),
        (1e-10, -9.059581458949507),
        (1e-09, -8.153623838370498),
        (1e-08, -7.247665467340144),
        (1e-07, -6.34170709630979),
        (1e-06, -5.435749100505109),
        (1e-05, -4.529790729474754),
        (1e-04, -3.623832733670072),
        (0.001, -2.7178745502525543),
        (0.01, -1.811916366835036),
        (0.1, -0.905958183417518),
        (10.0, 0.905958183417518),
        (100.0, 1.811916366835036),
        (1000.0, 2.7178745502525543),
        (10000.0, 3.623832733670072),
        (100000.0, 4.529790729474754),
        (1000000.0, 5.435749100505109),
        (10000000.0, 6.34170709630979),
        (100000000.0, 7.247665467340144),
        (1000000000.0, 8.153623838370498),
        (10000000000.0, 9.059581458949507),
        (100000000000.0, 9.965539829979862),
        (1000000000000.0, 10.871498201010217),
        (10000000000000.0, 11.77745657204057),
        (100000000000000.0, 12.68341419261958),
        (1000000000000000.0, 13.589372563649935),
        (1e+16, 14.495330934680288),
        (1e+17, 15.401289305710643),
        (1e+18, 16.307247676740996),
        (1e+19, 17.213204546868663),
    ];

    #[test]
    fn sanity_check() {
        assert_eq!(log_ln_approx(1_f32, 3.0), 0_f32);
        assert_eq!(log_ln_approx(1_f32, 5.5), 0_f32);
        assert_eq!(log_ln_approx(1_f32, 12.7), 0_f32);
        for (x, expected) in TEST_VECTORS_BASE3 {
            let log_x = log_ln_approx(*x, 3.0);
            let relative_error = abs::abs(log_x - *expected) / *expected;

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large: {} vs {}",
                relative_error,
                log_x,
                expected
            );
        }

        for (x, expected) in TEST_VECTORS_BASE5_5 {
            let log_x = log_ln_approx(*x, 5.5);
            let relative_error = abs::abs(log_x - *expected) / *expected;

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large: {} vs {}",
                relative_error,
                log_x,
                expected
            );
        }

        for (x, expected) in TEST_VECTORS_BASE12_7 {
            let log_x = log_ln_approx(*x, 12.7);
            let relative_error = abs::abs(log_x - *expected) / *expected;

            assert!(
                relative_error <= MAX_ERROR,
                "relative_error {} too large: {} vs {}",
                relative_error,
                log_x,
                expected
            );
        }
    }
}