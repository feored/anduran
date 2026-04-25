use crate::Error;
use crate::SaveString;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<SaveString>, Error> {
    reader.read_vec_u32("custom rumors count", |reader| {
        reader.read_save_string("custom rumor")
    })
}

pub(super) fn encode(
    writer: &mut Writer,
    custom_rumors: &[SaveString],
) -> std::result::Result<(), Error> {
    writer.write_len_u32(custom_rumors.len(), "world custom rumors")?;
    for rumor in custom_rumors {
        writer.write_save_string(rumor, "custom rumor")?;
    }

    Ok(())
}
