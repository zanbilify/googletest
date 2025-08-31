use fs_extra::dir::CopyOptions;
use zanbil_build::init_zanbil_crate;

fn main() {
    let zc = init_zanbil_crate();
    fs_extra::dir::copy(
        build_rs::input::cargo_manifest_dir().join("include"),
        zc.include_dir,
        &CopyOptions::new().content_only(true),
    )
    .unwrap();

    let mut cc = cc::Build::new();
    if let Some(compiler) = &zc.compiler {
        cc.compiler(compiler);
    }
    cc.includes(&zc.aggregated_include_dirs);
    cc.includes(["."]);

    for entry in walkdir::WalkDir::new("src") {
        let entry = entry.unwrap();
        let path = entry.path().to_path_buf();
        if path.extension().and_then(|x| x.to_str()) == Some("cc") {
            build_rs::output::rerun_if_changed(&path);
            cc.file(&path);
        }
    }

    cc.compile("main");
    build_rs::output::rustc_link_lib("main");
}
