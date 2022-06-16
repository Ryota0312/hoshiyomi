use std::f32::consts::PI;
use chrono::{Utc, DateTime, TimeZone, Datelike, Timelike, Date};

const ZONE_OFFSET: f64 = 9.0;
const R: f64 = 0.585556;

/**
 * 座標
 */
struct Geocode {
    longitude: f64,
    latitude: f64,
}

/**
 * 黄道座標
 */
struct Ecliptic {
    longitude: f64,
    latitude: f64,
}

/**
 * 赤道座標
 */
struct Equatorial {
    longitude: f64,
    latitude: f64,
}

fn main() {
    let dt: Result<DateTime<Utc>, _> = Utc.datetime_from_str("1999/11/14 00:00:00", "%Y/%m/%d %H:%M:%S");
    let d = dt.unwrap().date();
    println!("Local.datetime_from_str: {:?}", d);
    let geocode = Geocode { longitude: 139.7447, latitude: 35.6544 };

    let d = get_moon_rise(d, geocode);
    let moon_rise = Utc.timestamp(dt.unwrap().timestamp() + (60.0 * 60.0 * 24.0 * d) as i64, 0);
    println!("Result: {:?}", moon_rise);
}

fn get_moon_rise(date: Date<Utc>, geocode: Geocode) -> f64 {
    const THRESHOLD_DELTA_D: f64 = 0.000005;

    let mut delta_d = 0.0;
    let mut d = 0.5;

    loop {
        d += delta_d;
        let datetime_hms0 = date.and_hms(0, 0, 0);
        let tmp_datetime = Utc.timestamp(datetime_hms0.timestamp() + (60.0 * 60.0 * 24.0 * d) as i64, 0);

        let moon_parallax = get_moon_parallax(tmp_datetime);

        let moon_equatorial = ecliptic2equatorial(get_moon_ecliptic(tmp_datetime), ecliptic_tilt_angle(datetime_hms0));
        println!("{}", moon_equatorial.longitude);
        println!("{}", moon_equatorial.latitude);
        let k = -R + moon_parallax;
        println!("k: {}", k);
        let cos_tk = (deg2rad(k).sin() - deg2rad(moon_equatorial.latitude).sin() * deg2rad(geocode.latitude).sin())
            / (deg2rad(moon_equatorial.latitude).cos() * deg2rad(geocode.latitude).cos());
        println!("cos_tk: {}", cos_tk);
        let tk = -rad2deg(cos_tk.acos());
        println!("tk: {}", tk);
        let t = get_sidereal_time(d) - moon_equatorial.longitude;
        println!("t: {}", t);
        delta_d = (tk - t) / 347.8;
        println!("delta_d: {}", delta_d);
        if delta_d.abs() < THRESHOLD_DELTA_D {
            break;
        }
    }

    d
}

/**
 * 恒星時
 */
fn get_sidereal_time(d: f64) -> f64 {
    57.027999 + 360.985647 * d
}

/**
 * year年month月day日0時のJ2000.0(2000年１月１日力学時正午)からの経過日数
 */
