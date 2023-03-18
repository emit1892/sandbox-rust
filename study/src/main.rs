// 定数
// 任意のスコープに設定可能
// 定数は必ず型名を明記し、大文字とする
const B: i32 = 2;

fn main() {
    // 変数宣言
    // 変数に値をいれることを「束縛する」などという
    let a: i32 = 1;
    println!("{}", a);

    // mutをつけると可変とされる
    let mut b: i32 = 2;
    b = 2;

    let c: i32 = 1;

    // 変数と初期化を分けて書く
    let d: i32;
    d = 1;

    // シャドーイング(変数宣言の上書き)
    let d: &str = "test";

    // 数値型
    let a = 1;
    let b = 2.0;

    // 型を指定
    let c: u16 = 3;
    // 右辺がリテラルの場合は↓のように指定可能
    let d = 4.0f32;

    let e = 1 + 2;
    let e = 1 - 2;
    let e = 1 * 2;
    let e = 1 / 2;
    let e = 1 % 2;

    // cast
    let f = 1 as f64 + 2.0;

    // 論理型
    // true or false
    let g = false;

    let h = 1 == 2;
    let h = 1 != 2;
    let h = 1 > 2;
    let h = 1 < 2;
    let h = 1 >= 2;
    let h = 1 <= 2;

    println!("Hello, world!");
    print!("Heelo, ");
    print!("Rust!");
    
    // {}の中にstudentsが挿入される
    println!("Hello, {}", "students");

    /* 複数行のコメントアウト */
}
