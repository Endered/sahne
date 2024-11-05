# sahne

## はじめに
sahne(ドイツ語で生クリーム)はトレイトの実装として他のトレイトを利用するときのヘルパーの立ち位置を目指したRust向けライブラリです。
Scalaで多様されているMixinを基本としたDIをsahneで模倣することが目標です。
> このライブラリの用途に悩まれる方は、Cake PatternやMinimal Cake Pattern等で調べられることをお勧めします。

## 使用例
```
trait Animal {
    fn kind(&self) -> String;
}

#[provider(Animal)]
trait Dog {
    fn kind(&self) -> String {
        "Dog".to_string()
    }
}

#[provider(Animal)]
trait Cat {
    fn kind(&self) -> String {
        "Cat".to_string()
    }
}

#[mixin(Dog)]
struct DogImpl {}

#[mixin(Cat)]
struct CatImpl {}

fn describe_animal<T: Animal>(animal: T) {
    println!("This is a {}", animal.kind());
}

fn main() {
    let cat = CatImpl {};
    let dog = DogImpl {};

    describe_animal(cat);
    describe_animal(dog);
}
```
mixin属性にはカンマ区切りで複数のトレイトを指定することができます。

利用例は [example](./example) にあるので参考にしてください。


## サポートしていないこと
現状、以下のことは考慮していません(動くかもしれませんが)

- 関連型の利用
- async/await (直ぐに対処予定ではあります)
- `#[hoge]` などで属性の付いた関数
  - マクロ側では無いものとして扱います
- unsafe
- ジェネリクス
