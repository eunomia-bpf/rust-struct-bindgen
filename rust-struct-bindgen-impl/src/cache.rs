use std::collections::HashMap;

use btf::types::Btf;

pub(crate) struct SizeResolveCache<'a> {
    btf: &'a Btf<'a>,
    cache: HashMap<u32, usize>,
}

impl<'a> SizeResolveCache<'a> {
    pub(crate) fn new(btf: &'a Btf<'a>) -> Self {
        Self {
            btf,
            cache: HashMap::new(),
        }
    }
    pub(crate) fn resolve(&mut self, ty: u32) -> usize {
        *self
            .cache
            .entry(ty)
            .or_insert_with(|| self.btf.get_size_of(ty) as _)
    }
}
