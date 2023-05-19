/**
 * 标准类型
 * 有符号整数  i8,i16,i32,i64,i128 和 isize(指针宽度)
 * 无符号整数  u8,u16,u32,u64,u128 和 usize(指针宽度）
 * 浮点数 f32,f64
 * 字符 char 'a'  'α'  'Ω' (每个都是4字节)
 * bool true false
 * 单元类型 ()
 */

/**
 * 复合类型
 * 数组  [1,2,3]
 * 元祖  (1,true)
 */

fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;

    let an_integer = 5i32;

    let default_float = 1.0;
    let default_integer = 5;

    let mut inferred_type = 12;
    inferred_type = 44433321i64;

    let mut mutable = 12;
    mutable = 21;

    // mutable= true;

    let mutable = true;
}
