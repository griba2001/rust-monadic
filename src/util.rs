// mod util

pub fn concat_vec_array<T: Clone>( mut v1: Vec<T>, v2: &[T]) -> Vec<T> {

   v1.append( &mut Vec::from(v2));
   v1
}

pub fn concat_string_str( mut s1: String, s2: &str) -> String {
   s1.push_str( s2);
   s1
}
