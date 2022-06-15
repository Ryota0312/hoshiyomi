use std::f32::consts::PI;

const ZONE_OFFSET: f64 = 0.0;

fn main() {
    println!("{}", get_moon_longitude(1999, 11, 14, 0, 0, 0));
}

/**
 * year年month月day日0時のJ2000.0(2000年１月１日力学時正午)からの経過日数
 */
fn j2000day(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: i32) -> f64 {
    let mut fixed_year = (year - 2000) as f64;
    let mut fixed_month = month as f64;
    let fixed_day = day as f64;
    if month <= 2 {
        fixed_month = (month + 12) as f64;
        fixed_year -= 1.0;
    }
    let t = (hour as f64 * 60.0 * 60.0 + min as f64 * 60.0 + sec as f64) / 86400.0;

    // 地球の自転遅れ補正
    let rotate_rev = (57.0 + 0.8 * (year as f64 - 1990.0)) / 86400.0;

    365.0 * fixed_year + 30.0 * fixed_month + fixed_day - 33.5 - (ZONE_OFFSET / 24.0)
        + (3.0 * (fixed_month + 1.0) / 5.0).floor()
        + (fixed_year / 4.0).floor()
        + t
        + rotate_rev
}

/**
 * year年month月day日0時のJ2000.0(2000年１月１日力学時正午)からの経過年数
 */
fn j2000year(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: i32) -> f64 {
    j2000day(year, month, day, hour, min, sec) / 365.25
}

/**
 * 月の黄経の近似計算
 */
