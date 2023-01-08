use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontSizes {
    pub small: f64,
    pub medium: f64,
    pub large: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct InvoiceStructure {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Partner {
    pub partner_name: String,
    pub partner_address: String,
    pub partner_postal_code: String,
    pub partner_vat_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub service_name: String,
    pub service_quantity: i32,
    pub service_price: f64,
    pub service_tax: f64,
    pub service_payment: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub company_currency: String,
    pub company_name: String,
    pub company_address: String,
    pub company_postal_code: String,
    pub company_vat_id: String,
    pub company_iban: String,
    pub company_swift: String,
    pub company_registration_number: String,
    pub company_phone: String,
    pub company_signature: String, //Base64 string
    pub company_vat_rate: f64,
}
impl Company {}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Racun {
    pub small_font: f64,
    pub medium_font: f64,
    pub large_font: f64,
    pub invoice_number: i32,
    pub invoice_date: String,
    pub service_date: String,
    pub due_date: String,
    pub partner: Partner,
    pub company: Company,
    pub services: Vec<Service>,
}

impl Partner {}

impl Racun {
    pub fn parse_from_file() -> Self {
        let data = read_to_string("data.json").expect("Cannot read file");
        println!("{}", data);
        let parsed: Self = serde_json::from_str(&data).expect("JSON does not have correct format.");
        println!("{:#?}", parsed);
        parsed
    }
}
//