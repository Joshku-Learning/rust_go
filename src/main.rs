// 註解用法使用//
// 這是一個猜數字的程式
// 這邊宣告要使用rand這個套件
// Rust 裡只要不加分號就會變回傳值
extern crate rand;

// 引入需要的函式,以下是比較方便的寫法,
// 很多套件為了方便使用,都會提供叫preLude的模組,
use rand::prelude::*;
// 這邊是標準輸入,也就是鍵盤輸入
use std::io::stdin;

// rust的程式都會由main這個fn開始做入口,跟golang很類似
fn main() {
    println!("Hello, world!");
    // 這邊定義一個變數ans 來當做我們的答案
    // 設定1-100之間的隨機數字
    let ans = thread_rng().gen_range(1..100);

    // 定義兩個變數,分別代表答案所在的上下範圍,
    // 加上mut來表示可修改的
    let mut upper_bound = 100;
    let mut lower_bound = 1;

    // 迴圈寫法 沒有終止條件,需要執行break才會停止
    loop {
        // 這邊暫時放玩家輸入的答案用的變數
        // String 是個字串型態,String:new 會建立一個空的字串
        let mut input = String::new();

        // 這邊要印出提示使用者輸入的顯示,同時也印出答案所在的上下界,
        // println! 會自動換行,
        // 裡面用 {} 占位置用的
        println!(
            "答案在 {}~{} 之間,請輸入一個數字",
            lower_bound, upper_bound
        );

        // 使用read_line 從鍵盤讀入一整行,
        // 進來的文字會放入input裡
        // 使用&mut 來允許 read_line 修改input
        // 用expect來處理,若回傳的值代表錯誤時, expect會印出使者給他的訊息並結束程式
        stdin().read_line(&mut input).expect("Fail to read input");

        let input = match input.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Please input a number!!!");
                continue;
            }
        };

        if input == ans {
            println!("congratulation!!,you are true");
            break;
        } else if input > ans {
            upper_bound = input;
        } else {
            lower_bound = input;
        }
    }
}
