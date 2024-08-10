use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize)]
#[rkyv(check_bytes)]
pub struct Foo {
    // Same error exists if we change `usize` to `u32` or `u64`
    pub content: usize,
}
