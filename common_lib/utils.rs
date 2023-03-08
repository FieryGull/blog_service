#[cfg(test)]
pub mod test_utils {
    use chrono::{DateTime, Utc as GlobalUtc};
    use lazy_static::lazy_static;

    lazy_static!(
    static ref TIME_NOW: DateTime<GlobalUtc> = GlobalUtc::now();
);

    pub struct Utc;

    impl Utc {
        pub fn now() -> DateTime<GlobalUtc>{
            *TIME_NOW
        }
    }
}
