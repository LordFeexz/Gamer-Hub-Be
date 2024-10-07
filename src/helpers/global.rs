use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::types::time::PrimitiveDateTime;

pub fn convert_primitive_to_naive(pd: PrimitiveDateTime) -> NaiveDateTime {
    let date = NaiveDate::from_ymd_opt(
        pd.date().year(),
        pd.date().month() as u32,
        pd.date().day() as u32,
    )
    .unwrap();
    let time = NaiveTime::from_hms_opt(
        pd.time().hour() as u32,
        pd.time().minute() as u32,
        pd.time().second() as u32,
    )
    .unwrap();

    NaiveDateTime::new(date, time)
}
