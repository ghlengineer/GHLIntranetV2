use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Interpretation<'a> {
    pub id: i32,
    pub date: &'a str,
    pub code_edition: &'a str,
    pub subject: &'a str,
    pub keywords: &'a str,
    pub reference: &'a str,
    pub question: &'a str,
    pub interpretation: &'a str,
}