// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir);

        // 1. Создаем сам XML манифест
        let manifest_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
        <security>
            <requestedPrivileges>
                <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
            </requestedPrivileges>
        </security>
    </trustInfo>
</assembly>"#;
        let manifest_path = dest_path.join("app.manifest");
        fs::write(&manifest_path, manifest_xml).unwrap();

        // 2. Создаем .rc файл, который ссылается на этот манифест
        // RT_MANIFEST имеет ID 24, а идентификатор ресурса обычно 1
        let rc_content = format!("1 24 \"{}\"", manifest_path.to_str().unwrap().replace("\\", "\\\\"));
        let rc_path = dest_path.join("resource.rc");
        fs::write(&rc_path, rc_content).unwrap();

        // 3. Компилируем именно .rc файл
        embed_resource::compile(rc_path, embed_resource::NONE);
    }
}