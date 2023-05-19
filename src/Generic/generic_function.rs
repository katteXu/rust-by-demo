struct A; // 具体类型 `A`。
struct S(A); // 具体类型 `S`。
struct SGen<T>(T); // 泛型类型 `SGen`。

fn reg_fn(_s: S) {}

// 不是泛型函数
fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

// 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // 显式指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式指定类型参数 `char`
    generic(SGen('b'));
}
