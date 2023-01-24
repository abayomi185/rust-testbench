use anyhow::Result;
use polars::prelude::*;
use serde_json;

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
    let df = CsvReader::from_path(file_path)?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;
    Ok(df)
}

fn applyCurrencyConversionAndColFiltering() {}

fn main() {
    // type CustomTypeAlias = i32;
    // let random_number: CustomTypeAlias = 10;

    // let mut dict = HashMap::new();
    // dict.insert(1, "one");

    // println!("{:#?}", dict);

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

    let mut df = load_csv(format!("data/{DATAFILE}")).unwrap();

    let mut file = std::fs::File::create("out/{DATAFILE}").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
}
