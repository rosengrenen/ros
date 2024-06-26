use alloc::collections::HashMap;
use alloc::vec::Vec;
#[allow(deprecated)]
use core::alloc::Allocator;
#[allow(deprecated)]
use core::hash::BuildHasherDefault;
#[allow(deprecated)]
use core::hash::SipHasher;
#[allow(deprecated)]
use core::marker::PhantomData;

use super::name::DualNamePath;
use super::name::NamePath;
use super::name::NameSeg;
use super::name::NullName;
use crate::aml::name::MultiNamePath;
use crate::aml::name::NameString;

pub struct Context<A: Allocator> {
    pub root_scope: Scope<A>,
    pub current_scope: Vec<ScopePath<A>, A>,
    alloc: A,
}

impl<A: Allocator> core::fmt::Debug for Context<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Context")
            .field("scopes", &self.root_scope)
            .field("current_scope", &self.current_scope)
            .finish()
    }
}

impl<A: Allocator + Clone> Context<A> {
    pub fn new(alloc: A) -> Self {
        Self {
            root_scope: Scope::new(alloc.clone()),
            current_scope: alloc::vec![alloc.clone(); ScopePath::new(alloc.clone())],
            alloc,
        }
    }

    pub(crate) fn push_scope(&mut self, name: &NameString<A>) {
        let new_scope = match self.current_scope.last() {
            Some(current_scope) => current_scope.clone().add_name_string(name),
            None => ScopePath::from_name_string(name, self.alloc.clone()),
        };
        self.current_scope.push(new_scope).unwrap();
    }

    pub(crate) fn pop_scope(&mut self) {
        self.current_scope.pop().unwrap().unwrap();
    }

    pub(crate) fn add_method(&mut self, method: &NameString<A>, args: usize) {
        let (scope, name) = method.split();
        let scope = self.current_scope_path().clone().add_name_string(&scope);
        self.get_or_create_scope_mut(&scope)
            .methods
            .insert(name, args);
    }

    pub(crate) fn method_args(&mut self, method: &NameString<A>) -> Option<usize> {
        let (scope, name) = method.split();
        let mut search_scope = self.current_scope_path().clone().add_name_string(&scope);
        while !search_scope.segments.is_empty() {
            if let Some(scope) = self.get_scope(&search_scope) {
                if let Some(args) = scope.methods.get(&name) {
                    return Some(*args);
                }
            }

            search_scope.segments.pop().unwrap().unwrap();
        }

        None
    }

    fn get_or_create_scope_mut(&mut self, path: &ScopePath<A>) -> &mut Scope<A> {
        let mut scope = &mut self.root_scope;
        for seg in &path.segments {
            scope = scope
                .scopes
                .entry(*seg)
                .or_insert(Scope::new(self.alloc.clone()))
        }

        scope
    }

    fn get_scope(&self, path: &ScopePath<A>) -> Option<&Scope<A>> {
        let mut scope = &self.root_scope;
        for seg in &path.segments {
            scope = match scope.scopes.get(seg) {
                Some(scope) => scope,
                None => return None,
            };
        }

        Some(scope)
    }

    fn current_scope_path(&self) -> &ScopePath<A> {
        self.current_scope.last().unwrap()
    }
}

pub struct Scope<A: Allocator> {
    #[allow(deprecated)]
    pub scopes: HashMap<NameSeg, Scope<A>, BuildHasherDefault<SipHasher>, A>,
    #[allow(deprecated)]
    pub methods: HashMap<NameSeg, usize, BuildHasherDefault<SipHasher>, A>,
    alloc: PhantomData<A>,
}

impl<A: Allocator> core::fmt::Debug for Scope<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Scope")
            .field("scopes", &self.scopes)
            .field("methods", &self.methods)
            .finish()
    }
}

impl<A: Allocator + Clone> Scope<A> {
    fn new(alloc: A) -> Self {
        Self {
            scopes: HashMap::new(BuildHasherDefault::default(), alloc.clone()),
            methods: HashMap::new(BuildHasherDefault::default(), alloc),
            alloc: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct ScopePath<A: Allocator> {
    pub segments: Vec<NameSeg, A>,
}

impl<A: Allocator> core::fmt::Debug for ScopePath<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ScopePath")
            .field("segments", &self.segments)
            .finish()
    }
}

impl<A: Allocator + Clone> ScopePath<A> {
    fn new(alloc: A) -> Self {
        Self {
            segments: Vec::new(alloc),
        }
    }

    fn from_name_string(name_string: &NameString<A>, alloc: A) -> Self {
        Self::new(alloc).add_name_string(name_string)
    }

    fn add_name_string(mut self, name_string: &NameString<A>) -> Self {
        let name_path = match name_string {
            NameString::Absolute(name_path) => {
                self.segments.clear();
                name_path
            }
            NameString::Relative(prefix, name_path) => {
                for _ in 0..*prefix {
                    self.segments.pop().unwrap();
                }

                name_path
            }
        };

        match name_path {
            NamePath::NameSeg(seg) => {
                self.segments.push(*seg).unwrap();
            }
            NamePath::DualNamePath(DualNamePath { first, second }) => {
                self.segments.push(*first).unwrap();
                self.segments.push(*second).unwrap();
            }
            NamePath::MultiNamePath(MultiNamePath(segments)) => {
                for segment in segments {
                    self.segments.push(*segment).unwrap();
                }
            }
            NamePath::NullName(_) => (),
        };

        self
    }
}

impl<A: Allocator + Clone> NameString<A> {
    fn split(&self) -> (Self, NameSeg) {
        match self {
            Self::Absolute(path) => {
                let (path, seg) = path.split();
                (Self::Absolute(path), seg)
            }
            Self::Relative(prefix, path) => {
                let (path, seg) = path.split();
                (Self::Relative(*prefix, path), seg)
            }
        }
    }
}

impl<A: Allocator + Clone> NamePath<A> {
    fn split(&self) -> (Self, NameSeg) {
        match self {
            Self::NameSeg(seg) => (Self::NullName(NullName), *seg),
            Self::DualNamePath(DualNamePath { first, second }) => (Self::NameSeg(*first), *second),
            Self::MultiNamePath(MultiNamePath(segments)) => {
                let mut segments = segments.clone();
                let segment = segments.pop().unwrap().unwrap();
                (Self::MultiNamePath(MultiNamePath(segments)), segment)
            }
            Self::NullName(_) => {
                unimplemented!()
            }
        }
    }
}
