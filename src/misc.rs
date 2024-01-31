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

/*  > -------------------------------------------
    |           proc macro functions 
    | ------------------------------------------
    |
    |   RUST CODES ---> TOKEN STREAM ---> AST
    |
    | 0 - compiler generates TokenStreams of Rust codes that this proc macro is placed of top of
    | 1 - parse (a new parser perhaps!) TokenStreams (Rust codes) to generate AST using syn
    | 2 - write new Rust codes using the patterns inside the generated AST like mutating idents or variables
    | 3 - convert generated or mutated either pure Rust codes in step 2 into a new AST using quote
    | 4 - return the new AST as a new TokenStream to the compiler to update the method or struct field at compile time
    |

    _> the input to all methods is of type TokenStream which is the extracted tokens
       of the actual Rust codes that can be used to build the AST later by compiler

    https://veykril.github.io/tlborm/introduction.html
    https://blog.logrocket.com/procedural-macros-in-rust/
    https://danielkeep.github.io/tlborm/book/README.html


    since macro processing in Rust happens after the construction of the AST, as such, 
    the syntax used to invoke a macro must be a proper part of the language's syntax 
    tree thus by adding a new code in this crate the compiler needs to compile the whole 
    things again, which forces us to reload the workspace everytime, means by that any 
    logging codes don't work in here at runtime and we must check them in console once 
    the code gets compiled.

    a TokenStream is simply built from the Rust codes which can be used to built the
    AST like: RUST CODES ---> TOKEN STREAM ---> AST also the following are matters:
    sync generates : the AST from the passed in TokenStream (sequence of token trees)
    quote generates: Rust codes that can be used to generate TokenStream and a new AST

    proc macro can be on top of methods, union, enum and struct and can be used to add 
    method to them before they get compiled since compiler will extend the struct AST 
    by doing this once we get the token stream of the struct Rust code. it can be used 
    to parse it into Rust pattern (ident, ty, tt and ...) that will be used to add a new 
    or edit a logic on them finally we must use the extended token stream of the Rust codes 
    that we've added to convert them into a new token stream to return from the macro to 
    tell the compiler that extend the old AST with this new one

    kinds: 
        decl_macro
        proc_macro
        proc_macro_derive
        proc_macro_attribute
    
    benefits:
        add a method to struct or check a condition against its fields
        convert trait into module to extend the trait methods
        extend the interface of a struct by changing the behaviour of its fields and methods
        create a DSL like jsx, css or a new keyword or a new lang
        build a new AST from the input TokenStream by parsing incoming tokens and return the generated TokenStream from a new Rust codes
        write parser using decl_macro
        changing and analysing the AST logics of methods at compile time before getting into their body
        bind rust code to other langs and extending code in rust using macros
        extend the user code by adding some code into his already coded logic at compile time
    

*/



// use proc_macro::TokenStream;
// use quote::{quote, ToTokens};
// use std::collections::HashSet as Set;
// use syn::parse::{Parse, ParseStream};
// use syn::punctuated::Punctuated;
// use syn::{parse_macro_input, parse_quote, Expr, Ident, Local, Pat, Stmt, Token, FnArg};



// struct Args{
//     vars: Set<Ident>
// }

// /*
//     we need to create our own parser to parse the 
//     args token stream into a new AST
// */
// impl Parse for Args {
//     fn parse(input: ParseStream) -> syn::parse::Result<Self> {
//         let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
//         Ok(Args {
//             vars: vars.into_iter().collect(),
//         })
//     }
// }

// /*
//     with the following proc macro we can do inspect and operate on the 
//     api methods before generating the output or executing any extra
//     logics before getting into the api body like actix #[get()] which
//     checks the request path in the first place before sliding into 
//     the request body, also to get the Rust token codes from TokenStream 
//     we must use syn::parse and to get the TokenStream from Rust codes 
//     we msut use quote
// */
// #[proc_macro_attribute]
// pub fn passport(args: TokenStream, input: TokenStream) -> TokenStream {

