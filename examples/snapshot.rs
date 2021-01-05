// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
use winwrap::{
    handle::SnapshotHandle,
    um::{processthreadsapi::get_current_process_id, tlhelp32::*},
};

pub struct ModuleIterator(bool, SnapshotHandle);

impl ModuleIterator {
    pub fn iter() -> Self {
        Self(
            false,
            create_tool_help32_snapshot(
                TH32CSFlags::SNAPMODULE,
                get_current_process_id(),
            )
            .unwrap(),
        )
    }
}

impl Iterator for ModuleIterator {
    type Item = ModuleEntry32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = Default::default();
        if !self.0 {
            // first time
            if let Err(x) = module32_first(&self.1, &mut entry) {
                println!("err: {}", x);
                return None;
            }
            self.0 = true;
            Some(entry)
        } else {
            if module32_next(&self.1, &mut entry).is_err() {
                return None;
            }
            Some(entry)
        }
    }
}

pub struct ModuleWIterator(bool, SnapshotHandle);

impl ModuleWIterator {
    pub fn iter() -> Self {
        Self(
            false,
            create_tool_help32_snapshot(
                TH32CSFlags::SNAPMODULE,
                get_current_process_id(),
            )
            .unwrap(),
        )
    }
}

impl Iterator for ModuleWIterator {
    type Item = ModuleEntry32W;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = Default::default();
        if !self.0 {
            // first time
            if let Err(x) = module32_first_w(&self.1, &mut entry) {
                println!("err:{}", x);
                return None;
            }
            self.0 = true;
            Some(entry)
        } else {
            if module32_next_w(&self.1, &mut entry).is_err() {
                return None;
            }
            Some(entry)
        }
    }
}

pub struct ProcessWIterator(bool, SnapshotHandle);

impl ProcessWIterator {
    pub fn iter() -> Self {
        Self(
            false,
            create_tool_help32_snapshot(
                TH32CSFlags::SNAPPROCESS,
                get_current_process_id(),
            )
            .unwrap(),
        )
    }
}

impl Iterator for ProcessWIterator {
    type Item = ProcessEntry32W;

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = Default::default();
        if !self.0 {
            // first time
            if let Err(x) = process32_first_w(&self.1, &mut entry) {
                println!("err:{}", x);
                return None;
            }
            self.0 = true;
            Some(entry)
        } else {
            if process32_next_w(&self.1, &mut entry).is_err() {
                return None;
            }
            Some(entry)
        }
    }
}

pub fn main() {
    let mi = ModuleIterator::iter();
    println!("******ModuleEntry32******");
    for mut x in mi {
        println!("----------------");
        println!("szExePath  : {}", x.get_exe_path().to_string_lossy());
        println!("szModule   : {}", x.get_module_name().to_string_lossy());
        println!("modBaseAddr: {:X}", x.mod_base_addr as usize);
        println!("modBaseSize: {:X}", x.mod_base_size);
    }
    let mi = ModuleWIterator::iter();
    println!("\n******ModuleEntry32W******");
    for mut x in mi {
        println!("----------------");
        println!("szExePath  : {}", x.get_exe_path().to_string().unwrap());
        println!("szModule   : {}", x.get_module_name().to_string().unwrap());
        println!("modBaseAddr: {:X}", x.mod_base_addr as usize);
        println!("modBaseSize: {:X}", x.mod_base_size);
    }
    let mi = ProcessWIterator::iter();
    println!("\n******ProcessEntry32W******");
    for mut x in mi {
        println!("----------------");
        println!("szExePath  : {}", x.get_exe_file().to_string().unwrap());
        println!("ProcessID: {}", x.process_id);
        println!("CountThreads: {}", x.cnt_threads);
        println!("ParentProcessID: {}", x.parent_process_id);
        println!("PriClassBase: {}", x.pri_class_base);
    }
}
