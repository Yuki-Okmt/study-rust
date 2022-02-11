use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }
    // コマンドライン引数を得る
    let target_dir = &args[1];
    let keyword = &args[2];
    // PathBufに変換
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

// 再起的にファイルを検索する関数
fn findfile(target: &path::PathBuf, keyword: &str) {
    // ファイル一覧を取得
    let files = target.read_dir().expect("存在しないパス");
    for dir_entry in files {
        // PatchBufを得る
        let path = dir_entry.unwrap().path();
        // ディレクトリーならば再起的に検索
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        // ファイル名を文字列に変換
        let fname = path.file_name().unwrap().to_string_lossy();
        // キーワードを含むかチェック
        if None == fname.find(keyword) {
            continue;
        }
        // キーワードを含むパスを表示
        println!("{}", path.to_string_lossy());
    }
}
