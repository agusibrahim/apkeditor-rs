//! APK Signature Scheme v2 Implementation
//! 
//! This module implements APK signing that works with byte arrays instead of file paths,
//! enabling signing in WASM environments.

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rsa::pkcs8::EncodePublicKey;
use sha2::{Digest, Sha256};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use xcommon::Signer;

/// Debug keystore PEM (same as used in the apk crate)
const DEBUG_PEM: &str = r#"-----BEGIN CERTIFICATE-----
MIIDeTCCAmGgAwIBAgIUCymsKTowQdR5TEv+vKSVjAWmYBowDQYJKoZIhvcNAQEL
BQAwTDELMAkGA1UEBhMCVVMxEzARBgNVBAgMClNvbWUtU3RhdGUxEDAOBgNVBAoM
B0FuZHJvaWQxFjAUBgNVBAMMDUFuZHJvaWQgRGVidWcwHhcNMjIwMTI4MTUyNjQ5
WhcNMzIwMTI2MTUyNjQ5WjBMMQswCQYDVQQGEwJVUzETMBEGA1UECAwKU29tZS1T
dGF0ZTEQMA4GA1UECgwHQW5kcm9pZDEWMBQGA1UEAwwNQW5kcm9pZCBEZWJ1ZzCC
ASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBANdFY1F564A3MzuCaTUGluti
pqLWr1o515BC8o42fIClqBWPcz3Hb4C56A6FLVq50gmFz+mMNGBqrgkT9RKICk+O
OV8hl0O/DzXM4COdfSdWZ1ZaNkFL1lboIAmfmTckWEymFj67gwqqpPy6dujteIn6
S28AbdHs2FAr1R+ciMoQ7ijxLSMq/JyYNSu/ldcvdzaevxiYMpcDZ6SMDTNn3eHs
D9w9iSkupVloUWx7ophdR0U2k2CFH3uEyDHC6L65K8aP+SQaN20IlmWftkwoRyum
cfzW/b9i77XnaT8PlrX1yjZ2ubeD7c/JyEVj2gd5B+OnkTmC+Mi0I+6Eke5vFVMC
AwEAAaNTMFEwHQYDVR0OBBYEFFVRccNTaUP2O9T8yrguVH4+CCSWMB8GA1UdIwQY
MBaAFFVRccNTaUP2O9T8yrguVH4+CCSWMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZI
hvcNAQELBQADggEBANbpPG3teQt/Z1ALsaIrsXOqpPKqVPCRp3w+hNzl/rleEpgm
zDIlyrLVDRzQyUFHhl9j1oJKPHzpE/1hy46rOZ509dqGqdfcDCTXjLi1O8JJ54wA
PdJ0h/8YPzh1md+GibZZYFimnFNoG9i6jQuEb4l5HIZLjJj02u+e4gpTD85LdOvw
S4jS/30KnuZVcr7TilrgOMMeP6GRzbBJ+/hXcfY2biSAu5pdEht2NV9SSKlIO3DD
ulXXz0+BJJ+PdVqTpPgHvbXbHktOD58srszwmLHHZJl5IfcBwJO0TNvad5lALBYI
kdxygt2CwyNOJUVd/nfQJ1O3YiwRkoVJ6on9Mnk=
-----END CERTIFICATE-----
-----BEGIN PRIVATE KEY-----
MIIEvwIBADANBgkqhkiG9w0BAQEFAASCBKkwggSlAgEAAoIBAQDXRWNReeuANzM7
gmk1BpbrYqai1q9aOdeQQvKONnyApagVj3M9x2+AuegOhS1audIJhc/pjDRgaq4J
E/USiApPjjlfIZdDvw81zOAjnX0nVmdWWjZBS9ZW6CAJn5k3JFhMphY+u4MKqqT8
unbo7XiJ+ktvAG3R7NhQK9UfnIjKEO4o8S0jKvycmDUrv5XXL3c2nr8YmDKXA2ek
jA0zZ93h7A/cPYkpLqVZaFFse6KYXUdFNpNghR97hMgxwui+uSvGj/kkGjdtCJZl
n7ZMKEcrpnH81v2/Yu+152k/D5a19co2drm3g+3PychFY9oHeQfjp5E5gvjItCPu
hJHubxVTAgMBAAECggEBAMAD45A0WOy30Bn/vAoRQ6LYDtzm8+hd+bpzDNnvHeS+
XoxEtT1g3EOND8GL5yWq4/+cfRTL+5gY7/2m8I3EDLZjnScO1lcWX+HUSgVan9zr
xCcRNp3NoHVKffE3i7nU0HImH2d7aGqmRZ4sUI5562/fc1OipVJ/mX8BagvVW2oo
RpThTUYC37T/X/kD0U/06pJzWmF3RAAhANk6+Z9VVX1kNsPEMBzoWTmhqb6dxiAc
Ayce8AslF8E0CmyMQ9HK7GwHCprENS7cIUMPG+vgrO5yFbGkIo4DrNTs2naA4f4S
iQvpNpGfRAfTdi4gV3YZoxfOOOhAh8A9RsAFrT8t6dECgYEA9hVWXHru1jlY1uiV
misILoSux+iE25HGqOdHuqF5vR5Ji1Z4iFE1UNAOtKaSbTDm0IccEBpTOkzL8A5f
BgRJRy+TjdE/ynzPgLLD/QnvGfdYarmr6H1xLKOlUY9vgUP2WAC4Zou9Jf/Ylbpg
BpfkXw0ebfhu1LGRXDj1sgqXAbsCgYEA3/Iuuq0YZy8msyc0Ap53mQgPjdqE2neo
xx7JHuXBGvVeCJ+zEzSg/rqWPNN4qpuHCc2ICb1nI5lkxJqimY30Em/Prpp9jMIK
wpeT/bPfOzITXyAOUIxRGqioTIv+ckyt+2t4x5qU+fWHBqWYTZb7EF3oJuipz9aZ
IoDwaKxd1UkCgYEArYNKC5daxI5XB+Gjarsg37wKiUZ4N2HIU9wQBZZKAoFSlf74
qhWopDyvwc0ZvggXF73MmcYWHSt9ONzJP7LSAHGZdwuuERaEMVjbPJY+k26GV2pn
vlyE6lbRAHtEwj6rek23uAab7ilCDAEIKF39VtAnPp9Hdo1l00MOauVwqHUCgYB9
FSsuj1ILCBYIiMQPFm3cptjxNXVxBNbbaQGS5WdHZHdCP9joyEOII7WYgdFrEXWK
byclsYmzI5FaErjxJY2G4rbQYm/vt84ExF8fnGD6Ek0pm6EDMmx2hG+EWckkFFo1
DOEoM9o0BwSFHOcFp2fRy3HIkbmPYeCkmfotrOC4KQKBgQC0OEniLk9PPhcaHO6/
Oo2xwWUq+TEN72jW5AV77xpykkAw3T4TeY5w84BZfCjOa4bYsvjvbjtn/DhtoDBj
TySd4PKKWF9XalNpbXmVQYtPU8huw1iwg+dV5llQG2pksFWDD2rglAEb2TEpwEvL
hmBjxp0mRtma4r/6hMJJzPdUmQ==
-----END PRIVATE KEY-----
"#;

