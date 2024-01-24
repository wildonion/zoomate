/* ------------------ 
  |    DSL MACROS
  |------------------
  |
  | dsl macros traits as plugin



    ---------------- MACRO PATTERNS -----------------

    rust types can be fallen into one the following categories

    item      ➔ an Item | an item, like a function, struct, module, etc.
    block     ➔ a BlockExpression | a block (i.e. a block of statements and/or an expression, surrounded by braces)
    stmt      ➔ a Statement without the trailing semicolon (except for item statements that require semicolons)
    pat_param ➔ a PatternNoTopAlt
    pat       ➔ at least any PatternNoTopAlt, and possibly more depending on edition
    expr      ➔ an Expression
    ty        ➔ a Type
    ident     ➔ an IDENTIFIER_OR_KEYWORD or RAW_IDENTIFIER
    path      ➔ a TypePath style path | a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
    tt        ➔ a TokenTree (a single token or tokens in matching delimiters (), [], or {})
    meta      ➔ an Attr, the contents of an attribute | a meta item; the things that go inside #[...] and #![...] attributes
    lifetime  ➔ a LIFETIME_TOKEN
    vis       ➔ a possibly empty Visibility qualifier
    literal   ➔ matches -?LiteralExpression

    
*/

#[macro_export]
macro_rules! o_O {
    (
        $(
            $x:expr; [ $( $y:expr ), * ]
        ); * /* multiple of this pattern */
    ) => {
        &[ $($( $x + $y ), *), * ]
    }
}
//////
/// let a: &[i32] = o_O![10; [1, 2, 3]; 20; [4, 5, 6]];
//////

#[macro_export]
macro_rules! list {
    ($id1:ident | $id2:ident <- [$start:expr; $end:expr], $cond:expr) => { //// the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc
        { //.... code block to return vec since if we want to use let statements we must be inside {} block
            let mut vec = Vec::new();
            for num in $start..$end + 1{
                if $cond(num){
                    vec.push(num);
                }
            }
            vec
        } //....
    };
}
//////
/// let even = |x: i32| x%2 == 0;
/// let odd = |x: i32| x%2 != 0;
/// let evens = list![x | x <- [1; 10], even];
//////

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr)*) => { //// if this pattern matches the input the following code will be executed - * means we can pass more than one key => value statement
        { //.... code block to return vec since if we want to use let statements we must be inside {} block
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )* //// * means we're inserting multiple key => value statement inside the map 
            map
        } //....
    };
}
//////
/// let d = dict!{"wildonion" => 1, "another_wildonion" => 2, "array": vec![1,3,4235,], "age": 24};
//////

#[macro_export]
macro_rules! exam {
    ($l:expr; and $r:expr) => { //// logical and match 
        $crate::macros::even(); //// calling even() function which is inside the macros module
        println!("{}", $l && $r);
    };

    ($l:expr; or $r:expr) => { //// logical or match 
        println!("{}", $l || $r);
    };
}
//////
/// exam!(1 == 2; and 3 == 2+1)
/// exam!(1 == 2; or 3 == 2+1)
//////


#[macro_export]
macro_rules! cmd {
    ($iden:ident, $ty: tt) => {
        pub struct $iden(pub $ty);
        impl Default for $iden{
            fn default() -> Self{
                todo!()
            }
        }  
    };

    ($func_name:ident) => {
        fn $func_name(){
            println!("you've just called {:?}()", stringify!($func_name));
        }
    }
}
//////
/// cmd!{bindgen, id} //// bindgen is the name of the struct and id is the name of the field
//////


#[macro_export]
macro_rules! query { // NOTE - this is a macro with multiple syntax support and if any pattern matches with the caller pattern, then the code block of that pattern will be emitted
    
    ( $value_0:expr, $value_1:expr, $value_2:expr ) => { //// passing multiple object syntax
        // ...
    };

    ( $($name:expr => $value:expr)* ) => { //// passing multiple key => value syntax 
        // ...

    };

}

#[macro_export]
macro_rules! dynamic_methods {
    ($builder:ident, $($field:ident: $field_type:ty),*) => {
        impl $builder {
            $(
                pub fn $field(mut self, $field: $field_type) -> Self {
                    self.$field = Some($field);
                    self
                }
            )*
        }
    };
}
//////
/// dynamic_methods!{StructName, id: None, name: None, age: i32}
//////

