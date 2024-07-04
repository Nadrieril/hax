use crate::prelude::*;

mod types {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::collections::HashSet;

    pub struct LocalContextS {
        pub vars: HashMap<rustc_middle::thir::LocalVarId, String>,
    }

    impl LocalContextS {
        pub fn new() -> LocalContextS {
            LocalContextS {
                vars: HashMap::new(),
            }
        }
    }

    #[derive(Clone)]
    pub struct Base<'tcx> {
        pub options: Rc<hax_frontend_exporter_options::Options>,
        pub macro_infos: MacroCalls,
        pub local_ctx: Rc<RefCell<LocalContextS>>,
        pub opt_def_id: Option<rustc_hir::def_id::DefId>,
        pub exported_spans: ExportedSpans,
        pub exported_def_ids: ExportedDefIds,
        pub cached_thirs: Rc<
            HashMap<
                rustc_span::def_id::LocalDefId,
                (
                    Rc<rustc_middle::thir::Thir<'tcx>>,
                    rustc_middle::thir::ExprId,
                ),
            >,
        >,
        pub tcx: rustc_middle::ty::TyCtxt<'tcx>,
        /// Rust doesn't enforce bounds on generic parameters in type
        /// aliases. Thus, when translating type aliases, we need to
        /// disable the resolution of implementation expressions. For
        /// more details, please see
        /// https://github.com/hacspec/hax/issues/707.
        pub ty_alias_mode: bool,
    }

    impl<'tcx> Base<'tcx> {
        pub fn new(
            tcx: rustc_middle::ty::TyCtxt<'tcx>,
            options: hax_frontend_exporter_options::Options,
        ) -> Self {
            Self {
                tcx,
                macro_infos: Rc::new(HashMap::new()),
                cached_thirs: Rc::new(HashMap::new()),
                options: Rc::new(options),
                // Always prefer `s.owner_id()` to `s.base().opt_def_id`.
                // `opt_def_id` is used in `utils` for error reporting
                opt_def_id: None,
                local_ctx: Rc::new(RefCell::new(LocalContextS::new())),
                exported_spans: Rc::new(RefCell::new(HashSet::new())),
                exported_def_ids: Rc::new(RefCell::new(HashSet::new())),
                ty_alias_mode: false,
            }
        }
    }

    pub type MacroCalls = Rc<HashMap<Span, Span>>;
    pub type ExportedSpans = Rc<RefCell<HashSet<rustc_span::Span>>>;
    pub type ExportedDefIds = Rc<RefCell<HashSet<rustc_hir::def_id::DefId>>>;
    pub type RcThir<'tcx> = Rc<rustc_middle::thir::Thir<'tcx>>;
    pub type RcMir<'tcx> = Rc<rustc_middle::mir::Body<'tcx>>;
}

#[derive(Clone)]
pub struct State<Base, Thir, Mir, OwnerId> {
    pub base: Base,
    pub thir: Thir,
    pub mir: Mir,
    pub owner_id: OwnerId,
}

pub trait HasBase<'tcx> {
    fn base(self: &Self) -> types::Base<'tcx>;
}
impl<'tcx, Thir, Mir, OwnerId> HasBase<'tcx> for State<types::Base<'tcx>, Thir, Mir, OwnerId> {
    fn base(self: &Self) -> types::Base<'tcx> {
        self.base.clone()
    }
}
pub trait HasBaseSetter<'tcx> {
    type Out;
    fn with_base(self: Self, base: types::Base<'tcx>) -> Self::Out;
}
#[allow(unused)]
impl<'tcx, Base, Thir, Mir, OwnerId> HasBaseSetter<'tcx> for State<Base, Thir, Mir, OwnerId> {
    type Out = State<types::Base<'tcx>, Thir, Mir, OwnerId>;
    fn with_base(self: Self, base: types::Base<'tcx>) -> Self::Out {
        let __this_field = base;
        let State {
            base,
            thir,
            mir,
            owner_id,
        } = self;
        let base = __this_field;
        State {
            base,
            thir,
            mir,
            owner_id,
        }
    }
}

