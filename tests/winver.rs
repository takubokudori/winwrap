#[cfg(test)]
pub mod winver_tests {
    use winwrap::um::winver::*;
    use winwrap::string::*;

    #[test]
    fn test_ver_language_name() {
        let lang_a = ver_language_name_a(0x40a).unwrap();
        let lang_w = ver_language_name_w(0x40a).unwrap();
        assert_eq!(lang_a.to_string_lossy(), lang_w.to_string_lossy());
    }

    #[test]
    fn test_ver_query_value() {
        let file_name_a = AString::from("ntdll.dll");
        let file_name_w = WString::from("ntdll.dll");
        let len_a = get_file_version_info_size_a(&file_name_a).unwrap();
        let len_w = get_file_version_info_size_w(&file_name_w).unwrap();
        let fvi_a = get_file_version_info_a(&file_name_a, len_a).unwrap();
        let fvi_w = get_file_version_info_w(&file_name_w, len_w).unwrap();
        let root_a = ver_query_value_a(&fvi_a, VerQuerySubBlock::Root).unwrap();
        let root_w = ver_query_value_w(&fvi_w, VerQuerySubBlock::Root).unwrap();
        assert_eq!(root_a.as_root(), root_w.as_root());
        let vfi_a = ver_query_value_a(&fvi_a, VerQuerySubBlock::VarFileInfo).unwrap();
        let vfi_w = ver_query_value_w(&fvi_w, VerQuerySubBlock::VarFileInfo).unwrap();
        let vfi_a = vfi_a.as_var_file_info();
        let vfi_w = vfi_w.as_var_file_info();
        let (language, code_page) = (vfi_a.language, vfi_a.code_page);
        let sfi_a = ver_query_value_a(
            &fvi_a,
            VerQuerySubBlock::StringFileInfo(language, code_page, "ProductVersion".to_string()),
        ).unwrap();
        let (language, code_page) = (vfi_w.language, vfi_w.code_page);
        let sfi_w = ver_query_value_w(
            &fvi_w,
            VerQuerySubBlock::StringFileInfo(language, code_page, "ProductVersion".to_string()),
        ).unwrap();
        assert_eq!(sfi_a.as_string_file_info().to_string_lossy(), sfi_w.as_string_file_info().to_string_lossy())
    }
}
