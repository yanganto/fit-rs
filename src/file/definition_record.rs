use super::base_type::BaseType;
use super::data_field::DataField;
use crate::messages::{new_record, DefinedMessageType};
use crate::reader::{Endian, Reader};

const FIELD_DEFINITION_ARCHITECTURE: u8 = 0b10_000_000;
const FIELD_DEFINITION_BASE_NUMBER: u8 = 0b00_011_111;

#[derive(Debug)]
pub struct DefinitionRecord {
    pub architecture: Endian,
    pub global_message_num: u16,
    pub number_of_fields: u8,
    pub field_defs: Vec<FieldDefinition>,
    dev_field_defs: Vec<u8>,
}
impl DefinitionRecord {
    pub fn new(reader: &mut Reader, dev_fields: bool) -> Self {
        if dev_fields {
            panic!("file has developer fields!")
        }
        reader.skip(1); // skip reserved byte
        let endian = match reader.byte() {
            Ok(1) => Endian::Big,
            Ok(0) => Endian::Little,
            _ => panic!("Error when trying to read endianness: value was not 1 or 0"),
        };
        let global_message_num = reader.u16(&endian).unwrap();
        let number_of_fields = reader.byte().unwrap();
        let field_defs: Vec<_> = (0..number_of_fields)
            .map(|_| {
                let buf = reader.bytes(3).unwrap();
                FieldDefinition::new(&buf)
            })
            .collect();
        DefinitionRecord {
            architecture: endian,
            global_message_num,
            number_of_fields,
            field_defs,
            dev_field_defs: Vec::new(),
        }
    }
    pub fn read_data_record(&self, reader: &mut Reader) -> Option<Box<dyn DefinedMessageType>> {
        let raw_fields: Vec<_> = self
            .field_defs
            .iter()
            .map(|fd| DataField::new(reader, &self.architecture, &fd))
            .collect();
        new_record(self.global_message_num).and_then(|mut r| {
            raw_fields.into_iter().for_each(|df| {
                if let Some(vals) = df.values {
                    let val = &vals[0];
                    r.process_raw_value(df.id, &val);
                }
            });
            Some(r)
        })
    }
}

#[derive(Debug)]
pub struct FieldDefinition {
    pub field_def_number: u16,
    pub size: u8,
    endianness: bool,
    pub base_type: BaseType,
}
impl FieldDefinition {
    fn new(buf: &[u8]) -> Self {
        let base_num = buf[2] & FIELD_DEFINITION_BASE_NUMBER;
        let endianness = (buf[2] & FIELD_DEFINITION_ARCHITECTURE) == FIELD_DEFINITION_ARCHITECTURE;
        Self {
            field_def_number: buf[0].into(),
            size: buf[1],
            endianness,
            base_type: BaseType::get(base_num),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefinitionRecord;
    use crate::tests::fit_setup;

    #[test]
    fn it_reads_a_definition() {
        let mut reader = fit_setup();
        reader.skip(14); // FileHeader
        reader.skip(1); // HeaderByte
        let definition = DefinitionRecord::new(&mut reader, false);
        // now 41
        assert_eq!(definition.number_of_fields, 7);
    }
}
