
// 1. 生命周期避免了悬垂引用问题
//   - 悬垂引用问题：引用指向的内存地址在引用被使用后已被释放，导致引用无效
//   - 生命周期确保了引用的内存地址在引用被使用前是有效的，避免了悬垂引用问题


// 2. 生命周期标注语法
//  - &i32        // 引用
//  - &'a i32     // 带有显式生命周期的引用
//  - &'a mut i32 // 带有显式生命周期的可变引用


// 3. 函数签名中的生命周期标注
// fn longest<'a>(x: &'a &str, y: &'a &str) -> &'a &str
// 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致
// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。

// longest 函数定义指定了签名中所有的引用必须有相同的生命周期 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    {
        // 外部作用域
        let r;                                                  // ---------+-- 'a
                                                                //          |
        {                                                       //          |
            // 内部作用域                                        // -+-- 'b  |
            let x = 5;                                          //  |       |
            r = &x;                                             //  |       |
        }                                                       // -+       |
                                                                //          |
        // println!("r: {}", r); // 报错，x 已离开内部作用域         //          |
    } 




    {
        // 2. 调用 longest 函数来寻找两个字符串 slice 中较长的一个
        let string1 = String::from("abcd");
        let string2 = "xyz";
        println!("String1: {}", string1.as_str());
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }


    {
        // 3. 通过拥有不同的具体生命周期的 String 值调用 longest 函数
        let string3 = String::from("long string is long");

        {
            let string4 = String::from("xyz");
            let result = longest2(string3.as_str(), string4.as_str());
            println!("The longest string2 is {}", result);
        }
    }


    {
        // 4. 尝试在 string2 离开作用域之后使用 result 引用
        // 报错，string2 已离开作用域，result 引用指向的内存地址已被释放
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    }



    {
        // 5. 深入理解生命周期

        // 5.1 y 的生命周期与参数 x 和返回值的生命周期没有任何关系，因此没有为参数 y 指定生命周期参数 'a
        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }


        // 5.2 即便我们为返回值指定了生命周期参数 'a，这个实现却编译失败了，因为返回值的生命周期与参数完全没有关联。
        // 出现的问题是 result 在 longest 函数的结尾将离开作用域并被清理，而我们尝试从函数返回一个 result 的引用。
        fn longest2<'a>(x: &str, y: &str) -> &'a str {
            let result = String::from("really long string");
            result.as_str()
        }
    }




    {
        // 6. 结构体定义中的生命周期标注

        // 6.1 一个存放引用的结构体，所以其定义需要生命周期标注
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }


    {
        // 7. 生命周期省略（Lifetime Elision）
        // 被编码进 Rust 引用分析的模式被称为 生命周期省略规则
        // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。

        // 规则：
        //   - 每一个是引用的参数都有它自己的生命周期参数。
        //   - 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
        //   - 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)，那么所有输出生命周期参数被赋予 self 的生命周期。

        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
    }


    {
        // 8、方法定义中的生命周期标注

        // impl 之后和类型名称之后的生命周期参数是必要的
        // 第一条生命周期规则我们并不必须标注 self 引用的生命周期
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }

            // 第三条生命周期规则
            // 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。
            // 接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }



    {
        // 9. 静态生命周期
        // 静态生命周期的声明语法是 &'static
        // 其生命周期能够存活于整个程序期间

        // 这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。
        let s: &'static str = "I have a static lifetime.";

    }


    {
        // 10. 结合泛型类型参数、trait bounds 和生命周期


        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where
            T: Display,
        {
            println!("Announcement! {}", ann);

            if(x.len() > y.len()) {
                x
            } else {
                y
            }
        }
    }

}
