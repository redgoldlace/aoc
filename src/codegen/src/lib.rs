use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Paren,
    GenericParam, ItemFn, Lifetime, LifetimeDef, LitInt, PatType, Path, ReturnType, Token, Type,
    TypePath, TypeReference, TypeTuple,
};

#[proc_macro]
pub fn aoc(input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(input as AocInput);
    let aoc_day = attr.day;
    let aoc_part = attr.part;

    let tokens = quote! {
        use crate::solution::Solution;

        impl<'a> crate::solution::Solution<'a> for crate::day!( #aoc_day part #aoc_part ) {
            type Transformed = ___Transformed<'a>;
            type Result = ___Result<'a>;

            fn transform(input: &'a str) -> Self::Transformed {
                ___transform(input)
            }

            fn solve(input: Self::Transformed) -> Self::Result {
                ___solve(input)
            }
        }
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn transform(_: TokenStream, item: TokenStream) -> TokenStream {
    process_fn(parse_macro_input!(item as ItemFn), Part::Transform)
}

#[proc_macro_attribute]
pub fn solve(_: TokenStream, item: TokenStream) -> TokenStream {
    process_fn(parse_macro_input!(item as ItemFn), Part::Solve)
}

fn process_fn(func: ItemFn, part: Part) -> TokenStream {
    let mut func = func;

    let arg = match first_arg(&mut func) {
        Ok(arg) => arg,
        Err(error) => return error.to_compile_error().into(),
    };

    arg.ty = Box::new(part.fn_input());

    let output = match func.sig.output {
        ReturnType::Default => Type::Tuple(TypeTuple {
            paren_token: Paren(Span::mixed_site()),
            elems: Punctuated::new(),
        }),
        ReturnType::Type(_, ref ty) => (**ty).clone(),
    };

    let ty = Type::Path(TypePath {
        qself: None,
        path: Path::from(Ident::new(part.ty_path(), Span::call_site())),
    });

    func.sig
        .generics
        .params
        .push(GenericParam::Lifetime(LifetimeDef {
            attrs: Default::default(),
            lifetime: Lifetime {
                apostrophe: Span::mixed_site(),
                ident: Ident::new("a", Span::mixed_site()),
            },
            colon_token: None,
            bounds: Punctuated::new(),
        }));

    func.sig.ident = Ident::new(part.fn_name(), func.sig.ident.span());
    func.attrs
        .retain(|attr| !attr.path.is_ident(part.attr_name()));

    let span = Span::mixed_site();

    let tokens = quote_spanned! {span=>
        type #ty<'a> = #output;

        #func
    };

    tokens.into()
}

enum Part {
    Transform,
    Solve,
}

impl Part {
    pub fn attr_name(&self) -> &'static str {
        match self {
            Part::Transform => "transform",
            Part::Solve => "solve",
        }
    }

    pub fn fn_name(&self) -> &'static str {
        match self {
            Part::Transform => "___transform",
            Part::Solve => "___solve",
        }
    }

    pub fn ty_path(&self) -> &'static str {
        match self {
            Part::Transform => "___Transformed",
            Part::Solve => "___Result",
        }
    }

    pub fn fn_input(&self) -> Type {
        match self {
            Part::Transform => Type::Reference(TypeReference {
                and_token: Token![&](Span::mixed_site()),
                lifetime: Some(Lifetime {
                    apostrophe: Span::mixed_site(),
                    ident: Ident::new("a", Span::mixed_site()),
                }),
                mutability: None,
                elem: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Ident::new("str", Span::mixed_site()).into(),
                })),
            }),
            Part::Solve => Type::Path(TypePath {
                qself: None,
                path: Ident::new("___Transformed", Span::mixed_site()).into(),
            }),
        }
    }
}

fn first_arg(func: &mut ItemFn) -> syn::Result<&mut PatType> {
    if func.sig.inputs.len() != 1 {
        return Err(syn::Error::new(
            func.sig.ident.span(),
            "this function must only take one argument",
        ));
    }

    match func.sig.inputs.first_mut().unwrap() {
        syn::FnArg::Receiver(receiver) => Err(syn::Error::new(
            receiver.self_token.span,
            "`self` not allowed here",
        )),
        syn::FnArg::Typed(typed) => Ok(typed),
    }
}

mod keywords {
    syn::custom_keyword!(day);
    syn::custom_keyword!(part);
}

struct AocInput {
    day_kw: keywords::day,
    eq: Token![=],
    day: LitInt,
    comma: Token![,],
    part_kw: keywords::part,
    eq_2: Token![=],
    part: LitInt,
    comma_2: Option<Token![,]>,
}

impl Parse for AocInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AocInput {
            day_kw: input.parse()?,
            eq: input.parse()?,
            day: input.parse()?,
            comma: input.parse()?,
            part_kw: input.parse()?,
            eq_2: input.parse()?,
            part: input.parse()?,
            comma_2: input.parse().ok(),
        })
    }
}

impl ToTokens for AocInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.day_kw.to_tokens(tokens);
        self.eq.to_tokens(tokens);
        self.day.to_tokens(tokens);
        self.comma.to_tokens(tokens);
        self.part_kw.to_tokens(tokens);
        self.eq_2.to_tokens(tokens);
        self.part.to_tokens(tokens);
        self.comma_2.to_tokens(tokens);
    }
}
