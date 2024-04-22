//! This module provides the logic for the quotation macros, which
//! allow for quoting F*/Coq/... code directly from Rust.
//!
//! In a F*/Coq/... quote, one can write antiquotations, that is,
//! embedded Rust snippets. The syntax is `$<PREFIX><PAYLOAD>`. The
//! payload `<PAYLOAD>` should be a Rust path, or a group with
//! arbitrary contents `{...contents...}`.
//!
//! The `<PREFIX>` describes the kind of the antiquotation:
//!  - empty prefix, the antiquotation is an expression;
//!  - `?`, the antiquotation is a pattern;
//!  - `$`, the antiquotation is a constructor name.

use crate::prelude::*;
use quote::ToTokens;

/// Marker that indicates a place where a antiquotation will be inserted
const SPLIT_MARK: &str = "SPLIT_QUOTE";

/// The different kinds of antiquotations
enum AntiquoteKind {
    Expr,
    Constructor,
    Pat,
}

impl ToTokens for AntiquoteKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(
            [match self {
                Self::Expr => quote! {_expr},
                Self::Constructor => quote! {_constructor},
                Self::Pat => quote! {_pat},
            }]
            .into_iter(),
        )
    }
}

/// An antiquotation
struct Antiquote {
    ts: pm::TokenStream,
    kind: AntiquoteKind,
}

impl ToTokens for Antiquote {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ts = TokenStream::from(self.ts.clone());
        fn wrap_pattern(pat: TokenStream) -> TokenStream {
            quote! {{#[allow(unreachable_code)]
                 match None { Some(#pat) => (), _ => () }
            }}
        }
        let ts = match self.kind {
            AntiquoteKind::Expr => ts,
            AntiquoteKind::Constructor => wrap_pattern(quote! {#ts {..}}),
            AntiquoteKind::Pat => wrap_pattern(ts),
        };
        tokens.extend([ts].into_iter())
    }
}

/// Extract antiquotations (`$[?][$]...`, `$[?][$]{...}`) and parses them.
fn process_string(s: &str) -> std::result::Result<(String, Vec<Antiquote>), String> {
    let mut chars = s.chars().peekable();
    let mut antiquotations = vec![];
    let mut output = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            '$' => {
                let mut s = String::new();
                let mut kind = AntiquoteKind::Expr;
                if let Some(prefix) = chars.next_if(|ch| *ch == '?' || *ch == '$') {
                    kind = match prefix {
                        '?' => AntiquoteKind::Pat,
                        '$' => AntiquoteKind::Constructor,
                        _ => unreachable!(),
                    };
                }
                // If the first character is `{`, we parse the block
                if let Some('{') = chars.peek() {
                    chars.next(); // Consume `{`
                    let mut level = 0;
                    while let Some(ch) = chars.next() {
                        level += match ch {
                            '{' => 1,
                            '}' => -1,
                            _ => 0,
                        };
                        if level < 0 {
                            break;
                        }
                        s.push(ch);
                    }
                } else {
                    while let Some(ch) =
                        chars.next_if(|ch| !matches!(ch, ' ' | '\t' | '\n' | '(' | '{' | ')'))
                    {
                        s.push(ch)
                    }
                }
                if s.is_empty() {
                    return Err(format!(
                        "Empty antiquotation just before `{}`",
                        chars.collect::<String>()
                    ));
                }
                output += SPLIT_MARK;
                // See https://github.com/rust-lang/rust/issues/58736
                let ts: std::result::Result<TokenStream, _> = syn::parse_str(&s)
                    .map_err(|err| format!("Could not parse antiquotation `{s}`: got error {err}"));
                if let Err(message) = &ts {
                    // If we don't panic, the error won't show up,
                    // this is because `parse_str` is not only
                    // panicking, but also makes rustc to exit earlier.
                    panic!("{message}");
                }
                let ts: pm::TokenStream = ts?.into();
                antiquotations.push(Antiquote { ts, kind })
            }
            _ => output.push(ch),
        }
    }
    Ok((output, antiquotations))
}

pub(super) fn item(
    kind: ItemQuote,
    attribute_to_inject: TokenStream,
    payload: pm::TokenStream,
    item: pm::TokenStream,
) -> pm::TokenStream {
    let expr = TokenStream::from(expression(payload));
    let item = TokenStream::from(item);
    let uid = ItemUid::fresh();
    let uid_attr = AttrPayload::Uid(uid.clone());
    let assoc_attr = AttrPayload::AssociatedItem {
        role: AssociationRole::ItemQuote,
        item: uid,
    };
    let kind_attr = AttrPayload::ItemQuote(kind);
    let status_attr = AttrPayload::ItemStatus(ItemStatus::Included { late_skip: true });
    use AttrPayload::NeverDropBody;
    quote! {
        #assoc_attr
        #item
        #attribute_to_inject
        const _: () = {
            #NeverDropBody
            #uid_attr
            #status_attr
            #kind_attr
            fn quote_contents() {
                #expr
            }
        };
    }
    .into()
}

pub(super) fn expression(payload: pm::TokenStream) -> pm::TokenStream {
    let (mut backend_code, antiquotes) = {
        let payload = parse_macro_input!(payload as LitStr).value();
        if payload.find(SPLIT_MARK).is_some() {
            return quote! {std::compile_error!(std::concat!($SPLIT_MARK, " is reserved"))}.into();
        }
        let (string, antiquotes) = match process_string(&payload) {
            Ok(x) => x,
            Err(message) => return quote! {std::compile_error!(#message)}.into(),
        };
        let string = proc_macro2::Literal::string(&string);
        let string: TokenStream = [proc_macro2::TokenTree::Literal(string)]
            .into_iter()
            .collect();
        (quote! {#string}, antiquotes)
    };

    for user in antiquotes.iter().rev() {
        let kind = &user.kind;
        backend_code = quote! {
            let #kind = #user;
            #backend_code
        };
    }

    quote! {hax_lib::inline(#[allow(unused_variables)]{#backend_code})}.into()
}