//     /*  
//         build the new AST from the `input` TokenStream to extend the one that we 
//         already have by using syn to parse the args & input tokens into a syntax 
//         tree, note that the type of TokenStream that we want to parse it with syn 
//         to generate AST, must be specified, like parsing a function TokenStream 
//         into ItemFn AST, then we need to generate a new TokenStream from generated 
//         Rust types parsed from the input TokenStream, using quote to do so, generate 
//         a new TokenStream from the passed in Rust codes (either pure or using #variable) 
//         to it that can be used to build a new AST, this will replace whatever `input` 
//         is annotated with this attribute proc macro, finally we'll return the token 
//         stream either generated by the quote or the passed in input.

//         when we are defining a procedural macro, we're not actually interacting with 
//         the runtime data, instead, we're generating code that will be inserted into 
//         the function thus we can't access the token inside the request object in this 
//         proc macro since procedural macros work at compile time, they don't have access 
//         to runtime data, in our case, the token in the HTTP request header is available 
//         at runtime, so it's impossible to directly inspect the header's content inside
//         a procedural macro.
//     */
//     let mut api_ast = syn::parse::<syn::ItemFn>(input.clone()).unwrap(); /* parsing the input token stream or the method into the ItemFn AST */
//     let roles_set = parse_macro_input!(args as Args).vars; /* casting the args TokenStream into the Args parser */
//     let mut granted_roles = vec![];
//     for role in roles_set{
//         granted_roles.push(role.to_string()); /* converting the Ident into String */
//     }

//     /*  
//         every variable can be shown as ident in Rust thus if we wanna have a new variable we must 
//         create new ident instance, like the following for the request object, also every token 
//         in a TokenStream has an associated Span holding some additional info, a span, is a region 
//         of source code, along with macro expansion information, it points into a region of the 
//         original source code(important for displaying diagnostics at the correct places) as well 
//         as holding the kind of hygiene for this location. The hygiene is relevant mainly for 
//         identifiers, as it allows or forbids the identifier from referencing things or being 
//         referenced by things defined outside of the invocation.
//     */
//     let mut req_ident = syn::Ident::new("req", proc_macro2::Span::call_site());
//     for input in api_ast.clone().sig.inputs{
//         if let FnArg::Typed(pat_type) = input{
//             if let Pat::Ident(pat_ident) = *pat_type.pat{
//                 if pat_ident.ident.to_string() == "req".to_string(){
//                     req_ident = pat_ident.ident;
//                     break;
//                 }
//             }
//         }
//     }

//     /* 
//         generating a token stream from granted_roles variable, 
//         quote generates new AST or token stream from Rust codes
//         that can be returned to the proc macro caller since quote
//         generates a token stream which is the return type of this
//         proc macro methods
//     */
//     let new_stmt = syn::parse2(
//         quote!{ /* building new token stream from the Rust token codes */
            
//             /* 
//                 granted_roles can be accessible inside the api body at runtime, 
//                 vec![#(#granted_roles),*] means that we're pushing all the roles
//                 inside a vec![] and since there are multiple roles we used * to 
//                 push them all into the vec![] which means repetition pattern
//             */
//             let granted_roles = vec![#(#granted_roles),*]; // extending the AST of the api method at compile time

//         }
//     ).unwrap();

//     /* injecting the granted_roles into the api body at compile time */
//     api_ast.block.stmts.insert(0, new_stmt);
    
//     /* 
//         returning the newly generated AST by the quote of the input api Rust code  
//         which contains the updated and compiled codes of the function body, at this
//         stage we're building new token stream from the updated api_ast Rust token codes
//     */
//     TokenStream::from(quote!(#api_ast))


// }


// #[proc_macro]
// pub fn passport_proc(input: TokenStream) -> TokenStream {

//     // ex:
//     // #[passport_proc]
//     // #[passport_proc(access=all)]
//     // #[passport_proc(access=user)]
//     // #[passport_proc(access=admin)]
//     // #[passport_proc(access=dev)]
//     // fn im_a_method(){}
    
//     input

// }

// #[proc_macro_derive(Passport)]
// pub fn derive_proc_macro(input: TokenStream) -> TokenStream {

//     // ex:
//     // #[derive(Passport)]
//     // struct SexyStruct{}
    
//     // this will be implemented in here for the struct inside input token stream
//     // so later on we can call the method on the struct once we implement the
//     // method for the struct in here.
//     // SexyStruct::passport() // like checking jwt in request object

//     input

// }