use julian::*;
use math::*;
use sun_transit::*;
//use delta_time::*;

#[allow(dead_code)]
#[derive(PartialEq)]
enum Event {
    Moonrise,
    Moonset
}

fn get_time_of(event: Event, timestamp: i64, longitude: f64, latitude: f64) -> i64 {
    // Julian day
    let jd = (unix_to_julian(timestamp) + longitude / 360.0 + 0.5).floor() - 0.5;

    // Julian century
    let t = jde_to_julian_century(jd);

    // Mean longitude of the Moon
    // (L')
    let lm = 218.316_4477 + 481_267.881234_21 * t
           - 0.0015786 * t.powi(2)
           + t.powi(3) / 538_841.0
           - t.powi(4) / 65_194_000.0;

    // Mean elongation of the Moon
    // (D)
    let d = 297.850_1921 + 445_267.1114034 * t
          - 0.001_8819 * t.powi(2)
          + t.powi(3) / 545_868.0 - t.powi(4) / 113_065_000.0;

    // Sun mean anomaly
    // (M)
    let sm = 357.529_1092 + 35_999.050_2909 * t
           - 0.000_1536 * t.powi(2)
           + t.powi(3) / 24_490_000.0;

    // Moon mean anomaly
    // (M')
    let mm = 134.963_3964 + 477_198.867_5055 * t
           + 0.008_7414 * t.powi(2)
           - t.powi(3) / 69_699.0
           - t.powi(4) / 14_712_000.0;

    // Moon argument of latitude
    // (F)
    let f = 93.272_0950 + 483_202.017_5233 * t
          - 0.003_6539 * t.powi(2)
          - t.powi(3) / 3_526_000.0
          + t.powi(4) / 863_310_000.0;

    let lm = modulo(lm, 360.0);
    let sm = modulo(sm, 360.0);
    let mm = modulo(mm, 360.0);
    let d = modulo(d, 360.0);
    let f = modulo(f, 360.0);

    let a1 = modulo(119.75 + 131.849 * t,     360.0);
    let a2 = modulo( 53.09 + 479_264.290 * t, 360.0);
    let a3 = modulo(313.45 + 481_266.484 * t, 360.0);

    // Periodic terms for the longitude, distance and latitude of the Moon
    //     D,    M,   M',   F,        sine,         cosine,        sine
    let terms = vec![
        (0.0,  0.0,  1.0,  0.0, 6_288_774.0, -20_905_355.0,         0.0),
        (2.0,  0.0, -1.0,  0.0, 1_274_027.0,  -3_699_111.0,         0.0),
        (2.0,  0.0,  0.0,  0.0,   658_314.0,  -2_955_968.0,         0.0),
        (0.0,  0.0,  2.0,  0.0,   213_618.0,    -569_925.0,         0.0),
        (0.0,  1.0,  0.0,  0.0,  -185_116.0,      48_888.0,         0.0),
        (0.0,  0.0,  0.0,  2.0,  -114_332.0,      -3_149.0,         0.0),
        (2.0,  0.0, -2.0,  0.0,    58_793.0,     246_158.0,         0.0),
        (2.0, -1.0, -1.0,  0.0,    57_066.0,    -152_138.0,         0.0),
        (2.0,  0.0,  1.0,  0.0,    53_322.0,    -170_733.0,         0.0),
        (2.0, -1.0,  0.0,  0.0,    45_758.0,    -204_586.0,         0.0),
        (0.0,  1.0, -1.0,  0.0,   -40_923.0,    -129_620.0,         0.0),
        (1.0,  0.0,  0.0,  0.0,   -34_720.0,     108_743.0,         0.0),
        (0.0,  1.0,  1.0,  0.0,   -30_383.0,     104_755.0,         0.0),
        (2.0,  0.0,  0.0, -2.0,    15_327.0,      10_321.0,         0.0),
        (0.0,  0.0,  1.0,  2.0,   -12_528.0,           0.0,         0.0),
        (0.0,  0.0,  1.0, -2.0,    10_980.0,      79_661.0,         0.0),
        (4.0,  0.0, -1.0,  0.0,    10_675.0,     -34_782.0,         0.0),
        (0.0,  0.0,  3.0,  0.0,    10_034.0,     -23_210.0,         0.0),
        (4.0,  0.0, -2.0,  0.0,     8_548.0,     -21_636.0,         0.0),
        (2.0,  1.0, -1.0,  0.0,    -7_888.0,      24_208.0,         0.0),
        (2.0,  1.0,  0.0,  0.0,    -6_766.0,      30_824.0,         0.0),
        (1.0,  0.0, -1.0,  0.0,    -5_163.0,      -8_379.0,         0.0),
        (1.0,  1.0,  0.0,  0.0,     4_987.0,     -16_675.0,         0.0),
        (2.0, -1.0,  1.0,  0.0,     4_036.0,     -12_831.0,         0.0),
        (2.0,  0.0,  2.0,  0.0,     3_994.0,     -10_445.0,         0.0),
        (4.0,  0.0,  0.0,  0.0,     3_861.0,     -11_650.0,         0.0),
        (2.0,  0.0, -3.0,  0.0,     3_665.0,      14_403.0,         0.0),
        (0.0,  1.0, -2.0,  0.0,    -2_689.0,      -7_003.0,         0.0),
        (2.0,  0.0, -1.0,  2.0,    -2_602.0,           0.0,         0.0),
        (2.0, -1.0, -2.0,  0.0,     2_390.0,      10_056.0,         0.0),
        (1.0,  0.0,  1.0,  0.0,    -2_348.0,       6_322.0,         0.0),
        (2.0, -2.0,  0.0,  0.0,     2_236.0,      -9_884.0,         0.0),

        (0.0,  1.0,  2.0,  0.0,    -2_120.0,       5_751.0,         0.0),
        (0.0,  2.0,  0.0,  0.0,    -2_069.0,           0.0,         0.0),
        (2.0, -2.0, -1.0,  0.0,     2_048.0,      -4_950.0,         0.0),
        (2.0,  0.0,  1.0, -2.0,    -1_773.0,       4_130.0,         0.0),
        (2.0,  0.0,  0.0,  2.0,    -1_595.0,           0.0,         0.0),
        (4.0, -1.0, -1.0,  0.0,     1_215.0,      -3_958.0,         0.0),
        (0.0,  0.0,  2.0,  2.0,    -1_110.0,           0.0,         0.0),
        (3.0,  0.0, -1.0,  0.0,      -892.0,       3_258.0,         0.0),
        (2.0,  1.0,  1.0,  0.0,      -810.0,       2_616.0,         0.0),
        (4.0, -1.0, -2.0,  0.0,       759.0,      -1_897.0,         0.0),
        (0.0,  2.0, -1.0,  0.0,      -713.0,      -2_117.0,         0.0),
        (2.0,  2.0, -1.0,  0.0,      -700.0,       2_354.0,         0.0),
        (2.0,  1.0, -2.0,  0.0,       691.0,           0.0,         0.0),
        (2.0, -1.0,  0.0, -2.0,       596.0,           0.0,         0.0),
        (4.0,  0.0,  1.0,  0.0,       549.0,      -1_423.0,         0.0),
        (0.0,  0.0,  4.0,  0.0,       537.0,      -1_117.0,         0.0),
        (4.0, -1.0,  0.0,  0.0,       520.0,      -1_571.0,         0.0),
        (1.0,  0.0, -2.0,  0.0,      -487.0,      -1_739.0,         0.0),
        (2.0,  1.0,  0.0, -2.0,      -399.0,           0.0,         0.0),
        (0.0,  0.0,  2.0, -2.0,      -381.0,      -4_421.0,         0.0),
        (1.0,  1.0,  1.0,  0.0,       351.0,           0.0,         0.0),
        (3.0,  0.0, -2.0,  0.0,      -340.0,           0.0,         0.0),
        (4.0,  0.0, -3.0,  0.0,       330.0,           0.0,         0.0),
        (2.0, -1.0,  2.0,  0.0,       327.0,           0.0,         0.0),
        (0.0,  2.0,  1.0,  0.0,      -323.0,       1_165.0,         0.0),
        (1.0,  1.0, -1.0,  0.0,       299.0,           0.0,         0.0),
        (2.0,  0.0,  3.0,  0.0,       294.0,           0.0,         0.0),
        (2.0,  0.0, -1.0, -2.0,         0.0,       8_752.0,         0.0),

        (0.0,  0.0,  0.0,  1.0,         0.0,           0.0, 5_128_122.0),
        (0.0,  0.0,  1.0,  1.0,         0.0,           0.0,   280_602.0),
        (0.0,  0.0,  1.0, -1.0,         0.0,           0.0,   277_693.0),
        (2.0,  0.0,  0.0, -1.0,         0.0,           0.0,   173_237.0),
        (2.0,  0.0, -1.0,  1.0,         0.0,           0.0,    55_413.0),
        (2.0,  0.0, -1.0, -1.0,         0.0,           0.0,    46_271.0),
        (2.0,  0.0,  0.0,  1.0,         0.0,           0.0,    32_573.0),
        (0.0,  0.0,  2.0,  1.0,         0.0,           0.0,    17_198.0),
        (2.0,  0.0,  1.0, -1.0,         0.0,           0.0,     9_266.0),
        (0.0,  0.0,  2.0, -1.0,         0.0,           0.0,     8_822.0),
        (2.0, -1.0,  0.0, -1.0,         0.0,           0.0,     8_216.0),
        (2.0,  0.0, -2.0, -1.0,         0.0,           0.0,     4_324.0),
        (2.0,  0.0,  1.0,  1.0,         0.0,           0.0,     4_200.0),
        (2.0,  1.0,  0.0, -1.0,         0.0,           0.0,    -3_359.0),
        (2.0, -1.0, -1.0,  1.0,         0.0,           0.0,     2_463.0),
        (2.0, -1.0,  0.0,  1.0,         0.0,           0.0,     2_211.0),
        (2.0, -1.0, -1.0, -1.0,         0.0,           0.0,     2_065.0),
        (0.0,  1.0, -1.0, -1.0,         0.0,           0.0,    -1_870.0),
        (4.0,  0.0, -1.0, -1.0,         0.0,           0.0,     1_828.0),
        (0.0,  1.0,  0.0,  1.0,         0.0,           0.0,    -1_794.0),
        (0.0,  0.0,  0.0,  3.0,         0.0,           0.0,    -1_749.0),
        (0.0,  1.0, -1.0,  1.0,         0.0,           0.0,    -1_565.0),
        (1.0,  0.0,  0.0,  1.0,         0.0,           0.0,    -1_491.0),
        (0.0,  1.0,  1.0,  1.0,         0.0,           0.0,    -1_475.0),
        (0.0,  1.0,  1.0, -1.0,         0.0,           0.0,    -1_410.0),
        (0.0,  1.0,  0.0, -1.0,         0.0,           0.0,    -1_344.0),
        (1.0,  0.0,  0.0, -1.0,         0.0,           0.0,    -1_335.0),
        (0.0,  0.0,  3.0,  1.0,         0.0,           0.0,     1_107.0),
        (4.0,  0.0,  0.0, -1.0,         0.0,           0.0,     1_021.0),
        (4.0,  0.0, -1.0,  1.0,         0.0,           0.0,       833.0),

        (0.0,  0.0,  1.0, -3.0,         0.0,           0.0,       777.0),
        (4.0,  0.0, -2.0,  1.0,         0.0,           0.0,       671.0),
        (2.0,  0.0,  0.0, -3.0,         0.0,           0.0,       607.0),
        (2.0,  0.0,  2.0, -1.0,         0.0,           0.0,       596.0),
        (2.0, -1.0,  1.0, -1.0,         0.0,           0.0,       491.0),
        (2.0,  0.0, -2.0,  1.0,         0.0,           0.0,      -451.0),
        (0.0,  0.0,  3.0, -1.0,         0.0,           0.0,       439.0),
        (2.0,  0.0,  2.0,  1.0,         0.0,           0.0,       422.0),
        (2.0,  0.0, -3.0, -1.0,         0.0,           0.0,       421.0),
        (2.0,  1.0, -1.0,  1.0,         0.0,           0.0,      -366.0),
        (2.0,  1.0,  0.0,  1.0,         0.0,           0.0,      -351.0),
        (4.0,  0.0,  0.0,  1.0,         0.0,           0.0,       331.0),
        (2.0, -1.0,  1.0,  1.0,         0.0,           0.0,       315.0),
        (2.0, -2.0,  0.0, -1.0,         0.0,           0.0,       302.0),
        (0.0,  0.0,  1.0,  3.0,         0.0,           0.0,      -283.0),
        (2.0,  1.0,  1.0, -1.0,         0.0,           0.0,      -229.0),
        (1.0,  1.0,  0.0, -1.0,         0.0,           0.0,       223.0),
        (1.0,  1.0,  0.0,  1.0,         0.0,           0.0,       223.0),
        (0.0,  1.0, -2.0, -1.0,         0.0,           0.0,      -220.0),
        (2.0,  1.0, -1.0, -1.0,         0.0,           0.0,      -220.0),
        (1.0,  0.0,  1.0,  1.0,         0.0,           0.0,      -185.0),
        (2.0, -1.0, -2.0, -1.0,         0.0,           0.0,       181.0),
        (0.0,  1.0,  2.0,  1.0,         0.0,           0.0,      -177.0),
        (4.0,  0.0, -2.0, -1.0,         0.0,           0.0,       176.0),
        (4.0, -1.0, -1.0, -1.0,         0.0,           0.0,       166.0),
        (1.0,  0.0,  1.0, -1.0,         0.0,           0.0,      -164.0),
        (4.0,  0.0,  1.0, -1.0,         0.0,           0.0,       132.0),
        (1.0,  0.0, -1.0, -1.0,         0.0,           0.0,      -119.0),
        (4.0, -1.0,  0.0, -1.0,         0.0,           0.0,       115.0),
        (2.0, -2.0,  0.0,  1.0,         0.0,           0.0,       107.0)
    ];

    // (Σl)
    let mut el = 0.0;

    // (Σr)
    let mut er = 0.0;

    // (Σr)
    let mut eb = 0.0;

    let e = 1.0 - 0.002_516 * t - 0.000_0074 * t.powi(2);

    for (d_arg, sm_arg, mm_arg, f_arg, sin_arg, cos_arg, sin_arg2) in terms {
        let arg = d * d_arg
                + f * f_arg
                + sm * sm_arg
                + mm * mm_arg;

        let cor = match sm_arg {
            -1.0 => e,
             1.0 => e,
            -2.0 => e * e,
             2.0 => e * e,
               _ => 1.0
        };

        el += sin_arg  * cor * sin_deg(arg);
        er += cos_arg  * cor * cos_deg(arg);
        eb += sin_arg2 * cor * sin_deg(arg);
    }

    el = el
       + 3958.0 * sin_deg(a1)
       + 1962.0 * sin_deg(lm - f)
       +  318.0 * sin_deg(a2);

    eb = eb
       - 2235.0 * sin_deg(lm)
       +  382.0 * sin_deg(a3)
       +  175.0 * sin_deg(a1 - f)
       +  175.0 * sin_deg(a1 + f)
       +  127.0 * sin_deg(lm - mm)
       -  115.0 * sin_deg(lm + mm);

    // (λ)
    let l = lm + el / 1_000_000.0;
    
    // (β)
    let b = eb / 1_000_000.0;

    // (Δ)
    let delta = 385_000.56 + er / 1000.0;


    let (nl, no) = nutation(t);

    // Mean obliquity of the eliptic
    // (ε0)
    let e0 = mean_obliquity_eliptic(t);

    // True obliquity of the eliptic
    // (ε)
    let ep = e0 + no;

    // Apparent λ
    let l = l + nl;

    // Moon apparent right ascension
    // (α)
    let a_x = sin_deg(l) * cos_deg(ep) - tan_deg(b) * sin_deg(ep);
    let a_y = cos_deg(l);
    let a = modulo(atan2_deg(a_x, a_y), 360.0); // TODO: Verify this modulo

    // Moon apparent declinaison
    // (δ)
    let s_x = sin_deg(b) * cos_deg(ep) + cos_deg(b) * sin_deg(ep) * sin_deg(l);
    let s = modulo(asin_deg(s_x), 180.0); // TODO: Verify this modulo

    /*
    println!("");
    println!("DEBUG: JDE  = {}", jd);
    println!("DEBUG:   T  = {}", t);
    println!("DEBUG:   L' = {}", lm);
    println!("DEBUG:   D  = {}", d);
    println!("DEBUG:   M  = {}", sm);
    println!("DEBUG:   M' = {}", mm);
    println!("DEBUG:   F  = {}", f);
    println!("");
    println!("DEBUG:  A1  = {}", a1);
    println!("DEBUG:  A2  = {}", a2);
    println!("DEBUG:  A3  = {}", a3);
    println!("DEBUG:   E  = {}", e);
    println!("DEBUG:  el  = {}", el);
    println!("DEBUG:  eb  = {}", eb);
    println!("DEBUG:  er  = {}", er);
    println!("");
    println!("DEBUG:   l  = {}", l);
    println!("DEBUG:   b  = {}", b);
    println!("DEBUG:   d  = {}", delta);
    println!("DEBUG:   ep = {}", ep);
    println!("DEBUG:   a  = {}", a);
    println!("DEBUG:   s  = {}", s);
    */

    // Moon horizontal parallax
    let p = asin_deg(6378.14 / delta);

    //let h0 = 0.125; // Low accuracy
    let h0 = 0.7275 * p - dec_deg(0.0, 34.0, 0.0);

    // H0
    let hh0_1 = sin_deg(h0);
    let hh0_2 = sin_deg(latitude) * sin_deg(s); // TODO: Should be between -1..1
    let hh0 = acos_deg((hh0_1 - hh0_2) / cos_deg(latitude) * cos_deg(s));
    let hh0 = modulo(hh0, 180.0);
    /*
    println!("DEBUG: hh0_2 = {}", hh0_2);
    println!("DEBUG: hh0   = {}", hh0);
    */

    // Apparent sideral time at 0h Universal Time on day D for the meridian
    // of Greenwich converted into degree.
    // (formula 12.3)
    // (Θ0)
    let ast = 100.460_618_37
            + 36_000.770_053_608 * t
            + 0.000_387_933 * t.powi(2)
            - t.powi(3) / 38_710_000.0;
    let ast = modulo(ast, 360.0);

    let m0 = (a - longitude - ast) / 360.0;

    let m0 = modulo(m0, 1.0); // Fraction of a day

    let m = match event {
        Event::Moonrise => m0 - hh0 / 360.0,
        Event::Moonset  => m0 + hh0 / 360.0
    };

    //let m = modulo(m, 1.0); // Fraction of a day

    // Sideral time at Greenwich in degree
    // (θ0)
    let st = ast + 360.985_647 * m;
    //let st = modulo(st, 360.0); // FIXME: ???

    // NOTE: In the next calculations we should interpolate α and δ from the
    // previous and the following days.

    // Local hour angle of the Moon
    // (H)
    let hh = st + longitude - a;
    //let hh = modulo(hh, 360.0) - 180.0; // FIXME: ???

    // Moon altitude
    // (formula 13.6)
    // (h)
    let h = asin_deg(sin_deg(latitude) * sin_deg(s) + cos_deg(latitude) * cos_deg(s) * cos_deg(hh));

    // Correction to m in case of rising and setting
    let dm = (h - h0) / (360.0 * cos_deg(s) * cos_deg(latitude) * sin_deg(hh));

    let m = m + dm;

    /*
    println!("");
    println!("DEBUG: ast  = {}", ast);
    println!("DEBUG:  st  = {}", st);
    println!("DEBUG:  hh  = {}", hh);
    println!("DEBUG:   h  = {}", h);
    println!("DEBUG:  h0  = {}", h0);
    println!("DEBUG:  m0  = {}", m0);
    println!("DEBUG:   m  = {}", m);
    println!("DEBUG:  dm  = {}", dm);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    */

    //terrestrial_to_universal_time(julian_to_unix(jd + m))
    julian_to_unix(jd + m)
}

pub fn get_moonrise(timestamp: i64, longitude: f64, latitude: f64) -> i64 {
    get_time_of(Event::Moonrise, timestamp, longitude, latitude)
}

pub fn get_moonset(timestamp: i64, longitude: f64, latitude: f64) -> i64 {
    get_time_of(Event::Moonset, timestamp, longitude, latitude)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::*;

    #[test]
    fn get_moonrise_test() {
        let accuracy = 2000; // FIXME: Improve accuracy
        let times = vec![
            ("2015-06-21T09:12:30+00:00", "2015-06-21T12:00:00+00:00", 45.0, 0.0)
            //("1988-03-20T00:00:00+00:00", "1988-03-20T00:00:00+00:00", 71.0833, 42.3333),
            //("1992-04-12T00:00:00+00:00", "1992-04-12T00:00:00+00:00", 45.0, 0.0)
        ];
        for (t0, t1, lat, lon) in times {
            assert_approx_eq!(parse_time(t0), get_moonrise(parse_time(t1), lon, lat), accuracy);
        }
    }
}
