use crate::Error;
use crate::SaveString;
use crate::internal::reader::Reader;
use crate::internal::writer::Writer;

pub(super) fn decode(reader: &mut Reader<'_>) -> std::result::Result<Vec<SaveString>, Error> {
    let count = reader.read_u32_be("custom rumors count")?;
    let mut rumors = Vec::with_capacity(usize::try_from(count).unwrap_or(0));
    for _ in 0..count {
        rumors.push(reader.read_save_string("custom rumor")?);
    }

    Ok(rumors)
}

pub(super) fn encode(
    writer: &mut Writer,
    custom_rumors: &[SaveString],
) -> std::result::Result<(), Error> {
    writer.write_u32_be(
        u32::try_from(custom_rumors.len()).map_err(|_| Error::InvalidModel {
            field: "world custom rumors",
            message: "custom rumor count must fit in u32",
        })?,
    );
    for rumor in custom_rumors {
        writer.write_save_string(rumor);
    }

    Ok(())
}
