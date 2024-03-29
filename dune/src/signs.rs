use ansi_term::Color::{Green, Purple, Red};
use anyhow::Result;
use bumpalo::Bump;
use dune_lib::world::{
    anvil::{Region, CHUNKS_PER_REGION},
    chunk::{read_chunk, BlockEntityKind, Chunk},
};
use std::{
    ffi::OsStr,
    fs::{self, DirEntry, File},
    io::BufWriter,
    path::Path,
    time::Instant,
};
use std::{io::Write, path::PathBuf};

struct SignsPrinter {
    out: BufWriter<File>,
    max: usize,
    signs_count: u32,
    total_signs_count: u32,
    errors_count: u32,
}

fn do_print(context: &mut SignsPrinter, chunk: Chunk) -> Result<()> {
    const DASHES80: &str =
        "--------------------------------------------------------------------------------";

    for i in chunk.block_entities {
        match i.kind {
            BlockEntityKind::Sign(sign) => {
                if sign.text.iter().all(String::is_empty) {
                    continue;
                }
                context.max = sign
                    .text
                    .iter()
                    .map(String::len)
                    .max()
                    .unwrap_or(context.max);

                writeln!(
                    context.out,
                    "/tp {} {} {}\n{:^80}\n{:^80}\n{:^80}\n{:^80}\n{}\n",
                    i.position.x,
                    i.position.y,
                    i.position.z,
                    sign.text[0],
                    sign.text[1],
                    sign.text[2],
                    sign.text[3],
                    DASHES80
                )?;
                context.signs_count += 1;
                context.total_signs_count += 1;
            }
            BlockEntityKind::Storage(chest) => {
                if chest.items.is_empty() {
                    continue;
                }
                println!("{:?}", chest.items);
            }
            _ => {}
        }
    }
    Ok(())
}

fn print_chunk(
    context: &mut SignsPrinter,
    region: &mut Region,
    tmp: &mut Vec<u8>,
    bump: &Bump,
    index: usize,
) -> Result<()> {
    let data = region.get_chunk(tmp, index)?;
    if data.is_empty() {
        return Ok(());
    }

    let chunk = read_chunk(data, bump)?;
    do_print(context, chunk)
}

fn print_region(
    context: &mut SignsPrinter,
    tmp: &mut Vec<u8>,
    bump: &Bump,
    path: &Path,
) -> Result<()> {
    let mut region = Region::load(path, false)?;
    context.signs_count = 0;
    for i in 0..CHUNKS_PER_REGION {
        if let Err(e) = print_chunk(context, &mut region, tmp, bump, i) {
            context.errors_count += 1;
            eprintln!("error in file {}: {}", path.display(), e);
            break;
        }
    }

    Ok(())
}

fn parse_file_path((path, _): &(PathBuf, u64)) -> (i32, i32) {
    let mut it = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .split('.');
    it.next();
    let x = it.next().unwrap_or_default().parse().unwrap_or(i32::MIN);
    let z = it.next().unwrap_or_default().parse().unwrap_or(i32::MIN);
    (x, z)
}
fn get_paths(path: String) -> Result<Vec<(PathBuf, u64)>> {
    let mut files = Vec::new();
    let get = |i| -> Result<_> {
        let i: DirEntry = i?;
        let i = i.path();
        let size = i.metadata()?.len();
        Ok((i, size))
    };
    let mut zero_size_files = 0;
    for i in fs::read_dir(path)? {
        let i = match get(i) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("can't read path: {}", e);
                continue;
            }
        };
        if i.1 == 0 {
            zero_size_files += 1;
            continue;
        }

        files.push(i);
    }
    files.sort_by_cached_key(parse_file_path);

    if zero_size_files > 0 {
        println!("ignoring {} zero size files", zero_size_files);
    }

    Ok(files)
}

pub fn print(path: String) -> Result<()> {
    let mut tmp = Vec::new();
    let bump = &mut Bump::with_capacity(128 * 1024); // 128kb based on experimentation
    let mut context = SignsPrinter {
        out: BufWriter::new(File::create("out.txt")?),
        max: 0,
        signs_count: 0,
        total_signs_count: 0,
        errors_count: 0,
    };

    let files = get_paths(path)?;
    let files_count = files.len();
    for (index, (path, file_size)) in files.into_iter().enumerate() {
        let time = Instant::now();
        if path.extension() != Some(OsStr::new("mca")) {
            continue;
        }

        if let Err(e) = print_region(&mut context, &mut tmp, bump, &path) {
            context.errors_count += 1;
            eprintln!("can't read path: {}", e);
            continue;
        }

        println!(
            "{:>4}/{} --- {:<20} --- {:>4} signs --- {:>10} --- {:?}",
            index + 1,
            files_count,
            path.file_name()
                .unwrap_or(path.as_os_str())
                .to_string_lossy(),
            context.signs_count,
            humansize::format_size(file_size, humansize::BINARY),
            time.elapsed(),
        );

        bump.reset();
    }
    println!(
        "output written to `out.txt`\n{}={}\n{}={}\n{}={}",
        Purple.paint("max sign length"),
        context.max,
        Red.paint("errors count"),
        context.errors_count,
        Green.paint("total signs count"),
        context.total_signs_count,
    );

    Ok(())
}