fn j2000day(datetime: DateTime<Utc>) -> f64 {
    let year = datetime.year();
    let month = datetime.month();
    let day = datetime.day();
    let hour = datetime.hour();
    let min = datetime.minute();
    let sec = datetime.second();

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
fn j2000year(datetime: DateTime<Utc>) -> f64 {
    j2000day(datetime) / 365.25
}

/**
 * 月の黄道座標
 */
fn get_moon_ecliptic(datetime: DateTime<Utc>) -> Ecliptic {
    Ecliptic { longitude: get_moon_longitude(datetime), latitude: get_moon_latitude(datetime) }
}

/**
 * 月の黄経の近似計算
 */
fn get_moon_longitude(datetime: DateTime<Utc>) -> f64 {
    let t = j2000year(datetime);
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
 * 月の黄緯の近似計算
 */
fn get_moon_latitude(datetime: DateTime<Utc>) -> f64 {
    let t = j2000year(datetime);
    let bm = 0.0267 * deg2rad(234.95 + 19.341 * t).sin()
        + 0.0043 * deg2rad(322.1 + 19.36 * t).sin()
        + 0.0040 * deg2rad(119.5 + 1.33 * t).sin()
        + 0.0020 * deg2rad(55.0 + 19.34 * t).sin()
        + 0.0005 * deg2rad(307.0 + 19.4 * t).sin();

    let betam = 5.1282 * deg2rad(93.273 + 4832.0202 * t + bm).sin()
        + 0.2806 * deg2rad(228.235 + 9604.0088 * t).sin()
        + 0.2777 * deg2rad(138.311 + 60.0316 * t).sin()
        + 0.1732 * deg2rad(142.427 + 4073.3220 * t).sin()
        + 0.0554 * deg2rad(194.01 + 8965.374 * t).sin()
        + 0.0463 * deg2rad(172.55 + 698.667 * t).sin()
        + 0.0326 * deg2rad(328.96 + 13737.362 * t).sin()
        + 0.0172 * deg2rad(3.18 + 14375.997 * t).sin()
        + 0.0093 * deg2rad(277.4 + 8845.31 * t).sin()
        + 0.0088 * deg2rad(176.7 + 4711.96 * t).sin()
        + 0.0082 * deg2rad(144.9 + 3713.33 * t).sin()
        + 0.0043 * deg2rad(307.6 + 5470.66 * t).sin()
        + 0.0042 * deg2rad(103.9 + 18509.35 * t).sin()
        + 0.0034 * deg2rad(319.9 + 4433.31 * t).sin()
        + 0.0025 * deg2rad(196.5 + 8605.38 * t).sin()
        + 0.0022 * deg2rad(331.4 + 13377.37 * t).sin()
        + 0.0021 * deg2rad(170.1 + 1058.66 * t).sin()
        + 0.0019 * deg2rad(230.7 + 9244.02 * t).sin()
        + 0.0018 * deg2rad(243.3 + 8206.68 * t).sin()
        + 0.0018 * deg2rad(270.8 + 5192.01 * t).sin()
        + 0.0017 * deg2rad(99.8 + 14496.06 * t).sin()
        + 0.0016 * deg2rad(135.7 + 420.02 * t).sin()
        + 0.0015 * deg2rad(211.1 + 9284.69 * t).sin()
        + 0.0015 * deg2rad(45.8 + 9964.00 * t).sin()
        + 0.0014 * deg2rad(219.2 + 299.96 * t).sin()
        + 0.0013 * deg2rad(95.8 + 4472.03 * t).sin()
        + 0.0013 * deg2rad(155.4 + 379.35 * t).sin()
        + 0.0012 * deg2rad(38.4 + 4812.68 * t).sin()
        + 0.0012 * deg2rad(148.2 + 4851.36 * t).sin()
        + 0.0011 * deg2rad(138.3 + 19147.99 * t).sin()
        + 0.0010 * deg2rad(18.0 + 12978.66 * t).sin()
        + 0.0008 * deg2rad(70.0 + 17870.7 * t).sin()
        + 0.0008 * deg2rad(326.0 + 9724.1 * t).sin()
        + 0.0007 * deg2rad(294.0 + 13098.7 * t).sin()
        + 0.0006 * deg2rad(224.0 + 5590.7 * t).sin()
        + 0.0006 * deg2rad(52.0 + 13617.3 * t).sin()
        + 0.0005 * deg2rad(280.0 + 8485.3 * t).sin()
        + 0.0005 * deg2rad(239.0 + 4193.4 * t).sin()
        + 0.0004 * deg2rad(311.0 + 9483.9 * t).sin()
        + 0.0004 * deg2rad(238.0 + 23281.3 * t).sin()
        + 0.0004 * deg2rad(81.0 + 10242.6 * t).sin()
        + 0.0004 * deg2rad(13.0 + 9325.4 * t).sin()
        + 0.0004 * deg2rad(147.0 + 14097.4 * t).sin()
        + 0.0003 * deg2rad(205.0 + 22642.7 * t).sin()
        + 0.0003 * deg2rad(107.0 + 18149.4 * t).sin()
        + 0.0003 * deg2rad(146.0 + 3353.3 * t).sin()
        + 0.0003 * deg2rad(234.0 + 19268.0 * t).sin();

    adjust0to360(betam)
}

/**
 * 月の視差を近似計算
 */
fn get_moon_parallax(datetime: DateTime<Utc>) -> f64 {
    let t = j2000year(datetime);

    let p = 0.9507 * deg2rad(90.0).sin()
        + 0.0518 * deg2rad(224.98 + 4771.989 * t).sin()
        + 0.0095 * deg2rad(190.7 + 4133.35 * t).sin()
        + 0.0078 * deg2rad(325.7 + 8905.34 * t).sin()
        + 0.0028 * deg2rad(0.0 + 9543.98 * t).sin()
        + 0.0009 * deg2rad(100.0 + 13677.3 + t).sin()
        + 0.0005 * deg2rad(329.0 + 8545.4 * t).sin()
        + 0.0004 * deg2rad(194.0 + 3773.4 * t).sin()
        + 0.0003 * deg2rad(227.0 + 4412.0 * t).sin();

    adjust0to360(p)
}

/**
 * 黄道座標を赤道座標に変換
 */
fn ecliptic2equatorial(ecliptic: Ecliptic, e: f64) -> Equatorial {
    let u = deg2rad(ecliptic.latitude).cos() * deg2rad(ecliptic.longitude).cos();
    let v = -deg2rad(ecliptic.latitude).sin() * deg2rad(e).sin() + deg2rad(ecliptic.latitude).cos() * deg2rad(ecliptic.longitude).sin() * deg2rad(e).cos();
    let w = deg2rad(ecliptic.latitude).sin() * deg2rad(e).cos() + deg2rad(ecliptic.latitude).cos() * deg2rad(ecliptic.longitude).sin() * deg2rad(e).sin();

    let a = adjust0to360(rad2deg((v / u).atan()));
    let d = rad2deg((w / (u.powi(2) + v.powi(2)).sqrt()).atan());
    Equatorial { longitude: a, latitude: d }
}

/**
 * 黄道傾角
 */
fn ecliptic_tilt_angle(datetime: DateTime<Utc>) -> f64 {
    let t = j2000year(datetime);

    adjust0to360(23.439291 - 0.000130042 * t)
}

/**
 * 度数法から弧度法への変換
 */
fn deg2rad(deg: f64) -> f64 {
    deg * PI as f64 / 180.0
}

/**
 * 弧度法から度数法への変換
 */
fn rad2deg(rad: f64) -> f64 {
    rad * 180.0 / PI as f64
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
