use super::row::NBytes;

trait Serializable {
    fn serialize(self) -> Vec<u8>;
    fn deserialize(value: &[u8]) -> Self;
}

// Internal Node
// key_size = 4 byte
// child_size = 4 byte
// key = [key_size] byte
struct InternalNode<K>
where
    K: PartialOrd + Serializable,
{
    key_size: u32,
    child: u32,
    key: K,
}

impl<K> InternalNode<K>
where
    K: PartialOrd + Serializable,
{
    const KEY_SIZE_OFFSET: usize = 0;
    const KEY_SIZE_LEN: usize = 4;
    const CHILD_SIZE_OFFSET: usize = Self::KEY_SIZE_OFFSET + Self::KEY_SIZE_LEN;
    const CHILD_SIZE_LEN: usize = 4;
    const KEY_OFFSET: usize = Self::CHILD_SIZE_OFFSET + Self::CHILD_SIZE_LEN;
}

impl<K> Serializable for InternalNode<K>
where
    K: PartialOrd + Serializable,
{
    fn serialize(self) -> Vec<u8> {
        let mut value: Vec<u8> =
            Vec::with_capacity(Self::KEY_SIZE_LEN + Self::CHILD_SIZE_LEN + self.key_size as usize);
        value.extend(self.key_size.to_le_bytes());
        value.extend(self.child.to_le_bytes());
        value.extend(self.key.serialize());
        value
    }

    fn deserialize(value: &[u8]) -> Self {
        let key_size: &[u8; 4] = &value[Self::KEY_OFFSET..(Self::KEY_OFFSET + Self::KEY_SIZE_LEN)]
            .try_into()
            .expect("Unable to deserialize Internal Node");
        let child_pointer: &[u8; 4] = &value
            [Self::CHILD_SIZE_OFFSET..(Self::CHILD_SIZE_OFFSET + Self::CHILD_SIZE_LEN)]
            .try_into()
            .expect("Unable to deserialize Internal Node");
        Self {
            key_size: u32::from_le_bytes(*key_size),
            child: u32::from_le_bytes(*child_pointer),
            key: K::deserialize(&value[Self::KEY_OFFSET..]),
        }
    }
}

// Leaf Node
// key_size = 4 byte
// value_size = 4 byte
// key = [key_size] byte
// value = [value_size] byte
struct LeafNode<K, V>
where
    K: PartialOrd + Serializable,
    V: Serializable,
{
    key_size: u32,
    value_size: u32,
    key: K,
    value: V,
}

impl<K, V> LeafNode<K, V>
where
    K: PartialOrd + Serializable,
    V: Serializable,
{
    const KEY_SIZE_OFFSET: usize = 0;
    const KEY_SIZE_LEN: usize = 4;
    const VALUE_SIZE_OFFSET: usize = Self::KEY_SIZE_OFFSET + Self::KEY_SIZE_LEN;
    const VALUE_SIZE_LEN: usize = 4;
    const KEY_OFFSET: usize = Self::VALUE_SIZE_OFFSET + Self::VALUE_SIZE_LEN;
}

impl<K, V> Serializable for LeafNode<K, V>
where
    K: PartialOrd + Serializable,
    V: Serializable,
{
    fn serialize(self) -> Vec<u8> {
        let mut value: Vec<u8> = Vec::with_capacity(
            Self::KEY_SIZE_LEN
                + Self::VALUE_SIZE_LEN
                + self.key_size as usize
                + self.value_size as usize,
        );
        value.extend(self.key_size.to_le_bytes());
        value.extend(self.value_size.to_le_bytes());
        value.extend(self.key.serialize());
        value.extend(self.value.serialize());
        value
    }

    fn deserialize(value: &[u8]) -> Self {
        let key_size: &[u8; 4] = &value
            [Self::KEY_SIZE_OFFSET..(Self::KEY_SIZE_OFFSET + Self::KEY_SIZE_LEN)]
            .try_into()
            .expect("Unable to deserialize Internal Node");
        let value_size: &[u8; 4] = &value
            [Self::VALUE_SIZE_OFFSET..(Self::VALUE_SIZE_OFFSET + Self::VALUE_SIZE_LEN)]
            .try_into()
            .expect("Unable to deserialize Internal Node");
        let key_size = u32::from_le_bytes(*key_size);
        let value_size = u32::from_le_bytes(*value_size);
        Self {
            key_size,
            key: K::deserialize(&value[Self::KEY_OFFSET..key_size as usize]),
            value_size,
            value: V::deserialize(
                &value[(Self::KEY_OFFSET + key_size as usize)..value_size as usize],
            ),
        }
    }
}

enum Node<K, V>
where
    K: PartialOrd + Serializable,
    V: Serializable,
{
    Internal(InternalNode<K>),
    Leaf(LeafNode<K, V>),
}

