use mimalloc::MiMalloc ;  // Встроенный mimalloc глобадьный алокатор от Microsoft.

#[global_allocator] // Атрибут macro применённый к static переменной чтобы зарегистрировать его как глобальный алокатор.
static MY_GLOB_ALLOC: MiMalloc = MiMalloc ; // имя переменной любое, главное атрибут global_allocator


fn main() {
    let v = vec![0u32; 10] ;

    println!("v: {:?}", v) ;    // Out: v: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
}