pub trait HasThir<'tcx> {
    fn thir(self: &Self) -> types::RcThir<'tcx>;
}
impl<'tcx, Base, Mir, OwnerId> HasThir<'tcx> for State<Base, types::RcThir<'tcx>, Mir, OwnerId> {
    fn thir(self: &Self) -> types::RcThir<'tcx> {
        self.thir.clone()
    }
}
pub trait HasThirSetter<'tcx> {
    type Out;
    fn with_thir(self: Self, thir: types::RcThir<'tcx>) -> Self::Out;
}
#[allow(unused)]
impl<'tcx, Base, Thir, Mir, OwnerId> HasThirSetter<'tcx> for State<Base, Thir, Mir, OwnerId> {
    type Out = State<Base, types::RcThir<'tcx>, Mir, OwnerId>;
    fn with_thir(self: Self, thir: types::RcThir<'tcx>) -> Self::Out {
        let __this_field = thir;
        let State {
            base,
            thir,
            mir,
            owner_id,
        } = self;
        let thir = __this_field;
        State {
            base,
            thir,
            mir,
            owner_id,
        }
    }
}

pub trait HasMir<'tcx> {
    fn mir(self: &Self) -> types::RcMir<'tcx>;
}
impl<'tcx, Base, Thir, OwnerId> HasMir<'tcx> for State<Base, Thir, types::RcMir<'tcx>, OwnerId> {
    fn mir(self: &Self) -> types::RcMir<'tcx> {
        self.mir.clone()
    }
}
pub trait HasMirSetter<'tcx> {
    type Out;
    fn with_mir(self: Self, mir: types::RcMir<'tcx>) -> Self::Out;
}
#[allow(unused)]
impl<'tcx, Base, Thir, Mir, OwnerId> HasMirSetter<'tcx> for State<Base, Thir, Mir, OwnerId> {
    type Out = State<Base, Thir, types::RcMir<'tcx>, OwnerId>;
    fn with_mir(self: Self, mir: types::RcMir<'tcx>) -> Self::Out {
        let __this_field = mir;
        let State {
            base,
            thir,
            mir,
            owner_id,
        } = self;
        let mir = __this_field;
        State {
            base,
            thir,
            mir,
            owner_id,
        }
    }
}

pub trait HasOwnerId {
    fn owner_id(self: &Self) -> rustc_hir::def_id::DefId;
}
impl<Base, Thir, Mir> HasOwnerId for State<Base, Thir, Mir, rustc_hir::def_id::DefId> {
    fn owner_id(self: &Self) -> rustc_hir::def_id::DefId {
        self.owner_id.clone()
    }
}
pub trait HasOwnerIdSetter {
    type Out;
    fn with_owner_id(self: Self, owner_id: rustc_hir::def_id::DefId) -> Self::Out;
}
#[allow(unused)]
impl<Base, Thir, Mir, OwnerId> HasOwnerIdSetter for State<Base, Thir, Mir, OwnerId> {
    type Out = State<Base, Thir, Mir, rustc_hir::def_id::DefId>;
    fn with_owner_id(self: Self, owner_id: rustc_hir::def_id::DefId) -> Self::Out {
        let __this_field = owner_id;
        let State {
            base,
            thir,
            mir,
            owner_id,
        } = self;
        let owner_id = __this_field;
        State {
            base,
            thir,
            mir,
            owner_id,
        }
    }
}

pub trait IsState<'tcx> =
    HasBaseSetter<'tcx> + HasThirSetter<'tcx> + HasMirSetter<'tcx> + HasOwnerIdSetter;

pub use types::*;

impl<'tcx> State<Base<'tcx>, (), (), ()> {
    pub fn new(
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        options: hax_frontend_exporter_options::Options,
    ) -> Self {
        Self {
            thir: (),
            mir: (),
            owner_id: (),
            base: Base::new(tcx, options),
        }
    }
}