impl<K, V> From<Node<K, V>> for InternalNode<K>
where
    K: PartialOrd + Serializable,
    V: Serializable,
{
    fn from(value: Node<K, V>) -> Self {
        if let Node::Internal(node) = value {
            return node;
        }
        panic!("This is not a Internal Node")
    }
}

impl<K, V> From<Node<K, V>> for LeafNode<K, V>
where
    K: PartialOrd + Serializable,
    V: Serializable,
{
    fn from(value: Node<K, V>) -> Self {
        if let Node::Leaf(node) = value {
            return node;
        }
        panic!("This is not a Leaf Node")
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct PageFlags(u8);

bitflags::bitflags! {
    impl PageFlags: u8 {
        // leaf of internal node
        const IS_LEAF = 1 << 0;
        // root or other node
        const IS_ROOT = 1 << 1;
    }
}

impl PageFlags {
    pub(crate) fn is_root(&self) -> bool {
        self.contains(PageFlags::IS_ROOT)
    }
    pub(crate) fn is_leaf(&self) -> bool {
        self.contains(PageFlags::IS_LEAF)
    }
}

// Size of page = 4096 bytes
struct Page {
    pub(crate) flags: PageFlags,
    pub(crate) free_start: u16,
    pub(crate) free_end: u16,
    pub(crate) offsets: Vec<u16>,
    cells: Vec<u8>,
}

impl Page {
    const SIZE: usize = 4096; // bytes
    const FLAG_SIZE: usize = 1;
    const FREE_START_OFFSET: usize = Self::FLAG_SIZE;
    const FREE_START_SIZE: usize = 2;
    const FREE_END_OFFSET: usize = Self::FREE_START_OFFSET + Self::FREE_START_SIZE;
    const FREE_END_SIZE: usize = 2;
    const OFFSETS_OFFSET: usize = Self::FREE_END_OFFSET + Self::FREE_END_SIZE;

    pub(crate) fn new(value: &[u8]) -> Self {
        assert!(value.len() == Self::SIZE);

        // FLAG
        let flags = PageFlags::from_bits_retain(value[0]);

        // FREE START
        let free_start: &[u8; 2] = &value
            [Self::FREE_START_OFFSET..(Self::FREE_START_OFFSET + Self::FREE_START_SIZE)]
            .try_into()
            .expect("Unable to deserialize free_start value");
        let free_start = u16::from_le_bytes(*free_start);

        // FREE END
        let free_end: &[u8; 2] = &value
            [Self::FREE_END_OFFSET..(Self::FREE_END_OFFSET + Self::FREE_END_SIZE)]
            .try_into()
            .expect("Unable to deserialize free_end value");
        let free_end = u16::from_le_bytes(*free_end);

        // OFFSETS
        let offsets_bytes = &value[Self::OFFSETS_OFFSET..free_start as usize];
        assert!(
            offsets_bytes.len() % 2 == 0,
            "Offsets should be multiples of 2"
        );
        let mut offsets = Vec::with_capacity(offsets_bytes.len() / 2);
        let mut i = 0;
        while i < offsets_bytes.len() {
            let offset: &[u8; 2] = &offsets_bytes[i..i + 2]
                .try_into()
                .expect("Unable to deserialize offsets of a page");
            offsets.push(u16::from_le_bytes(*offset));
            i += 2;
        }

        // CELLS
        let cells = if value.len() == free_end.into() {
            Vec::new()
        } else {
            value[(free_end + 1) as usize..].to_vec()
        };

        Self {
            flags,
            free_start,
            free_end,
            offsets,
            cells,
        }
    }

    pub(crate) fn get(&self, i: usize) -> Option<&[u8]> {
        let offset = self.offsets.get(i)?.to_owned().into();
        let end_offset = if self.offsets.len() - 1 == i {
            self.offsets.len()
        } else {
            self.offsets
                .get(i + 1)
                .expect("This is not possible?")
                .to_owned()
                .into()
        };
        Some(&self.cells[offset..end_offset])
    }
}

//impl<K, V> Serializable for Page<K, V>
//where
//    K: PartialOrd + Serializable,
//    V: Serializable,
//{
//    fn serialize(self) -> Vec<u8> {
//        let mut value: Vec<u8> = Vec::with_capacity(Self::SIZE);
//        value.push(self.flags.0);
//        for offset in self.offsets {
//            value.extend(offset.to_le_bytes());
//        }
//        for _ in 0..self.free_end - self.free_start {
//            value.push(0);
//        }
//        for node in self.cells {
//            value.extend(node.serialize());
//        }
//        value
//    }
//
//    fn deserialize(value: &[u8]) -> Self {
//        let flag = PageFlags::from_bits_retain(value[0]);
//        let free_start: [u8; 2] = (&value[1..3]).get_n_bytes();
//        let free_start = u16::from_le_bytes(free_start);
//        let free_end: [u8; 2] = (&value[3..5]).get_n_bytes();
//        let free_end = u16::from_le_bytes(free_end);
//        todo!()
//    }
//}
