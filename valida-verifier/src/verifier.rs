use anyhow::Result;
// use flate2::read::GzDecoder;
use std::env;
// use std::io::Read;
use std::{env::consts::ARCH, fs, path::Path, process::Command};
use tempfile::tempdir;

use parity_scale_codec::{Decode, Encode};
use std::borrow::Cow;
use std::io;
use std::path::PathBuf;

#[derive(Encode, Decode, Debug, Clone)]
pub enum VerifyError {
    TooManyFilesInArchive,
    WrongName,
    DirectoryWasSent,
}

fn decompress(fname: &Path) -> Result<String, VerifyError> {
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    if archive.len() > 1 {
        return Err(VerifyError::TooManyFilesInArchive);
    }

    let mut file = archive.by_index(0).unwrap();
    let outpath = match file.enclosed_name() {
        Some(path) => path,
        None => return Err(VerifyError::WrongName),
    };

    if file.is_dir() {
        return Err(VerifyError::DirectoryWasSent);
    } else {
        println!(
            "File extracted to \"{}\" ({} bytes)",
            outpath.display(),
            file.size()
        );

        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p).unwrap();
            }
        }
        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();
    }

    // Get and Set permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if let Some(mode) = file.unix_mode() {
            fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
        }
    }

    let name = outpath.to_string_lossy().into_owned();
    Ok(name)
}

// Write program and proof to local zip files named by hash of either, and return written filenames
fn write_proof_files_local(
    compressed_proof: &[u8],
    compressed_program: &[u8],
) -> (PathBuf, PathBuf) {
    use sp_core::hashing::keccak_256;

    let program_zip_hash = keccak_256(compressed_program);
    let proof_zip_hash = keccak_256(compressed_proof);

    let local_program_zip_name = format!("{}.zip", hex::encode(program_zip_hash));
    let local_proof_zip_name = format!("{}.zip", hex::encode(proof_zip_hash));

    println!("Writing files to {:?}", std::env::current_dir());

    let output_dir = Path::new("/data/artifacts");
    std::fs::create_dir_all(&output_dir).unwrap();
    let local_program_zip_name = output_dir.join(format!("{}.zip", hex::encode(program_zip_hash)));
    let local_proof_zip_name = output_dir.join(format!("{}.zip", hex::encode(proof_zip_hash)));

    std::fs::write(&local_program_zip_name, &compressed_program).unwrap();
    std::fs::write(&local_proof_zip_name, &compressed_proof).unwrap();

    (
        PathBuf::from(local_proof_zip_name),
        PathBuf::from(local_program_zip_name),
    )
}

// Verify a proof generated with the 0.5.0 valida toolkit. FYI: for now, Valida toolkit heavily relies on docker, which we wrap here.
// It's expected that over time, there would be a well-known rust verifier that could be imported here
pub fn verify(compressed_proof_bytes: &[u8], compressed_program: &[u8]) -> Result<bool> {
    // Write compressed files to local and get their paths. Valida can work based off of files so we need these
    // let (local_proof_zip_name, local_program_zip_name) =
    //     write_proof_files_local(compressed_proof_bytes, compressed_program);

    // Decompress files to local TODO: convert this error and remove unwrap
    // let local_proof_filename = decompress(&local_proof_zip_name).unwrap();
    // let local_program_file_name = decompress(&local_program_zip_name).unwrap();

    let work_dir = env::current_dir().unwrap();
    println!("Current {:?}", work_dir);

    let host_container_path =
        env::var("HOST_CONTAINER_PATH").expect("HOST_CONTAINER_PATH environment variable not set");

    let mut command = Command::new("docker");
    command
        .arg("run")
        .arg("--rm")
        .arg("--platform")
        .arg("linux/amd64")
        .arg("-v")
        .arg(format!("{}:/src", host_container_path))
        .arg("-v")
        .arg("/data:/data")
        .arg("--entrypoint=valida")
        .arg("ghcr.io/lita-xyz/llvm-valida-releases/valida-build-container:v0.5.0-alpha")
        .arg("verify")
        // .arg(format!("./{}", local_program_file_name))
        // .arg(format!("./{}", local_proof_filename));
        .arg("./fibonacci")
        .arg("./proof.lita");

    let output = command.output().unwrap();

    // Capture and print stdout
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Capture and print stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        println!("Docker stderr: {}", stderr);
    }

    Ok(false)
}

fn is_docker_installed() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_x86_architecture() -> bool {
    ARCH == "x86_64" || ARCH == "x86"
}
