use std::path::{Path, Prefix};

pub trait GetDiskLetter {
    fn get_disk_letter(&self) -> Option<char>;
}

impl<T: AsRef<Path>> GetDiskLetter for T {
    fn get_disk_letter(&self) -> Option<char> {
        self.as_ref().components()
            .find_map(|component| {
                if let std::path::Component::Prefix(prefix_component) = component {
                    if let Prefix::Disk(letter) = prefix_component.kind() {
                        return Some(letter as char);
                    }
                }
                None
            })
    }
}