impl<'tcx> State<Base<'tcx>, (), (), rustc_hir::def_id::DefId> {
    pub fn new_from_state_and_id<S: BaseState<'tcx>>(s: &S, id: rustc_hir::def_id::DefId) -> Self {
        State {
            thir: (),
            mir: (),
            owner_id: id,
            base: s.base().clone(),
        }
    }
}
impl<'tcx> State<Base<'tcx>, (), Rc<rustc_middle::mir::Body<'tcx>>, rustc_hir::def_id::DefId> {
    pub fn new_from_mir(
        tcx: rustc_middle::ty::TyCtxt<'tcx>,
        options: hax_frontend_exporter_options::Options,
        mir: rustc_middle::mir::Body<'tcx>,
        owner_id: rustc_hir::def_id::DefId,
    ) -> Self {
        Self {
            thir: (),
            mir: Rc::new(mir),
            owner_id,
            base: Base::new(tcx, options),
        }
    }
}

/// Updates the OnwerId in a state, making sure to override `opt_def_id` in base as well.
pub fn with_owner_id<'tcx, THIR, MIR>(
    mut base: types::Base<'tcx>,
    thir: THIR,
    mir: MIR,
    owner_id: rustc_hir::def_id::DefId,
) -> State<types::Base<'tcx>, THIR, MIR, rustc_hir::def_id::DefId> {
    base.opt_def_id = Some(owner_id);
    State {
        thir,
        owner_id,
        base,
        mir,
    }
}

pub trait BaseState<'tcx> = HasBase<'tcx> + Clone + IsState<'tcx>;
/// State of anything below a `owner_id`
pub trait UnderOwnerState<'tcx> = BaseState<'tcx> + HasOwnerId;

/// Meta-informations about an `impl<GENERICS[: PREDICATES]> TRAIT for
/// TYPE where PREDICATES {}`
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ImplInfos {
    pub generics: TyGenerics,
    pub clauses: Vec<(Clause, Span)>,
    pub typ: Ty,
    pub trait_ref: Option<TraitRef>,
}

impl ImplInfos {
    fn from<'tcx>(base: Base<'tcx>, did: rustc_hir::def_id::DefId) -> Self {
        let tcx = base.tcx;
        let s = &with_owner_id(base, (), (), did);

        Self {
            generics: tcx.generics_of(did).sinto(s),
            typ: tcx.type_of(did).instantiate_identity().sinto(s),
            trait_ref: tcx.impl_trait_ref(did).sinto(s),
            clauses: tcx.predicates_defined_on(did).predicates.sinto(s),
        }
    }
}

/// Returns a map from every implementation (`Impl`) `DefId`s to the
/// type they implement, plus the bounds.
pub fn impl_def_ids_to_impled_types_and_bounds<'tcx, S: BaseState<'tcx>>(
    s: &S,
) -> HashMap<DefId, ImplInfos> {
    let Base {
        tcx,
        exported_def_ids,
        ..
    } = s.base();

    let def_ids = exported_def_ids.as_ref().borrow().clone();
    let with_parents = |mut did: rustc_hir::def_id::DefId| {
        let mut acc = vec![did.clone()];
        while let Some(parent) = tcx.opt_parent(did) {
            did = parent;
            acc.push(did);
        }
        acc.into_iter()
    };
    use itertools::Itertools;
    def_ids
        .iter()
        .cloned()
        .map(with_parents)
        .flatten()
        .unique()
        .filter(|&did| {
            // keep only DefIds that corresponds to implementations
            matches!(
                tcx.def_path(did).data.last(),
                Some(rustc_hir::definitions::DisambiguatedDefPathData {
                    data: rustc_hir::definitions::DefPathData::Impl,
                    ..
                })
            )
        })
        .map(|did| (did.sinto(s), ImplInfos::from(s.base(), did)))
        .collect()
}
