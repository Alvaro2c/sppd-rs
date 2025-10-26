pub enum ProcurementType {
  MinorContracts,
  PublicTenders,
}

impl From<&str> for ProcurementType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "mc" | "minor-contracts" | "min" => Self::MinorContracts,
            "pt" | "pub" | "public-tenders" => Self::PublicTenders,
            _ =>{
                eprintln!(
                  "Unknown procurement type '{}', defaulting to 'public-tenders'",
                value
              );
            Self::PublicTenders
            }
        }
    }
}
