fn main() {
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // この変数はスコープ外の同名の変数を *シャドーイング* します。
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // この変数束縛は以前に定義した変数を *シャドーイング* します。
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
