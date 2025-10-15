use std::{
    cmp::PartialEq,
    convert::AsRef,
    slice::from_raw_parts,
    fmt::{Debug, Display, Error as FmtError, Formatter, Result as FmtResult, Write},
    mem::{MaybeUninit, transmute},
    ops::Deref,
    ops::{Bound, RangeBounds},
    result::Result,
    str::{self, from_utf8_unchecked},
};

#[cfg(test)]
#[allow(non_camel_case_types)]
pub trait yGeneric<'y, const C: usize> {
    fn iden<'b>(self: &'b Self) -> yPattern<'y, C>
    where
        'y: 'b;
}

#[cfg(test)]
#[allow(non_camel_case_types)]
pub trait yTrait {
    type Ygn;
    fn to_yangon(self: &Self) -> Self::Ygn;
}

#[cfg(test)]
#[allow(non_camel_case_types)]
pub enum yPattern<'y, const C: usize> {
    Slice(&'y str),
    Char(char),
    CharSlice(&'y [char; C]),
    Closure(fn(char) -> bool),
}

#[cfg(test)]
#[allow(non_camel_case_types)]
pub enum yError {
    FromUtf8Error,
    CapacityOverflow,
}

#[cfg(test)]
#[allow(non_camel_case_types)]
pub enum yCow<'c, X> {
    Borrowed(&'c str),
    Owned(X),
}

#[cfg(test)]
#[derive(Clone)]
pub struct Yangon<const N: usize = 10240> {
    list: [MaybeUninit<u8>; N],
    len: usize,
    capacity: usize,
}


#[cfg(test)]
#[allow(warnings)]
impl<const N: usize> Yangon<N> {
    pub fn push_str(self: &mut Self, slice: &str) -> Result<(), yError> {
        let mut len: usize = (*self).len;
        if slice.len() + len > (*self).capacity {
            Err(yError::CapacityOverflow)
        } else {
            let ptr: *mut u8 = (*self).list[0].as_mut_ptr();
            for &x in slice.as_bytes() {
                unsafe {
                    *ptr.add(len) = x;
                }
                len += 1;
            }
            (*self).len = len;
            Ok(())
        }
    }

    pub unsafe fn push_str_unchecked(self: &mut Self, slice: &str) {
        let mut len: usize = (*self).len;
        let ptr: *mut u8 = (*self).list[0].as_mut_ptr();
        for &x in slice.as_bytes() {
            *ptr.add(len) = x;
            len += 1;
        }
        (*self).len = len;
    }

    #[inline]
    pub fn with_capacity() -> Self {
        Self {
            list: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
            capacity: N,
        }
    }

    #[inline]
    pub fn capacity(self: &Self) -> usize {
        (*self).capacity
    }

    #[inline]
    pub fn shrink_to_fit(self: &mut Self) {
        (*self).capacity = (*self).len + 1;
    }

    #[inline]
    pub fn shrink_to(self: &mut Self, shrk_to: usize) {
        if shrk_to > (*self).len && shrk_to < (*self).capacity {
            (*self).capacity = shrk_to;
        }
    }

    #[inline]
    pub fn as_ptr(self: &Self) -> *const u8 {
        (*self).list[0].as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(self: &mut Self) -> *mut u8 {
        (*self).list[0].as_mut_ptr()
    }

    pub fn to_string(self: &Self) -> String {
        let len: usize = (*self).len;
        let ptr: *const u8 = (*self).list[0].as_ptr();
        let mut string: String = String::with_capacity(len);
        let string_ptr: *mut u8 = string.as_mut_ptr();
        unsafe {
            for x in 0..len {
                *string_ptr.add(x) = *ptr.add(x);
            }
            string.as_mut_vec().set_len(len);
        }
        string
    }

    pub fn replace_range<R>(self: &mut Self, range: R, slice: &str)
    where
        R: RangeBounds<usize>,
    {
        let mut list_len: usize = (*self).len;
        let mut str_idx: usize = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&e) => e + 1,
        };
        let end_idx: usize = match range.end_bound() {
            Bound::Unbounded => list_len,
            Bound::Excluded(&e) => e,
            Bound::Included(&i) => i + 1,
        };
        let list: &mut [MaybeUninit<u8>] = &mut (*self).list;
        let slice_ptr: *const u8 = slice.as_ptr();
        let slice_len: usize = slice.len();
        let mut counter: usize = 0;
        if slice_len <= end_idx - str_idx {
            let mut times_to_loop: usize = 0;
            for x in str_idx..end_idx {
                if counter < slice_len {
                    unsafe {
                        *(*list)[x].as_mut_ptr() = *slice_ptr.add(counter);
                    }
                    counter += 1;
                } else {
                    unsafe {
                        *(*list)[x].as_mut_ptr() = 0;
                    }
                    times_to_loop += 1;
                }
            }
            let mut watch: usize = 0;
            while watch < times_to_loop {
                unsafe {
                    for x in 0..list_len {
                        if transmute::<MaybeUninit<u8>, u8>((*list)[x]) == 0 {
                            (*list)[x] = (*list)[x + 1];
                            *(*list)[x + 1].as_mut_ptr() = 0;
                        }
                    }
                    watch += 1;
                    list_len -= 1;
                }
            }
            (*self).len = list_len;
        } else {
            for x in 0..slice_len - (end_idx - str_idx) {
                unsafe {
                    *(*list)[list_len].as_mut_ptr() = 0;
                }
                list_len += 1;
            }
            loop {
                unsafe {
                    if transmute::<MaybeUninit<u8>, u8>((*list)[end_idx]) == 0 {
                        break;
                    } else {
                        for x in 0..list_len {
                            unsafe {
                                if transmute::<MaybeUninit<u8>, u8>((*list)[x]) == 0 {
                                    (*list)[x] = (*list)[x - 1];
                                    *(*list)[x - 1].as_mut_ptr() = 0;
                                }
                            }
                        }
                    }
                }
            }
            for &x in slice.as_bytes() {
                unsafe {
                    *(*list)[str_idx].as_mut_ptr() = x;
                }
                str_idx += 1;
            }
            (*self).len = list_len;
        }
    }

    #[inline]
    pub fn len(self: &Self) -> usize {
        (*self).len
    }

    pub fn pop(self: &mut Self) -> Option<char> {
        let len: usize = (*self).len;
        if len > 0 {
            let list: &[u8] = unsafe {
                &*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>((
                    (*self).list.as_ptr(),
                    len,
                ))
            };
            let mut end_idx: usize = len - 1;
            loop {
                match str::from_utf8(&(*list)[end_idx..len]) {
                    Ok(slice) => {
                        (*self).len -= len - end_idx;
                        return Some(slice.chars().next().unwrap());
                    }
                    Err(_) => {
                        if end_idx == 0 {
                            return None;
                        } else {
                            end_idx -= 1;
                        }
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn remove(self: &mut Self, mut idx: usize) -> char {
        let mut len: usize = (*self).len;
        let list: &mut [u8] =
            unsafe { &mut *transmute::<(*mut u8, usize), *mut [u8]>(((*self).as_mut_ptr(), len)) };
        let mut end_idx: usize = idx + 1;
        loop {
            if end_idx > len {
                panic!("Index is out of bound");
            }
            match str::from_utf8(&(*list)[idx..end_idx]) {
                Ok(slice) => {
                    let ch: char = slice.chars().next().unwrap();
                    let ptr: *mut u8 = (*self).as_mut_ptr();
                    let frt_ptr: *mut u8 = unsafe { ptr.add(idx) };
                    let lst_ptr: *mut u8 = unsafe { ptr.add(end_idx) };
                    for x in 0..len - end_idx {
                        unsafe {
                            *frt_ptr.add(x) = *lst_ptr.add(x);
                        }
                    }
                    (*self).len -= end_idx - idx;
                    return ch;
                }
                Err(_) => {
                    end_idx += 1;
                }
            }
        }
    }

    #[inline]
    pub fn clear(self: &mut Self) {
        (*self).len = 0;
    }

    #[inline]
    pub fn truncate(self: &mut Self, t_cate: usize) {
        if t_cate <= (*self).len {
            (*self).len = t_cate;
        }
    }

    pub fn push(self: &mut Self, ch: char) -> Result<(), yError> {
        let mut bind: [u8; 4] = [0, 0, 0, 0];
        let bytes: &[u8] = ch.encode_utf8(&mut bind).as_bytes();
        let mut len: usize = (*self).len;
        if bytes.len() + len > (*self).capacity {
            Err(yError::CapacityOverflow)
        } else {
            for &x in bytes {
                unsafe {
                    *(*self).list[len].as_mut_ptr() = x;
                }
                len += 1;
            }
            (*self).len = len;
            Ok(())
        }
    }

    pub fn from_utf8(vector: Vec<u8>) -> Result<Self, yError> {
        if str::from_utf8(&vector).is_ok() {
            let mut inst: Self = Self::with_capacity();
            let mut counter: usize = 0;
            for x in vector.into_iter() {
                unsafe {
                    *inst.list[counter].as_mut_ptr() = x;
                }
                counter += 1;
            }
            inst.len = counter;
            Ok(inst)
        } else {
            Err(yError::FromUtf8Error)
        }
    }

    #[inline]
    pub unsafe fn set_len(self: &mut Self, len: usize) {
        (*self).len = len;
    }

    #[inline]
    pub unsafe fn set_cap(self: &mut Self, cap: usize) {
        (*self).capacity = cap;
    }

    pub unsafe fn from_utf8_unchecked(vector: Vec<u8>) -> Self {
        let mut inst: Self = Self::with_capacity();
        for x in vector {
            *inst.list[inst.len].as_mut_ptr() = x;
            inst.len += 1;
        }
        inst
    }

    pub fn from_utf8_lossy<'b>(list_ref: &'b [u8]) -> yCow<'b, Self> {
        if str::from_utf8(list_ref).is_ok() {
            yCow::Borrowed(unsafe { from_utf8_unchecked(list_ref) })
        } else {
            let mut inst: Self = Self::with_capacity();
            let ptr: *mut u8 = inst.list[0].as_mut_ptr();
            let len: usize = list_ref.len();
            let mut srt_idx: usize = 0;
            let mut idx: usize = 0;
            loop {
                match str::from_utf8(&(*list_ref)[srt_idx..]) {
                    Ok(slice) => {
                        for &x in &(*list_ref)[srt_idx..] {
                            unsafe {
                                *ptr.add(idx) = x;
                            }
                            idx += 1;
                        }
                        break;
                    }
                    Err(e) => {
                        let err_srt: usize = e.valid_up_to();
                        for &x in &(*list_ref)[srt_idx..srt_idx + err_srt] {
                            unsafe {
                                *ptr.add(idx) = x;
                            }
                            idx += 1;
                        }
                        match e.error_len() {
                            Some(err_len) => {
                                for x in [0xEF, 0xBF, 0xBD] {
                                    unsafe {
                                        *ptr.add(idx) = x;
                                    }
                                    idx += 1;
                                }
                                srt_idx += err_srt + err_len;
                                if srt_idx >= len {
                                    break;
                                }
                            }
                            _ => {
                                for x in [0xEF, 0xBF, 0xBD] {
                                    unsafe {
                                        *ptr.add(idx) = x;
                                    }
                                    idx += 1;
                                }
                                break;
                            }
                        }
                    }
                }
            }
            inst.len = idx;
            yCow::Owned(inst)
        }
    }

    #[inline]
    pub fn is_empty(self: &Self) -> bool {
        (*self).len == 0
    }

    pub fn insert(self: &mut Self, mut idx: usize, ch: char) {
        let len: usize = (*self).len;
        let mut bind: [u8; 4] = [0, 0, 0, 0];
        let bytes: &[u8] = ch.encode_utf8(&mut bind).as_bytes();
        let byt_len: usize = bytes.len();
        if idx > len {
            panic!("Index out of bounds.");
        } else if len + byt_len > (*self).capacity {
            panic!("Capacity Overflow.")
        } else {
            let ptr: *mut u8 = (*self).list[0].as_mut_ptr();
            let jumps: usize = len - idx;
            let mut lst_idx: usize = len - 1 + byt_len;
            let mut edg_idx: usize = len - 1;
            for _ in 0..jumps {
                unsafe {
                    *ptr.add(lst_idx) = *ptr.add(edg_idx);
                }
                lst_idx -= 1;
                edg_idx -= 1;
            }
            for &x in bytes {
                unsafe {
                    *ptr.add(idx) = x;
                }
                idx += 1;
            }
        }
        (*self).len += byt_len;
    }

    pub fn retain<F>(self: &mut Self, mut closure: F)
    where
        F: FnMut(char) -> bool,
    {
        let mut list_len: usize = (*self).len;
        let list: &mut [u8] = unsafe {
            &mut *transmute::<(*mut u8, usize), *mut [u8]>(((*self).as_mut_ptr(), list_len))
        };
        let mut srt_idx: usize = 0;
        let mut end_idx: usize = 1;
        let mut zero: usize = 0;
        loop {
            if srt_idx >= list_len {
                break;
            }
            match str::from_utf8(&(*list)[srt_idx..end_idx]) {
                Ok(slice) => {
                    if !closure(slice.chars().next().unwrap()) {
                        let mut idx: usize = srt_idx;
                        for x in srt_idx..end_idx {
                            (*list)[idx] = 0;
                            zero += 1;
                            idx += 1;
                        }
                    }
                    srt_idx = end_idx;
                    end_idx += 1;
                }
                Err(_) => {
                    end_idx += 1;
                }
            }
        }
        let reality: usize = list_len - zero;
        let mut is_completed: usize = 0;
        loop {
            is_completed = 0;
            for x in 0..reality {
                if (*list)[x] != 0 {
                    is_completed += 1;
                }
            }
            if is_completed == reality {
                break;
            }
            for x in 0..list_len - 1 {
                if (*list)[x] == 0 {
                    (*list)[x] = (*list)[x + 1];
                    (*list)[x + 1] = 0;
                }
            }
        }
        (*self).len = reality;
    }

    pub fn split_off(self: &mut Self, spl_off: usize) -> Self {
        let list: &mut [u8] = unsafe {
            &mut *transmute::<(*mut MaybeUninit<u8>, usize), *mut [u8]>((
                (*self).list.as_mut_ptr(),
                (*self).len,
            ))
        };
        let len_cap: usize = (*list)[spl_off..].len();
        let mut idx: usize = 0;
        let mut inst: Self = Self::with_capacity();
        inst.len = len_cap;
        inst.capacity = len_cap;
        for &x in &(*list)[spl_off..] {
            unsafe {
                *inst.list[idx].as_mut_ptr() = x;
            }
            idx += 1;
        }
        (*self).len = spl_off;
        inst
    }

    #[inline]
    pub fn as_str(self: &Self) -> &str {
        unsafe {
            from_utf8_unchecked(&*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>(
                ((*self).list.as_ptr(), (*self).len),
            ))
        }
    }

    pub fn into_bytes(self: &Self) -> Vec<u8> {
        let len: usize = (*self).len;
        let mut list: Vec<u8> = Vec::with_capacity(len);
        let vec_ptr: *mut u8 = list.as_mut_ptr();
        let ptr: *const u8 = (*self).list[0].as_ptr();
        let mut idx: usize = 0;
        unsafe {
            for _ in 0..len {
                *vec_ptr.add(idx) = *ptr.add(idx);
                idx += 1;
            }
            list.set_len(len);
        }
        list
    }

    pub fn replace_it(self: &Self, slice: &str, upg: &str) -> Self {
        let upg_byt: &[u8] = upg.as_bytes();
        let upg_len: usize = upg_byt.len();
        let bytes: &[u8] = slice.as_bytes();
        let byt_len: usize = bytes.len();
        if byt_len == 0 && upg_len == 0 {
            return (*self).clone();
        }
        let mut inst: Yangon<N> = Self::with_capacity();
        let ist_ptr: *mut u8 = inst.list[0].as_mut_ptr();
        let ptr: *const u8 = (*self).list[0].as_ptr();
        let len: usize = (*self).len;
        let mut ist_idx: usize = 0;
        let mut idx: usize = 0;
        if byt_len == 0 && upg_len != 0 {
            let mut end_idx: usize = 1;
            let list: &[u8] = unsafe { &*transmute::<(*const u8, usize), *const [u8]>((ptr, len)) };
            for &x in upg_byt {
                unsafe {
                    *ist_ptr.add(ist_idx) = x;
                    ist_idx += 1;
                }
            }
            if len == 0 {
                inst.len = ist_idx;
                return inst;
            }
            loop {
                if end_idx > len {
                    break;
                }
                match str::from_utf8(&(*list)[idx..end_idx]) {
                    Ok(slicx) => {
                        for &x in slicx.as_bytes() {
                            unsafe {
                                *ist_ptr.add(ist_idx) = x;
                            }
                            ist_idx += 1;
                        }
                        for &x in upg_byt {
                            unsafe {
                                *ist_ptr.add(ist_idx) = x;
                            }
                            ist_idx += 1;
                        }
                        idx = end_idx;
                        end_idx += 1;
                    }
                    Err(_) => {
                        end_idx += 1;
                    }
                }
            }
            inst.len = ist_idx;
            return inst;
        }
        let frt_byt: u8 = (*bytes)[0];
        let mut is_match: usize = 0;
        let mut counter: usize = 0;
        unsafe {
            loop {
                if idx >= len {
                    break;
                }
                if frt_byt == *ptr.add(idx) {
                    counter = idx;
                    for &x in bytes {
                        if x == *ptr.add(counter) {
                            is_match += 1;
                        } else {
                            break;
                        }
                        counter += 1;
                    }
                    if is_match == byt_len {
                        for &x in upg_byt {
                            *ist_ptr.add(ist_idx) = x;
                            ist_idx += 1;
                        }
                        idx = counter;
                    } else {
                        *ist_ptr.add(ist_idx) = *ptr.add(idx);
                        ist_idx += 1;
                        idx += 1;
                    }
                    is_match = 0;
                } else {
                    *ist_ptr.add(ist_idx) = *ptr.add(idx);
                    ist_idx += 1;
                    idx += 1;
                }
            }
        }
        inst.len = ist_idx;
        inst
    }

    pub fn replace<'y, G: yGeneric<'y, C>, const C: usize>(self: &Self, pre: G, upg: &str) -> Self {
        match pre.iden() {
            yPattern::Slice(slice) => (*self).replace_it(slice, upg),
            yPattern::Char(ch) => (*self).replace_it(ch.encode_utf8(&mut [0, 0, 0, 0]), upg),
            yPattern::CharSlice(ch_slice) => {
                let ch_str_len: usize = ch_slice.len();
                if ch_str_len == 0 {
                    return (*self).clone();
                } else if ch_str_len == 1 {
                    return (*self).replace_it((*ch_slice)[0].encode_utf8(&mut [0, 0, 0, 0]), upg);
                } else {
                    let mut inst: Self =
                        (*self).replace_it((*ch_slice)[0].encode_utf8(&mut [0, 0, 0, 0]), upg);
                    for x in &(*ch_slice)[1..] {
                        inst = inst.replace_it(x.encode_utf8(&mut [0, 0, 0, 0]), upg);
                    }
                    inst
                }
            }
            yPattern::Closure(closure) => {
                let len: usize = (*self).len;
                if len == 0 {
                    return (*self).clone();
                }
                let mut inst: Yangon<N> = Self::with_capacity();
                let upg_byt: &[u8] = upg.as_bytes();
                let list: &[u8] = unsafe { from_raw_parts((*self).as_ptr(), len) };
                let mut srt_idx: usize = 0;
                let mut end_idx: usize = 1;
                let ist_ptr: *mut u8 = inst.list[0].as_mut_ptr();
                let mut ist_idx: usize = 0;
                loop {
                    if end_idx > len {
                        break;
                    }
                    match str::from_utf8(&(*list)[srt_idx..end_idx]) {
                        Ok(slice) => {
                            if closure(slice.chars().next().unwrap()) {
                                for &x in upg_byt {
                                    unsafe {
                                        *ist_ptr.add(ist_idx) = x;
                                        ist_idx += 1;
                                    }
                                }
                            } else {
                                unsafe {
                                    for &x in &(*list)[srt_idx..end_idx] {
                                        *ist_ptr.add(ist_idx) = x;
                                        ist_idx += 1;
                                    }
                                }
                            }
                            srt_idx = end_idx;
                            end_idx += 1;
                        }
                        Err(_) => {
                            end_idx += 1;
                        }
                    }
                }
                inst.len = ist_idx;
                inst
            }
        }
    }

    #[inline]
    pub unsafe fn list(self: &mut Self) -> &mut [MaybeUninit<u8>] {
        &mut (*self).list
    }

    pub fn trim(self: &Self) -> &str {
        let len: usize = (*self).len;
        if len == 0 {
            return "";
        }
        let list: &[u8] = unsafe {
            &*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>((
                (*self).list.as_ptr(),
                len,
            ))
        };
        let mut exist: bool = false;
        for &x in list {
            if x != 32 {
                exist = true;
            }
        }
        if !exist {
            return "";
        }
        let mut srt_idx: usize = 0;
        let mut end_idx: usize = len - 1;
        loop {
            if srt_idx >= len || (*list)[srt_idx] != 32 {
                break;
            } else {
                srt_idx += 1;
            }
        }
        loop {
            if end_idx == 0 || (*list)[end_idx] != 32 {
                break;
            } else {
                end_idx -= 1;
            }
        }
        unsafe { from_utf8_unchecked(&(*list)[srt_idx..end_idx + 1]) }
    }

    pub fn from(slice: &str) -> Self {
        let mut inst: Self = Self::with_capacity();
        let mut idx: usize = 0;
        let ptr: *mut u8 = inst.list[0].as_mut_ptr();
        for &x in slice.as_bytes() {
            unsafe {
                *ptr.add(idx) = x;
            }
            idx += 1;
        }
        inst.len = idx;
        inst
    }

    #[inline]
    pub fn new() -> Self {
        Self::with_capacity()
    }
}


#[macro_export]
#[cfg(test)]
macro_rules! yangon {
    ($($str: expr)?) => {{
        use std::mem::MaybeUninit;
        let mut inst: Yangon<10240> = Yangon::with_capacity();
        let mut idx: usize = 0;
        let list: &mut [MaybeUninit<u8>] = unsafe { inst.list() };
        $(
            for &x in $str.as_bytes() {
                unsafe {
                    *(*list)[idx].as_mut_ptr() = x;
                }
                idx += 1;
            }
        )?
        unsafe {
            inst.set_len(idx);
        }
        inst
    }};
}

#[cfg(test)]
impl<'y, const C: usize> yGeneric<'y, C> for fn(char) -> bool {
    fn iden<'b>(self: &'b Self) -> yPattern<'y, C>
    where
        'y: 'b,
    {
        yPattern::Closure(*self)
    }
}

#[cfg(test)]
impl<'y, const C: usize> yGeneric<'y, C> for &'y [char; C] {
    fn iden<'b>(self: &'b Self) -> yPattern<'y, C>
    where
        'y: 'b,
    {
        yPattern::CharSlice(*self)
    }
}