const APK_SIGNING_BLOCK_MAGIC: &[u8] = b"APK Sig Block 42";
const APK_SIGNING_BLOCK_V2_ID: u32 = 0x7109871a;
const RSA_PKCS1V15_SHA2_256: u32 = 0x0103;
const MAX_CHUNK_SIZE: usize = 1024 * 1024;

/// Sign APK bytes using APK Signature Scheme v2
/// Returns the signed APK as a new Vec<u8>
pub fn sign_apk_bytes(apk_data: &[u8]) -> Result<Vec<u8>> {
    let signer = Signer::new(DEBUG_PEM)?;
    
    let mut r = Cursor::new(apk_data);
    let block = parse_apk_signing_block(&mut r)?;
    let zip_hash = compute_digest(&mut r, block.sb_start, block.cd_start, block.cde_start)?;
    
    // Create new signing block
    let mut nblock = vec![];
    let mut w = Cursor::new(&mut nblock);
    write_apk_signing_block(&mut w, zip_hash, &signer)?;
    
    // Assemble the signed APK
    let mut output = Vec::with_capacity(apk_data.len() + nblock.len());
    
    // Write content before signing block
    output.extend_from_slice(&apk_data[..(block.sb_start as usize)]);
    
    // Write new signing block
    output.extend_from_slice(&nblock);
    
    let new_cd_start = output.len() as u64;
    
    // Write central directory
    output.extend_from_slice(&apk_data[(block.cd_start as usize)..(block.cde_start as usize)]);
    
    let new_cde_start = output.len() as u64;
    
    // Write central directory end
    output.extend_from_slice(&apk_data[(block.cde_start as usize)..]);
    
    // Update CD offset in EOCD record (offset 16 from cde_start)
    let offset_pos = (new_cde_start + 16) as usize;
    if offset_pos + 4 <= output.len() {
        output[offset_pos..offset_pos + 4].copy_from_slice(&(new_cd_start as u32).to_le_bytes());
    }
    
    Ok(output)
}

