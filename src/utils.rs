fn test() {
    let offset_str = "2023-09-17T14:30:00Z";  // ISO 8601 para OffsetDateTime
    let primitive_str = "2023-09-17 14:30:00"; // Formato comum para PrimitiveDateTime

    match to_offset_date_time(offset_str) {
        Ok(datetime) => println!("OffsetDateTime: {}", datetime),
        Err(e) => println!("Erro ao converter para OffsetDateTime: {}", e),
    }

    match to_primitive_date_time(primitive_str) {
        Ok(datetime) => println!("PrimitiveDateTime: {}", datetime),
        Err(e) => println!("Erro ao converter para PrimitiveDateTime: {}", e),
    }
}

use log::info;
use time::{OffsetDateTime, format_description::well_known::Iso8601};

pub fn to_offset_date_time(date_str: &str) -> Result<OffsetDateTime, time::error::Parse> {
    //exemplo 2023-09-17 14:30:00 +00:00
    info!("{:?}", &Iso8601::DEFAULT);
    let offset_datetime = OffsetDateTime::parse(date_str, &Iso8601::DEFAULT)?;
    info!("{}", offset_datetime);
    Ok(offset_datetime)
}

//format_description::well_known::Rfc3339,  [year]-[month]-[day]T[hour]:[minute]:[second]+00
use time::{PrimitiveDateTime, macros::format_description};

pub fn to_primitive_date_time(date_str: &str) -> Result<PrimitiveDateTime, time::error::Parse> {
    // Formato de exemplo: "2023-09-17 14:30:00"
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    let primitive_datetime = PrimitiveDateTime::parse(date_str, &format)?;
    Ok(primitive_datetime)
}