#[cfg(test)]
impl<'y, const C: usize> yGeneric<'y, C> for char {
    fn iden<'b>(self: &'b Self) -> yPattern<'y, C>
    where
        'y: 'b,
    {
        yPattern::Char(*self)
    }
}

#[cfg(test)]
impl<'y, const C: usize> yGeneric<'y, C> for &'y str {
    fn iden<'b>(self: &'b Self) -> yPattern<'y, C>
    where
        'y: 'b,
    {
        yPattern::Slice(*self)
    }
}

#[cfg(test)]
impl<const N: usize> FromIterator<char> for Yangon<N> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut inst: Self = Self::with_capacity();
        let mut idx: usize = 0;
        let mut bind: [u8; 4] = [0, 0, 0, 0];
        let ptr: *mut u8 = inst.list[0].as_mut_ptr();
        for x in iter {
            for &y in x.encode_utf8(&mut bind).as_bytes() {
                unsafe {
                    *ptr.add(idx) = y;
                }
                idx += 1;
            }
        }
        inst.len = idx;
        inst
    }
}

#[cfg(test)]
impl yTrait for &str {
    type Ygn = Yangon;
    fn to_yangon(self: &Self) -> Self::Ygn {
        let mut inst: Yangon = Yangon::with_capacity();
        let ptr: *mut u8 = inst.list[0].as_mut_ptr();
        if (*self).len() > 10240 {
            let slc_ptr: *const u8 = (*self).as_ptr();
            for x in 0..10240 {
                unsafe {
                    *ptr.add(x) = *slc_ptr.add(x);
                }
            }
            inst.len = 10240;
            return inst;
        }
        let mut idx: usize = 0;
        for &x in (*self).as_bytes() {
            unsafe {
                *ptr.add(idx) = x;
            }
            idx += 1;
        }
        inst.len = idx;
        inst
    }
}

