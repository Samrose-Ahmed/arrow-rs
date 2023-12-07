use crate::format::SizeStatistics as TSizeStatistics;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SizeStatistics {
    pub unencoded_byte_array_data_bytes: Option<i64>,
    pub repetition_level_histogram: Option<Vec<i64>>,
    pub definition_level_histogram: Option<Vec<i64>>,
}

impl SizeStatistics {
    pub fn new() -> Self {
        Self {
            unencoded_byte_array_data_bytes: None,
            repetition_level_histogram: None,
            definition_level_histogram: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.unencoded_byte_array_data_bytes.is_none()
            && self.repetition_level_histogram.is_none()
            && self.definition_level_histogram.is_none()
    }
}

pub fn from_thrift(t: TSizeStatistics) -> SizeStatistics {
    SizeStatistics {
        unencoded_byte_array_data_bytes: t.unencoded_byte_array_data_bytes,
        repetition_level_histogram: t.repetition_level_histogram,
        definition_level_histogram: t.definition_level_histogram,
    }
}

pub fn to_thrift(t: SizeStatistics) -> TSizeStatistics {
    TSizeStatistics {
        unencoded_byte_array_data_bytes: t.unencoded_byte_array_data_bytes,
        repetition_level_histogram: t.repetition_level_histogram,
        definition_level_histogram: t.definition_level_histogram,
    }
}
