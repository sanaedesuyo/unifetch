fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/unifetch.ico"); // 图标路径（必须是 .ico 格式）
        res.compile().unwrap();
    }
}