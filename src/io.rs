use hftbacktest::types::Event;
use std::fs::File;
use std::io::Write;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub fn save_as_csv(events: &[Event], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "ev,exch_ts,local_ts,px,qty,order_id,ival,fval")?;
    for ev in events {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{}",
            ev.ev, ev.exch_ts, ev.local_ts, ev.px, ev.qty, ev.order_id, ev.ival, ev.fval
        )?;
    }
    Ok(())
}

pub fn save_as_npz(events: &[Event], filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    zip.start_file("data.npy", options)?;

    let header = format!(
        "{{'descr': [('ev', '<u8'), ('exch_ts', '<i8'), ('local_ts', '<i8'), ('px', '<f8'), ('qty', '<f8'), ('order_id', '<u8'), ('ival', '<i8'), ('fval', '<f8')], 'fortran_order': False, 'shape': ({},)}}",
        events.len()
    );

    let mut header_bytes = header.as_bytes().to_vec();
    let padding_len = 64 - ((10 + header_bytes.len() + 1) % 64);
    header_bytes.extend(std::iter::repeat(b' ').take(padding_len));
    header_bytes.push(b'\n');

    let header_len = header_bytes.len() as u16;

    zip.write_all(b"\x93NUMPY")?;
    zip.write_all(&[1, 0])?;
    zip.write_all(&header_len.to_le_bytes())?;
    zip.write_all(&header_bytes)?;

    for ev in events {
        zip.write_all(&ev.ev.to_le_bytes())?;
        zip.write_all(&ev.exch_ts.to_le_bytes())?;
        zip.write_all(&ev.local_ts.to_le_bytes())?;
        zip.write_all(&ev.px.to_le_bytes())?;
        zip.write_all(&ev.qty.to_le_bytes())?;
        zip.write_all(&ev.order_id.to_le_bytes())?;
        zip.write_all(&ev.ival.to_le_bytes())?;
        zip.write_all(&ev.fval.to_le_bytes())?;
    }

    zip.finish()?;
    Ok(())
}

pub fn save_as_bin(events: &[Event], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for ev in events {
        file.write_all(&ev.ev.to_le_bytes())?;
        file.write_all(&ev.exch_ts.to_le_bytes())?;
        file.write_all(&ev.local_ts.to_le_bytes())?;
        file.write_all(&ev.px.to_le_bytes())?;
        file.write_all(&ev.qty.to_le_bytes())?;
        file.write_all(&ev.order_id.to_le_bytes())?;
        file.write_all(&ev.ival.to_le_bytes())?;
        file.write_all(&ev.fval.to_le_bytes())?;
    }
    Ok(())
}