#[macro_export]
macro_rules! log {
    ($arg:tt) => { //// passing single String message 
        $crate::env::log($arg.as_bytes()) //// log function only accepts utf8 bytes
    };
    ($($arg:tt)*) => { //// passing multiple String messages 
        $crate::env::log(format!($($arg)*).as_bytes()) //// log function only accepts utf8 bytes
    };
}


#[macro_export]
macro_rules! impl_ecq_engine_constructor {
    ($( $new:ident: [ $( $pos:expr ),* ] anchored at $anchor:expr; )*) => { //// the match pattern can be any syntax :) - only ident can be followed by some symbols and words like <-, |, @ and etc 
        $(
            pub fn $new() -> Self{
                Self{
                    positions: [$( $pos ),*].into_iter().collect(),
                    anchor: $anchor,
                }
            }
        )* //// * means defining function for every new Pos
    };
}

// #[derive(Debug, Clone)]
// pub struct Shape{
//     typ: &'static str,
//     positions: HashSet<Pos>,
//     anchor: Pos,
// }


// #[derive(Debug, Clone, Copy)]
// pub struct Pos(pub i32, pub i32);



// impl Shape {
//     impl_ecq_engine_constructor! {
//       new_i "🟦": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)] @ Pos(1, 0);
//       new_o "🟨": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//       new_t "🟫": [Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(1, 1)] @ Pos(1, 0);
//       new_j "🟪": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(-1, 2)] @ Pos(0, 1);
//       new_l "🟧": [Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(1, 2)] @ Pos(0, 1);
//       new_s "🟩": [Pos(0, 0), Pos(1, 0), Pos(0, 1), Pos(-1, 1)] @ Pos(0, 0);
//       new_z "🟥": [Pos(0, 0), Pos(-1, 0), Pos(0, 1), Pos(1, 1)] @ Pos(0, 0);
//     }
// }

#[macro_export]
macro_rules! iterator{
    ($ty:ty, $ident:ident; $($state_ident:ident: $state_ty:ty),*; $next:expr) => (
        struct $ident {
            $($state_ident: $state_ty), *
        }

        impl Iterator for $ident {
            type Item = $ty;

            fn next(&mut self) -> Option<$ty> {
                $next(self)
            }
        }
    );
}
//////
// iterator!(i32, TestIterator; index: i32; |me: &mut TestIterator| {
//     let value = Some(me.index);
//     me.index += 1;
//     value
// });
//////


macro_rules! pat {
    ($i:ident) => (Some($i))
}

// if let pat!(x) = Some(1) {
//     assert_eq!(x, 1);
// }

macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

type N2 = Tuple!(i32, i32);

macro_rules! const_maker {
    ($t:ty, $v:tt) => { const CONST: $t = $v; };
}
trait T {
    const_maker!{i32, 7}
}

macro_rules! example {
    () => { println!("Macro call in a macro!"); };
}

#[macro_export]
macro_rules! contract {

    /*

        contract!{

            NftContract, //// name of the contract
            "wildonion.near", //// the contract owner
            /////////////////////
            //// contract fields
            /////////////////////
            [
                contract_owner: AccountId, 
                deposit_by_owner: HashMap<AccountId, near_sdk::json_types::U128>, 
                contract_balance: near_sdk::json_types::U128
            ]; //// fields
            /////////////////////
            //// contract methods
            /////////////////////
            [ 
                "init" => [ //// array of init methods
                    pub fn init_contract(){
            
                    }
                ],
                "private" => [ //// array of private methods
                    pub fn get_all_deposits(){

                    }
                ],
                "payable" => [ //// array of payable methods
                    pub fn deposit(){
            
                    }
                ],
                "external" => [ //// array of external methods
                    fn get_address_bytes(){

                    }
                ]
            ]

        }

    */

    // event!{
    //     name: "list_owner",
    //     log: [NewOwner, AddDeposit],

    //     // event methods

    //     fn add_owner(){

    //     } 

    //     fn add_deposit(){
            
    //     }
    // }

    // emit!{
    //     event_name
    // }

    (
     $name:ident, $signer:expr, //// ident can be used to pass struct
     [$($fields:ident: $type:ty),*]; 
     [$($method_type:expr => [$($method:item),*]),* ]
    ) 
     
     => {
            #[near_bindgen]
            #[derive(serde::Deserialize, serde::Serialize)]
            pub struct $name{
                $($fields: $type),*
            }

            impl $name{
                        
                // https://stackoverflow.com/questions/64790850/how-do-i-write-a-macro-that-returns-the-implemented-method-of-a-struct-based-on
                // implement methods here 
                // ...
            }
    }
}