#[derive(Debug, Default)]
struct ApkSignatureBlock {
    pub blocks: Vec<ApkOpaqueBlock>,
    pub sb_start: u64,
    pub cd_start: u64,
    pub cde_start: u64,
}

#[derive(Clone, Copy, Debug)]
struct ApkOpaqueBlock {
    pub id: u32,
    #[allow(dead_code)]
    pub start: u64,
}

fn parse_apk_signing_block<R: Read + Seek>(r: &mut R) -> Result<ApkSignatureBlock> {
    let info = xcommon::ZipInfo::new(r)?;
    let mut block = ApkSignatureBlock {
        cde_start: info.cde_start,
        cd_start: info.cd_start,
        ..Default::default()
    };
    
    // Check if signing block exists
    if block.cd_start < 24 {
        block.sb_start = block.cd_start;
        return Ok(block);
    }
    
    r.seek(SeekFrom::Start(block.cd_start - 16 - 8))?;
    let remaining_size = r.read_u64::<LittleEndian>()?;
    let mut magic = [0; 16];
    r.read_exact(&mut magic)?;
    
    if magic != APK_SIGNING_BLOCK_MAGIC {
        block.sb_start = block.cd_start;
        return Ok(block);
    }
    
    let mut pos = r.seek(SeekFrom::Current(-(remaining_size as i64)))?;
    block.sb_start = pos - 8;
    
    while remaining_size > 24 {
        let length = r.read_u64::<LittleEndian>()?;
        let id = r.read_u32::<LittleEndian>()?;
        block.blocks.push(ApkOpaqueBlock {
            id,
            start: pos + 8 + 4,
        });
        pos = r.seek(SeekFrom::Start(pos + length + 8))?;
        break; // Only read first block for now
    }
    
    Ok(block)
}

fn compute_digest<R: Read + Seek>(
    r: &mut R,
    sb_start: u64,
    cd_start: u64,
    cde_start: u64,
) -> Result<[u8; 32]> {
    let mut chunks = vec![];
    let mut hasher = Sha256::new();
    let mut chunk = vec![0u8; MAX_CHUNK_SIZE];

    // Chunk contents (before signing block)
    let mut pos = r.seek(SeekFrom::Start(0))?;
    while pos < sb_start {
        hash_chunk(&mut chunks, r, sb_start, &mut hasher, &mut chunk, &mut pos)?;
    }

    // Chunk central directory
    let mut pos = r.seek(SeekFrom::Start(cd_start))?;
    while pos < cde_start {
        hash_chunk(&mut chunks, r, cde_start, &mut hasher, &mut chunk, &mut pos)?;
    }

    // Chunk EOCD (with modified CD offset)
    chunk.clear();
    r.read_to_end(&mut chunk)?;
    
    // Modify EOCD to point to original sb_start
    if chunk.len() >= 20 {
        let mut cursor = Cursor::new(&mut chunk);
        cursor.seek(SeekFrom::Start(16))?;
        cursor.write_u32::<LittleEndian>(sb_start as u32)?;
    }
    
    hasher.update([0xa5]);
    hasher.update((chunk.len() as u32).to_le_bytes());
    hasher.update(&chunk);
    chunks.push(hasher.finalize_reset().into());

    // Compute root hash
    hasher.update([0x5a]);
    hasher.update((chunks.len() as u32).to_le_bytes());
    for chunk in &chunks {
        hasher.update(chunk);
    }
    
    Ok(hasher.finalize().into())
}

