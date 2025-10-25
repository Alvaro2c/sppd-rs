pub enum ProcurementType {
  MinorContracts,
  PublicTenders,
}

impl From<&str> for ProcurementType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "mc" | "minor-contracts" | "min" => ProcurementType::MinorContracts,
            "pt" | "pub" | "public-tenders" => ProcurementType::PublicTenders,
            _ => ProcurementType::PublicTenders, // default fallback
        }
    }
}
