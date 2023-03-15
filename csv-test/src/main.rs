use anyhow::Result;
use polars::prelude::*;
use serde_json;
use std::env;

#[macro_use]
extern crate fstrings;

const DATAFILE: &str = "data_large.csv";

const CURRENCY_COL: &str = "[CURRENCY]";
const CURRENCY_TOKEN: &str = "{currency}";

enum MapMonthNameToNumber {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

fn roundTo1dpString(number: f64) -> String {
    format!("{:.1}", number)
}

fn load_csv(file_path: &str) -> Result<DataFrame> {
    println_f!("Loading CSV file: {file_path}");
    let df = CsvReader::from_path(file_path)?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;
    Ok(df)
}

fn apply_currency_conversion_and_col_filtering(
    df: &DataFrame,
    inflation_factor: f64,
    exchange_rate: f64,
    currency_code: &str,
    columns_to_remove: Vec<&str>,
    start_year_for_granular_data: u32,
    start_month_for_granular_data: u8,
) {
    // df.filter(df.column("Time (UTC)").dt().year()? > );
    // .drop_in_place(columns_to_remove)
    // .unwrap();

    // df.filter(df.column("Time (UTC)").dt().year()? > start_year_for_granular_data)
    //     .drop_in_place(columns_to_remove)
    //     .unwrap();

    df.drop_many(&columns_to_remove);
}

fn main() {
    // type CustomTypeAlias = i32;
    // let random_number: CustomTypeAlias = 10;

    // let value_to_get = 1;
    // // Output is option<&str> - Some value or None
    // let value_option_gotten = dict.get(&value_to_get);

    // println!("{:?}", value_option_gotten);
    // // Safe handling
    // if let Some(value_gotten) = value_option_gotten {
    //     println!("{:?}", value_gotten);
    // }

    // let number = "-8.38E-05";
    // if let Ok(number) = number.parse::<f64>() {
    //     println!("Number is {}", number);
    // } else {
    //     println!("Error parsing number");
    // }

    let mut df = load_csv(format!("data/{DATAFILE}").as_str()).unwrap();

    println!("{:#?}", df);

    const INFLATION_FACTOR: f64 = 1.0;
    const EXCHANGE_RATE: f64 = 1.0;
    const CURRENCY_CODE: &str = "GBP";
    const COLUMNS_TO_REMOVE: Vec<&str> = vec![];
    const START_YEAR_FOR_GRANULAR_DATA: u32 = 2020;
    const START_MONTH_FOR_GRANULAR_DATA: u8 = 0;

    use std::time::Instant;
    let now = Instant::now();
    apply_currency_conversion_and_col_filtering(
        &df,
        INFLATION_FACTOR,
        EXCHANGE_RATE,
        CURRENCY_CODE,
        COLUMNS_TO_REMOVE,
        START_YEAR_FOR_GRANULAR_DATA,
        START_MONTH_FOR_GRANULAR_DATA,
    );

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let mut file = std::fs::File::create(format!("out/{DATAFILE}")).unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