#[cfg(test)]
impl<const N: usize> Write for Yangon<N> {
    fn write_str(self: &mut Self, slice: &str) -> FmtResult {
        let len: usize = (*self).len;
        if slice.len() + len > (*self).capacity {
            Err(FmtError)
        } else {
            let ptr: *mut u8 = (*self).list[len].as_mut_ptr();
            let mut idx: usize = 0;
            for &x in slice.as_bytes() {
                unsafe {
                    *ptr.add(idx) = x;
                    idx += 1;
                }
            }
            (*self).len += idx;
            Ok(())
        }
    }
}

#[cfg(test)]
impl<const N: usize> PartialEq<&str> for Yangon<N> {
    fn eq(self: &Self, slice: &&str) -> bool {
        unsafe {
            from_utf8_unchecked(&*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>(
                ((*self).list.as_ptr(), (*self).len),
            )) == *slice
        }
    }
}

#[cfg(test)]
impl<const N: usize> AsRef<str> for Yangon<N> {
    fn as_ref(self: &Self) -> &str {
        unsafe {
            from_utf8_unchecked(&*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>(
                ((*self).list.as_ptr(), (*self).len),
            ))
        }
    }
}

#[cfg(test)]
impl<const N: usize> Deref for Yangon<N> {
    type Target = str;
    fn deref(self: &Self) -> &Self::Target {
        unsafe {
            from_utf8_unchecked(&*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>(
                ((*self).list.as_ptr(), (*self).len),
            ))
        }
    }
}

#[cfg(test)]
impl<const N: usize> Deref for yCow<'_, Yangon<N>> {
    type Target = str;
    fn deref(self: &Self) -> &Self::Target {
        match self {
            yCow::Borrowed(slice) => slice,
            yCow::Owned(y) => unsafe {
                from_utf8_unchecked(&*transmute::<(*const MaybeUninit<u8>, usize), *const [u8]>(
                    (y.list.as_ptr(), y.len),
                ))
            },
        }
    }
}

#[cfg(test)]
impl<const N: usize> Display for Yangon<N> {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", unsafe {
            from_utf8_unchecked(transmute::<&[MaybeUninit<u8>], &[u8]>(
                &(*self).list[..(*self).len],
            ))
        })
    }
}

#[cfg(test)]
impl<const N: usize> Debug for Yangon<N> {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", unsafe {
            from_utf8_unchecked(transmute::<&[MaybeUninit<u8>], &[u8]>(
                &(*self).list[..(*self).len],
            ))
        })
    }
}

#[cfg(test)]
impl Debug for yError {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", "Error")
    }
}

#[cfg(test)]
impl Display for yError {
    fn fmt(self: &Self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Error")
    }
}




