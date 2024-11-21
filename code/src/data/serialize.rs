use std::fmt;

use crate::data::data::Data;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Visitor, Deserialize, Deserializer, SeqAccess, MapAccess};

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Data", 4)?;
        state.serialize_field("sets", &self.sets)?;
        state.serialize_field("relations", &self.relations)?;
        state.serialize_field("sources", &self.sources)?;
        state.serialize_field("providers", &self.providers)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Sets, Relations, Sources, Providers }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`sets`, `relations`, `sources`, or `providers`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "sets" => Ok(Field::Sets),
                            "relations" => Ok(Field::Relations),
                            "sources" => Ok(Field::Sources),
                            "providers" => Ok(Field::Providers),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DataVisitor;

        impl<'de> Visitor<'de> for DataVisitor {
            type Value = Data;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Data")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Data, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let relations = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let sources = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let relations = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Data::new(secs, relations))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Data, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut sets = None;
                let mut relations = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Sets => {
                            if sets.is_some() {
                                return Err(de::Error::duplicate_field("sets"));
                            }
                            sets = Some(map.next_value()?);
                        }
                        Field::Relations => {
                            if relations.is_some() {
                                return Err(de::Error::duplicate_field("relations"));
                            }
                            relations = Some(map.next_value()?);
                        }
                    }
                }
                let sets = sets.ok_or_else(|| de::Error::missing_field("sets"))?;
                let relations = relations.ok_or_else(|| de::Error::missing_field("relations"))?;
                Ok(Data::new(sets, relations))
            }
        }

        const FIELDS: &[&str] = &["sets", "relations", "sources", "providers"];
        deserializer.deserialize_struct("Data", FIELDS, DataVisitor)
    }
}
