/** 
 * in rust, there is no type checker (like typeof)
 * to solve that problem, this func is created
 */
pub fn what_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
