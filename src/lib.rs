#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    

    if _first_list == _second_list{
        return Comparison::Equal;
        
    }
    
    if _first_list.is_empty(){
        return Comparison::Sublist;
    }
    if _second_list.is_empty(){
        return Comparison::Superlist;
    }
    
    if _first_list.len() > _second_list.len(){
        if _first_list.windows(_second_list.len()).any(|window| window == _second_list){
            return Comparison::Superlist;
        }
    }
    if _first_list.len() < _second_list.len(){
        if _second_list.windows(_first_list.len()).any(|window| window == _first_list){
            return Comparison::Sublist;
        }
        
    } 

    Comparison::Unequal
        
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");
}