fn hash_chunk<R: Read + Seek>(
    chunks: &mut Vec<[u8; 32]>,
    r: &mut R,
    size: u64,
    hasher: &mut Sha256,
    buffer: &mut Vec<u8>,
    pos: &mut u64,
) -> Result<()> {
    let end = std::cmp::min(*pos + MAX_CHUNK_SIZE as u64, size);
    let len = (end - *pos) as usize;
    buffer.resize(len, 0);
    r.read_exact(buffer)?;
    hasher.update([0xa5]);
    hasher.update((len as u32).to_le_bytes());
    hasher.update(&*buffer);
    chunks.push(hasher.finalize_reset().into());
    *pos = end;
    Ok(())
}

fn write_apk_signing_block<W: Write + Seek>(
    w: &mut W,
    hash: [u8; 32],
    signer: &Signer,
) -> Result<()> {
    let mut buf = vec![];
    write_signature_block_v2(&mut buf, hash, signer)?;
    
    let size = buf.len() as u64 + 36;
    w.write_u64::<LittleEndian>(size)?;
    w.write_u64::<LittleEndian>(buf.len() as u64 + 4)?;
    w.write_u32::<LittleEndian>(APK_SIGNING_BLOCK_V2_ID)?;
    w.write_all(&buf)?;
    w.write_u64::<LittleEndian>(size)?;
    w.write_all(APK_SIGNING_BLOCK_MAGIC)?;
    
    Ok(())
}

fn write_signature_block_v2<W: Write>(
    w: &mut W,
    hash: [u8; 32],
    signer: &Signer,
) -> Result<()> {
    // Create signed data
    let mut signed_data = vec![];
    write_signed_data(&mut signed_data, hash, signer)?;
    
    // Sign the data
    let signature = signer.sign(&signed_data);
    let public_key = signer.pubkey().to_public_key_der()?.as_ref().to_vec();
    
    // Write signer block
    let mut signer_buffer = vec![];
    
    // Signed data
    signer_buffer.write_u32::<LittleEndian>(signed_data.len() as u32)?;
    signer_buffer.write_all(&signed_data)?;
    
    // Signatures
    let mut sig_buffer = vec![];
    sig_buffer.write_u32::<LittleEndian>(signature.len() as u32 + 8)?;
    sig_buffer.write_u32::<LittleEndian>(RSA_PKCS1V15_SHA2_256)?;
    sig_buffer.write_u32::<LittleEndian>(signature.len() as u32)?;
    sig_buffer.write_all(&signature)?;
    
    signer_buffer.write_u32::<LittleEndian>(sig_buffer.len() as u32)?;
    signer_buffer.write_all(&sig_buffer)?;
    
    // Public key
    signer_buffer.write_u32::<LittleEndian>(public_key.len() as u32)?;
    signer_buffer.write_all(&public_key)?;
    
    // Write total
    let mut buffer = vec![];
    buffer.write_u32::<LittleEndian>(signer_buffer.len() as u32)?;
    buffer.write_all(&signer_buffer)?;
    
    w.write_u32::<LittleEndian>(buffer.len() as u32)?;
    w.write_all(&buffer)?;
    
    Ok(())
}

fn write_signed_data<W: Write>(w: &mut W, hash: [u8; 32], signer: &Signer) -> Result<()> {
    // Digests
    let mut digests = vec![];
    digests.write_u32::<LittleEndian>(hash.len() as u32 + 8)?;
    digests.write_u32::<LittleEndian>(RSA_PKCS1V15_SHA2_256)?;
    digests.write_u32::<LittleEndian>(hash.len() as u32)?;
    digests.write_all(&hash)?;
    
    w.write_u32::<LittleEndian>(digests.len() as u32)?;
    w.write_all(&digests)?;
    
    // Certificates
    let cert_der = rasn::der::encode(signer.cert()).map_err(|e| anyhow::anyhow!("{}", e))?;
    w.write_u32::<LittleEndian>(cert_der.len() as u32 + 4)?;
    w.write_u32::<LittleEndian>(cert_der.len() as u32)?;
    w.write_all(&cert_der)?;
    
    // Additional attributes (empty)
    w.write_u32::<LittleEndian>(0)?;
    
    Ok(())
}
