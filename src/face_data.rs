use std::sync::Arc;

use thiserror::Error;
use winreg::{
    enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE},
    transaction::Transaction,
    RegKey, RegValue,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FaceData {
    pub face_text: String,
    pub face_color_index: u32,
    // "FaceRotation_h3532958608"=hex(4):00,00,00,80,b0,7d,a7,3f
    // "VisorColor_h2089036713"=hex(4):00,00,00,e0,f2,b7,eb,3f
    // "FaceSize_h3883780417"=hex(4):00,00,00,a0,99,99,99,3f
}

impl FaceData {
    const CONFIG_KEY: &'static str = "Software\\Landfall Games\\Content Warning";
    const FACE_TEXT_KEY: &'static str = "FaceText_h3883740665";
    const FACE_COLOR_INDEX_KEY: &'static str = "FaceColorIndex_h311401607";

    pub fn load_from_registry() -> Result<Self, FaceLoadError> {
        log::info!("Loading face data from registry");
        let t = Transaction::new()?;
        let game_keys =
            RegKey::predef(HKEY_CURRENT_USER).open_subkey_transacted(FaceData::CONFIG_KEY, &t)?;
        let face_text_bytes = game_keys.get_raw_value(FaceData::FACE_TEXT_KEY)?;
        let face_text = str_from_u8_nul_utf8(&face_text_bytes.bytes)?;
        let face_color_index: u32 = game_keys.get_value(FaceData::FACE_COLOR_INDEX_KEY)?;
        Ok(Self {
            face_color_index,
            face_text: face_text.to_string(),
        })
    }

    pub fn save_to_registry(&self) -> Result<(), std::io::Error> {
        log::info!("Saving face data to registry");

        let t = Transaction::new()?;
        let game_keys = RegKey::predef(HKEY_CURRENT_USER).open_subkey_transacted_with_flags(
            FaceData::CONFIG_KEY,
            &t,
            KEY_WRITE | KEY_READ,
        )?;
        game_keys.set_raw_value(
            FaceData::FACE_TEXT_KEY,
            &RegValue {
                vtype: winreg::enums::REG_BINARY,
                bytes: str_to_u8_nul_utf8(&self.face_text),
            },
        )?;
        game_keys.set_value(FaceData::FACE_COLOR_INDEX_KEY, &self.face_color_index)?;
        t.commit()
    }
}

impl Default for FaceData {
    fn default() -> Self {
        Self {
            face_text: ":)".to_string(),
            face_color_index: 0,
        }
    }
}

// https://stackoverflow.com/a/42067321
fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    ::std::str::from_utf8(&utf8_src[0..nul_range_end])
}

fn str_to_u8_nul_utf8(s: &str) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.push(0);
    v
}

#[derive(Error, Debug, Clone)]
pub enum FaceLoadError {
    #[error("Failed to load face data from registry")]
    RegistryLoadError(Arc<std::io::Error>),
    #[error("Failed to parse face text as UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),
}

impl From<std::io::Error> for FaceLoadError {
    fn from(e: std::io::Error) -> Self {
        FaceLoadError::RegistryLoadError(Arc::new(e))
    }
}
