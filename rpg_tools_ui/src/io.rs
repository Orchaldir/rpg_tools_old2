use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_each(
    path: impl AsRef<Path>,
    items: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    for i in items {
        file.write_all(i.as_ref())?;
        file.write_all("\n".as_ref())?;
    }
    file.sync_all()
}
