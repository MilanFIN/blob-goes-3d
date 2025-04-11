extern crate alloc;
use agb::save::Error;
use agb::InternalAllocator;
use alloc::vec::Vec;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum SaveType {
    Sram32K,
    Flash64K,
    Flash128K,
    Eeprom512B,
    Eeprom8K,
    None,
}

pub fn init_save(gba: &mut agb::Gba, save_type: SaveType) {
    match save_type {
        SaveType::Sram32K => gba.save.init_sram(),
        SaveType::Flash64K => gba.save.init_flash_64k(),
        SaveType::Flash128K => gba.save.init_flash_128k(),
        SaveType::Eeprom512B => gba.save.init_eeprom_512b(),
        SaveType::Eeprom8K => gba.save.init_eeprom_8k(),
        SaveType::None => {
            // No save type, do nothing
        }
    }
}

pub fn store_save(
    gba: &mut agb::Gba,
    data: &mut Vec<bool, InternalAllocator>,
    save_type: SaveType,
) -> Result<(), Error> {
    if save_type == SaveType::None {
        return Ok(());
    }
    let mut access = gba.save.access()?;
    let bytes: Vec<u8> = data.iter().map(|&b| b as u8).collect();

    access.prepare_write(0..bytes.len())?.write(0, &bytes)?;
    Ok(())
}

pub fn read_save(
    gba: &mut agb::Gba,
    length: usize,
    save_type: SaveType,
) -> Result<Vec<bool, InternalAllocator>, Error> {
    let mut booleans: Vec<bool, InternalAllocator> =
        Vec::with_capacity_in(length, InternalAllocator);

    if save_type == SaveType::None {
        booleans.resize(length, false);
    } else {
        let mut access = gba.save.access()?;

        let mut bytes: Vec<u8, InternalAllocator> =
            Vec::with_capacity_in(length, InternalAllocator);
        // Read each byte separately
        for i in 0..length {
            let mut val = 0;
            access.read(i, core::slice::from_mut(&mut val))?;
            if val == 1 {
                bytes.push(1);
            } else {
                bytes.push(0);
            }
        }

        // booleans = Vec::with_capacity_in(bytes.len(), InternalAllocator);
        for &byte in bytes.iter() {
            booleans.push(byte != 0);
        }
    }
    return Ok(booleans);
}