fn get_moon_longitude(year: i32, month: i32, day: i32, hour: i32, min: i32, sec: i32) -> f64 {
    let t = j2000year(year, month, day, hour, min, sec);
    let am = 0.0040 * deg2rad(119.5 + 1.33 * t).sin()
        + 0.0020 * deg2rad(55.0 + 19.34 * t).sin()
        + 0.0006 * deg2rad(71.0 + 0.2 * t).sin()
        + 0.0006 * deg2rad(54.0 + 19.3 * t).sin();

    let lm = 218.3161
        + 4812.67881 * t
        + 6.2887 * deg2rad(134.961 + 4771.9886 * t + am).sin()
        + 1.2740 * deg2rad(100.738 + 4133.3536 * t).sin()
        + 0.6583 * deg2rad(235.700 + 8905.3422 * t).sin()
        + 0.2136 * deg2rad(269.926 + 9543.9773 * t).sin()
        + 0.1856 * deg2rad(177.525 + 359.9905 * t).sin()
        + 0.1143 * deg2rad(6.546 + 9664.0404 * t).sin()
        + 0.0588 * deg2rad(214.22 + 638.635 * t).sin()
        + 0.0572 * deg2rad(103.21 + 3773.363 * t).sin()
        + 0.0533 * deg2rad(10.66 + 13677.331 * t).sin()
        + 0.0459 * deg2rad(238.18 + 8545.352 * t).sin()
        + 0.0410 * deg2rad(137.43 + 4411.998 * t).sin()
        + 0.0348 * deg2rad(117.84 + 4452.671 * t).sin()
        + 0.0305 * deg2rad(312.49 + 5131.979 * t).sin()
        + 0.0153 * deg2rad(130.84 + 758.698 * t).sin()
        + 0.0125 * deg2rad(141.51 + 14436.029 * t).sin()
        + 0.0110 * deg2rad(231.59 + 4892.052 * t).sin()
        + 0.0107 * deg2rad(336.44 + 13038.696 * t).sin()
        + 0.0100 * deg2rad(44.89 + 14315.966 * t).sin()
        + 0.0085 * deg2rad(201.5 + 8266.71 * t).sin()
        + 0.0079 * deg2rad(278.2 + 4493.34 * t).sin()
        + 0.0068 * deg2rad(53.2 + 9265.33 * t).sin()
        + 0.0052 * deg2rad(197.2 + 319.32 * t).sin()
        + 0.0050 * deg2rad(295.4 + 4812.66 * t).sin()
        + 0.0048 * deg2rad(235.0 + 19.34 * t).sin()
        + 0.0040 * deg2rad(13.2 + 13317.34 * t).sin()
        + 0.0040 * deg2rad(145.6 + 18449.32 * t).sin()
        + 0.0040 * deg2rad(119.5 + 1.33 * t).sin()
        + 0.0039 * deg2rad(111.3 + 17810.68 * t).sin()
        + 0.0037 * deg2rad(349.1 + 5410.62 * t).sin()
        + 0.0027 * deg2rad(272.5 + 9183.99 * t).sin()
        + 0.0026 * deg2rad(107.2 + 13797.39 * t).sin()
        + 0.0024 * deg2rad(211.9 + 998.63 * t).sin()
        + 0.0024 * deg2rad(252.8 + 9224.66 * t).sin()
        + 0.0022 * deg2rad(240.6 + 8185.36 * t).sin()
        + 0.0021 * deg2rad(87.5 + 9903.97 * t).sin()
        + 0.0021 * deg2rad(175.1 + 719.98 * t).sin()
        + 0.0021 * deg2rad(105.6 + 3413.37 * t).sin()
        + 0.0020 * deg2rad(55.0 + 19.34 * t).sin()
        + 0.0018 * deg2rad(4.1 + 4013.29 * t).sin()
        + 0.0016 * deg2rad(242.2 + 18569.38 * t).sin()
        + 0.0012 * deg2rad(339.0 + 12678.71 * t).sin()
        + 0.0011 * deg2rad(276.5 + 19208.02 * t).sin()
        + 0.0009 * deg2rad(218.0 + 8586.0 * t).sin()
        + 0.0008 * deg2rad(188.0 + 14037.3 * t).sin()
        + 0.0008 * deg2rad(204.0 + 7906.7 * t).sin()
        + 0.0007 * deg2rad(140.0 + 4052.0 * t).sin()
        + 0.0007 * deg2rad(275.0 + 4853.3 * t).sin()
        + 0.0007 * deg2rad(216.0 + 278.6 * t).sin()
        + 0.0006 * deg2rad(128.0 + 1118.7 * t).sin()
        + 0.0005 * deg2rad(247.0 + 22582.7 * t).sin()
        + 0.0005 * deg2rad(181.0 + 19088.0 * t).sin()
        + 0.0005 * deg2rad(114.0 + 17450.7 * t).sin()
        + 0.0005 * deg2rad(332.0 + 5091.3 * t).sin()
        + 0.0004 * deg2rad(313.0 + 398.7 * t).sin()
        + 0.0004 * deg2rad(278.0 + 120.1 * t).sin()
        + 0.0004 * deg2rad(71.0 + 9584.7 * t).sin()
        + 0.0004 * deg2rad(20.0 + 720.0 * t).sin()
        + 0.0003 * deg2rad(83.0 + 3814.0 * t).sin()
        + 0.0003 * deg2rad(66.0 + 3494.7 * t).sin()
        + 0.0003 * deg2rad(147.0 + 18089.3 * t).sin()
        + 0.0003 * deg2rad(311.0 + 5492.0 * t).sin()
        + 0.0003 * deg2rad(161.0 + 40.7 * t).sin()
        + 0.0003 * deg2rad(280.0 + 23221.3 * t).sin();

    adjust0to360(lm)
}

/**
 * 度数法から弧度法への変換
 */
fn deg2rad(deg: f64) -> f64 {
    deg * PI as f64 / 180.0
}

/**
 * 0 <= x <= 360 に修正する
 */
fn adjust0to360(deg: f64) -> f64 {
    let tmp = deg % 360.0;

    if tmp < 0.0 {
        tmp + 360.0
    } else {
        tmp
    }
}
